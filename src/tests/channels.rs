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

use crate::{tegra_drm, Main, EINVAL, ENODEV};

pub fn test_open_channel_invalid_ioctl(main: &Main) -> Result<()> {
    let zero: tegra_drm::drm_tegra_channel_open = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.flags = 0xffff_ffff;
        check_err!(
            main.drm.open_channel_raw(args),
            EINVAL,
            "expected EINVAL, got {left:?}"
        );
    }

    {
        let mut args = zero;
        args.host1x_class = 0x0;
        check_err!(
            main.drm.open_channel_raw(args),
            ENODEV,
            "expected ENODEV, got {left:?}"
        );
    }

    {
        let mut args = zero;
        args.host1x_class = 0xff;
        check_err!(
            main.drm.open_channel_raw(args),
            ENODEV,
            "expected ENODEV, got {left:?}"
        );
    }

    Ok(())
}

pub fn test_open_close_channel(main: &Main) -> Result<()> {
    let channel = main.drm.open_channel(main.engine_class)?;
    let zero: tegra_drm::drm_tegra_channel_close = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.context = channel.context();
        main.drm.close_channel_raw(args)?;
        check_err!(
            main.drm.close_channel_raw(args),
            EINVAL,
            "expected EINVAL, got {left:?}"
        );
    }

    Ok(())
}

pub fn test_engine_metadata(main: &Main) -> Result<()> {
    let channel = main.drm.open_channel(main.engine_class)?;

    check_eq!(
        channel.hw_version(),
        main.soc.chip_id(),
        "hw_version doesn't match chip id: {left}"
    );

    Ok(())
}

pub fn test_channel_map_invalid_ioctl(main: &Main) -> Result<()> {
    let channel = main.drm.open_channel(main.engine_class)?;

    let zero: tegra_drm::drm_tegra_channel_map = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.context = 0;
        check_err!(
            main.drm.channel_map_raw(args),
            EINVAL,
            "expected EINVAL, got {left:?}"
        );
    }

    {
        let mut args = zero;
        args.context = channel.context();
        args.handle = 0xdeadbeef;
        check_err!(
            main.drm.channel_map_raw(args),
            EINVAL,
            "expected EINVAL, got {left:?}"
        );
    }

    let gem = main.drm.gem_create(0x1000)?;

    {
        let mut args = zero;
        args.context = channel.context();
        args.flags = 0xffffffff;
        args.handle = gem.handle();
        check_err!(
            main.drm.channel_map_raw(args),
            EINVAL,
            "expected EINVAL, got {left:?}"
        );
    }

    Ok(())
}

pub fn test_channel_map_unmap(main: &Main) -> Result<()> {
    let channel = main.drm.open_channel(main.engine_class)?;
    let gem = main.drm.gem_create(0x1000)?;

    let m = main.drm.channel_map(&channel, &gem, 0x0, 0x1000, false)?;

    {
        let mut args: tegra_drm::drm_tegra_channel_unmap = unsafe { std::mem::zeroed() };
        args.context = channel.context();
        args.mapping = m.id();

        main.drm.channel_unmap_raw(args)?;
        check_err!(
            main.drm.channel_unmap_raw(args),
            EINVAL,
            "expected EINVAL when double unmapping, got {left:?}"
        );
        std::mem::forget(m);
    }

    Ok(())
}

pub fn test_channel_map_gem_close(main: &Main) -> Result<()> {
    let channel = main.drm.open_channel(main.engine_class)?;
    let gem = main.drm.gem_create(0x1000)?;
    let m = main.drm.channel_map(&channel, &gem, 0x0, 0x1000, false)?;

    drop(gem);

    m.drop()?;

    Ok(())
}
