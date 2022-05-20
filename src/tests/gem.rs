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

use anyhow::Result;

use crate::{tegra_drm, Main, EINVAL};

pub fn test_gem_mmap_invalid_ioctl(main: &Main) -> Result<()> {
    let zero: tegra_drm::drm_tegra_gem_mmap = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.pad = 0xffff_ffff;
        check_err!(
            main.drm.gem_mmap_raw(args),
            EINVAL,
            "expected EINVAL, got {left:?}"
        );
    }

    {
        let mut args = zero;
        args.handle = 0xdeadbeef;
        check_err!(
            main.drm.gem_mmap_raw(args),
            EINVAL,
            "expected EINVAL, got {left:?}"
        );
    }

    Ok(())
}

pub fn test_gem_mmap(main: &Main) -> Result<()> {
    let gem = main.drm.gem_create(0x1000)?;

    {
        let mut mmap = gem.map(0x1000)?;
        mmap[16] = 0xda;

        check_eq!(mmap[16], 0xda, "mmap write/read check failed, got {left}");
    }

    {
        let mmap = gem.map(0x1000)?;

        check_eq!(
            mmap[16],
            0xda,
            "mmap write/read check on remap failed, got {left}"
        );
    }

    Ok(())
}
