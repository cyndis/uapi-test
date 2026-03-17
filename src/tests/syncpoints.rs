/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2020 NVIDIA Corporation
 *                    uapi-test contributors
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use crate::{Main, EBUSY, EINVAL};
use anyhow::{Context, Result};

uapi_test!("syncpoint.read", read_syncpoints);
pub fn read_syncpoints(main: &Main) -> Result<()> {
    for i in 0..main.soc.num_syncpoints() {
        let value = main.drm.read_syncpoint(i);

        check_ok!(value, "could not read syncpoint {} value: {err}", i);
    }

    check_eq!(
        main.drm.read_syncpoint(main.soc.num_syncpoints() + 1),
        Err(EINVAL),
        "reading invalid syncpoint returned {left:?}"
    );

    Ok(())
}

uapi_test!("syncpoint.incr_and_read", incr_and_read_syncpoint);
pub fn incr_and_read_syncpoint(main: &Main) -> Result<()> {
    let sp = main.drm.allocate_syncpoint()?;
    let value = main.drm.read_syncpoint(sp.id())?;
    main.drm.increment_syncpoint(sp.id())?;
    check!(
        main.drm.read_syncpoint(sp.id())? == value.wrapping_add(1),
        "read value did not increment (standard)"
    );

    let sp = main.drm.allocate_syncpoint()?;
    let value = main
        .drm
        .read_syncpoint_with_threshold(sp.id(), 0x7fff_ffff)?;
    main.drm.increment_syncpoint(sp.id())?;
    check!(
        main.drm
            .read_syncpoint_with_threshold(sp.id(), 0x7fff_ffff)?
            == value.wrapping_add(1),
        "read value did not increment (standard)"
    );

    Ok(())
}

uapi_test!("syncpoint.allocate", allocate_syncpoint);
pub fn allocate_syncpoint(main: &Main) -> Result<()> {
    let mut sps = vec![];
    loop {
        let sp = main.drm.allocate_syncpoint();

        match sp {
            Ok(sp) => {
                sps.push(sp);
            }
            Err(EBUSY) => {
                break;
            }
            Err(e) => {
                anyhow::bail!("could not allocate syncpoint: {}", e);
            }
        }
    }

    for sp in &sps {
        let id = sp.id();
        check!(
            id < main.soc.num_syncpoints(),
            "syncpoint ID {} exceeds hardware maximum",
            id
        );
    }

    let last_id = sps.pop().unwrap().id();

    let sp = main.drm.allocate_syncpoint()?;
    let id = sp.id();
    check!(
        id == last_id,
        "freeing and reallocating resulted in different syncpoint"
    );

    Ok(())
}

uapi_test!("syncpoint.wait_all", wait_all_syncpoints);
pub fn wait_all_syncpoints(main: &Main) -> Result<()> {
    let mut sps = vec![];
    loop {
        let sp = main.drm.allocate_syncpoint();

        match sp {
            Ok(sp) => {
                sps.push(sp);
            }
            Err(EBUSY) => {
                break;
            }
            Err(e) => {
                anyhow::bail!("could not allocate syncpoint: {}", e);
            }
        }
    }

    for sp in &sps {
        let id = sp.id();
        let value = main
            .drm
            .read_syncpoint(sp.id())
            .context("failed to read syncpoint value")?;

        std::thread::scope(|s| {
            let thread = s.spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(10));
                main.drm.increment_syncpoint(id).unwrap();
            });

            main.drm
                .wait_syncpoint(
                    sp.id(),
                    value.wrapping_add(1),
                    std::time::Duration::from_millis(50),
                )
                .context("wait timed out")?;
            thread
                .join()
                .map_err(|_| anyhow::anyhow!("thread failed"))?;

            let r: Result<_> = Ok(());
            r
        })?;
    }

    Ok(())
}

uapi_test!("syncpoint.wait_multiple", wait_syncpoint_multiple);
pub fn wait_syncpoint_multiple(main: &Main) -> Result<()> {
    let sp = main
        .drm
        .allocate_syncpoint()
        .context("failed to allocate syncpoint")?;
    let value = main
        .drm
        .read_syncpoint(sp.id())
        .context("failed to read syncpoint value")?;

    std::thread::scope(|s| {
        let handles = (0..100)
            .map(|_| {
                s.spawn(|| {
                    main.drm.wait_syncpoint(
                        sp.id(),
                        value.wrapping_add(1),
                        std::time::Duration::from_millis(1000),
                    )
                })
            })
            .collect::<Vec<_>>();

        std::thread::sleep(std::time::Duration::from_millis(500));

        main.drm.increment_syncpoint(sp.id())?;

        for handle in handles {
            handle.join().unwrap()?;
        }

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}

uapi_test!("syncpoint.timeout_correctness", timeout_correctness);
pub fn timeout_correctness(main: &Main) -> Result<()> {
    let sp = main
        .drm
        .allocate_syncpoint()
        .context("failed to allocate syncpoint")?;
    let value = main
        .drm
        .read_syncpoint(sp.id())
        .context("failed to read syncpoint value")?;

    let _ = std::thread::scope(|s| {
        for iter in 1..1000 {
            s.spawn(|| {
                main.drm.increment_syncpoint(sp.id()).unwrap();
            });

            let a = std::time::Instant::now();
            let result = main.drm.wait_syncpoint(
                sp.id(),
                value.wrapping_add(iter),
                std::time::Duration::from_millis(1),
            );
            let b = std::time::Instant::now();

            let delta = b.duration_since(a);
            check!(
                result.is_ok() || delta.as_micros() >= 1000,
                "returned error even though wait completed within timeout"
            );
        }

        Ok(())
    });

    Ok(())
}
