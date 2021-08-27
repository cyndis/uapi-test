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

use crate::{Main, EINVAL, EBUSY};
use anyhow::Result;

pub fn test_read_syncpoints(main: &Main) -> Result<()> {
    for i in 0..main.soc.num_syncpoints() {
        let value = main.drm.read_syncpoint(i);

        check_ok!(value, "could not read syncpoint {} value: {err}", i);
    }

    check_eq!(
        main.drm.read_syncpoint(main.soc.num_syncpoints() + 1), Err(EINVAL),
        "reading invalid syncpoint returned {left:?}"
    );

    Ok(())
}

pub fn test_incr_and_read_syncpoint(main: &Main) -> Result<()> {
    let sp = main.drm.allocate_syncpoint()?;
    let value = main.drm.read_syncpoint(sp.id())?;
    main.drm.increment_syncpoint(sp.id())?;
    check!(main.drm.read_syncpoint(sp.id())? == value.wrapping_add(1), "read value did not increment (standard)");

    let sp = main.drm.allocate_syncpoint()?;
    let value = main.drm.read_syncpoint_with_threshold(sp.id(), 0x7fff_ffff)?;
    main.drm.increment_syncpoint(sp.id())?;
    check!(main.drm.read_syncpoint_with_threshold(sp.id(), 0x7fff_ffff)? == value.wrapping_add(1), "read value did not increment (standard)");

    Ok(())
}

pub fn test_allocate_syncpoint(main: &Main) -> Result<()> {
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
        check!(id < main.soc.num_syncpoints(), "syncpoint ID {} exceeds hardware maximum", id);
    }

    let last_id = sps.pop().unwrap().id();

    let sp = main.drm.allocate_syncpoint()?;
    let id = sp.id();
    check!(id == last_id, "freeing and reallocating resulted in different syncpoint");

    Ok(())
}
