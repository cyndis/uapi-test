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

use crate::{host1x, Main, EResult, EINVAL, EINTR, EBUSY};

pub fn test_read_syncpoints(main: &Main) -> EResult<()> {
    for i in 0..main.soc.num_syncpoints() {
        let value = main.host1x.read_syncpoint(i);

        check!(value.is_ok());
    }

    check_eq!(main.host1x.read_syncpoint(main.soc.num_syncpoints() + 1), Err(EINVAL));

    Ok(())
}

pub fn test_allocate_syncpoint_invalid_ioctl(main: &Main) -> EResult<()> {
    let zero: host1x::host1x_allocate_syncpoint = unsafe { std::mem::zeroed() };

    for i in 0..3 {
        let mut args = zero;
        args.reserved[i] = 1;
        check_eq!(main.host1x.allocate_syncpoint_raw(args), Err(EINVAL));
    }

    Ok(())
}

pub fn test_allocate_syncpoint(main: &Main) -> EResult<()> {
    let mut sps = vec![];
    loop {
        let sp = main.host1x.allocate_syncpoint();

        match sp {
            Ok(sp) => {
                sps.push(sp);
            }
            Err(EBUSY) => {
                break;
            }
            Err(_) => {
                check!(false);
            }
        }
    }

    for sp in &sps {
        let id = check_unwrap!(sp.id());
        check!(id < main.soc.num_syncpoints());
    }

    let last_id = check_unwrap!(sps.pop().unwrap().id());

    let sp = check_unwrap!(main.host1x.allocate_syncpoint());
    let id = check_unwrap!(sp.id());
    check!(id == last_id);

    Ok(())
}

pub fn test_increment_syncpoint(main: &Main) -> EResult<()> {
    let sp = check_unwrap!(main.host1x.allocate_syncpoint());
    let id = check_unwrap!(sp.id());

    let pre_val = check_unwrap!(main.host1x.read_syncpoint(id));

    check_unwrap!(sp.increment(1));

    let val = check_unwrap!(main.host1x.read_syncpoint(id));
    check_eq!(val, pre_val.wrapping_add(1));

    check_unwrap!(sp.increment(999));
    let val = check_unwrap!(main.host1x.read_syncpoint(id));
    check_eq!(val, pre_val.wrapping_add(1000));

    Ok(())
}

pub fn test_increment_syncpoint_intr(main: &Main) -> EResult<()> {
    let sp = check_unwrap!(main.host1x.allocate_syncpoint());

    unsafe {
        let mut newact: libc::sigaction = std::mem::zeroed();

        extern "C" fn signal_noop() {
        }

        newact.sa_sigaction = signal_noop as usize;
        newact.sa_flags = libc::SA_SIGINFO | libc::SA_RESETHAND;

        libc::sigaction(libc::SIGALRM, &newact as *const _, std::ptr::null_mut());
    }

    unsafe { libc::alarm(1); }

    let val = sp.increment(0xffff_ffff);

    check_eq!(val, Err(EINTR));

    Ok(())
}

pub fn test_create_fence_invalid_ioctl(main: &Main) -> EResult<()> {
    let zero: host1x::host1x_create_fence = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.reserved[0] = 1;
        check_eq!(main.host1x.create_fence_raw(args), Err(EINVAL));
    }

    {
        let mut args = zero;
        args.id = main.soc.num_syncpoints();
        check_eq!(main.host1x.create_fence_raw(args), Err(EINVAL));
    }

    Ok(())
}

pub fn test_create_fence(main: &Main) -> EResult<()> {
    let sp = check_unwrap!(main.host1x.allocate_syncpoint());

    /* Increment once to ensure syncpoint shows up in debugfs */
    check_unwrap!(sp.increment(1));

    let id = check_unwrap!(sp.id());
    let value = check_unwrap!(main.host1x.read_syncpoint(id));

    /* Fence slightly in the future */
    let fence1 = check_unwrap!(main.host1x.create_fence(id, value.wrapping_add(1)));

    /* Fence very far in the future */
    let fence2 = check_unwrap!(main.host1x.create_fence(id, value.wrapping_add(0x8000_0000)));

    check!(fence1.wait(10).is_err());
    check!(fence2.wait(10).is_err());

    /* Fence for current value after setting up fence very far in the future */
    check!(check_unwrap!(main.host1x.create_fence(id, value)).wait(10).is_ok());

    /* Fence slightly in the past */
    check!(check_unwrap!(main.host1x.create_fence(id, value.wrapping_sub(1))).wait(10).is_ok());

    /* Fence very far in the past */
    check!(check_unwrap!(main.host1x.create_fence(id, value.wrapping_add(0x8000_0001))).wait(10).is_ok());

    /* Check that SW timeout triggers */
    check!(fence2.wait(35000).is_ok());
    check!(fence1.wait(2000).is_ok());
    check_eq!(check_unwrap!(fence1.status()), -libc::ETIMEDOUT);
    check_eq!(check_unwrap!(fence2.status()), -libc::ETIMEDOUT);

    {
        use std::io::BufRead;

        let fp = std::io::BufReader::new(
            std::fs::File::open("/sys/kernel/debug/tegra-host1x/status")?);

        let pattern = format!("id {} ", id);
        let mut found = false;
        for line in fp.lines() {
            let line = line?;

            if line.starts_with(&pattern) {
                check!(line.contains("(0 waiters)"));
                found = true;
                break;
            }
        }

        check!(found);
    }

    Ok(())
}

pub fn test_create_fence_and_signal(main: &Main) -> EResult<()> {
    let sp = check_unwrap!(main.host1x.allocate_syncpoint());
    let id = check_unwrap!(sp.id());
    let value = check_unwrap!(main.host1x.read_syncpoint(id));

    check!(check_unwrap!(main.host1x.create_fence(id, value.wrapping_add(1))).wait(10).is_err());

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        sp.increment(1).unwrap();
    });

    check!(check_unwrap!(main.host1x.create_fence(id, value.wrapping_add(1))).wait(1000).is_ok());

    Ok(())
}

pub fn test_extract_fence(main: &Main) -> EResult<()> {
    let fence = main.host1x.create_fence(50, 51)?;

    let pts = check_unwrap!(main.host1x.extract_fence(fence.fd()));
    check_eq!(pts.len(), 1);
    check_eq!(pts[0].id, 50);
    check_eq!(pts[0].threshold, 51);

    Ok(())
}
