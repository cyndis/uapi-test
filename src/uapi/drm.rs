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

use super::{Channel, Gem, Mapping, Syncpoint};
use crate::tegra_drm::*;
use crate::{EResult, Errno, IocResult};

use bytemuck::Zeroable;
use std::os::unix::io::AsRawFd;

pub struct Drm {
    pub fd: std::fs::File,
}

macro_rules! define_raw {
    ($($name:ident : $ioc_fn:ident, $ty:ty);* $(;)?) => {
        $(
        #[allow(unused)]
        pub fn $name(&self, mut args: $ty) -> IocResult<$ty> {
            unsafe { $ioc_fn(self.fd.as_raw_fd(), &mut args) }?;
            Ok(args)
        }
        )*
    }
}

impl Drm {
    pub fn open() -> EResult<Drm> {
        let fd = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/dri/card0")?;

        Ok(Drm { fd })
    }

    define_raw!(
        allocate_syncpoint_raw: ioctl_syncpoint_allocate, drm_tegra_syncpoint_allocate;
        free_syncpoint_raw: ioctl_syncpoint_free, drm_tegra_syncpoint_free;
        wait_syncpoint_raw: ioctl_syncpoint_wait, drm_tegra_syncpoint_wait;
        increment_syncpoint_raw: ioctl_syncpt_incr, drm_tegra_syncpt_incr;
        open_channel_raw: ioctl_channel_open, drm_tegra_channel_open;
        close_channel_raw: ioctl_channel_close, drm_tegra_channel_close;
        channel_map_raw: ioctl_channel_map, drm_tegra_channel_map;
        channel_unmap_raw: ioctl_channel_unmap, drm_tegra_channel_unmap;
        channel_submit_raw: ioctl_channel_submit, drm_tegra_channel_submit;
        gem_create_raw: ioctl_gem_create, drm_tegra_gem_create;
        gem_mmap_raw: ioctl_gem_mmap, drm_tegra_gem_mmap;
        prime_handle_to_fd: ioctl_prime_handle_to_fd, drm_prime_handle;
    );

    // ioctl_gem_close takes *const (IOW not IOWR), so handle it separately.
    #[allow(unused)]
    pub fn gem_close_raw(&self, args: drm_gem_close) -> IocResult<drm_gem_close> {
        unsafe { ioctl_gem_close(self.fd.as_raw_fd(), &args) }?;
        Ok(args)
    }

    pub fn allocate_syncpoint(&self) -> IocResult<Syncpoint<'_>> {
        let args = self.allocate_syncpoint_raw(drm_tegra_syncpoint_allocate::zeroed())?;

        Ok(Syncpoint {
            drm: self,
            id: args.id,
        })
    }

    pub fn wait_syncpoint(
        &self,
        id: u32,
        threshold: u32,
        timeout: std::time::Duration,
    ) -> IocResult<u32> {
        let mut time: libc::timespec = unsafe { std::mem::zeroed() };
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut time) };

        let timeout = time.tv_sec as i64 * 1_000_000_000 + time.tv_nsec as i64 + timeout.as_nanos() as i64;

        let mut args = drm_tegra_syncpoint_wait::zeroed();
        args.timeout_ns = timeout;
        args.id = id;
        args.threshold = threshold;

        args = self.wait_syncpoint_raw(args)?;
        Ok(args.value)
    }

    pub fn increment_syncpoint(&self, id: u32) -> IocResult<()> {
        let mut args = drm_tegra_syncpt_incr::zeroed();
        args.id = id;
        self.increment_syncpoint_raw(args)?;
        Ok(())
    }

    pub fn read_syncpoint(&self, id: u32) -> IocResult<u32> {
        self.read_syncpoint_with_threshold(id, 0)
    }

    pub fn read_syncpoint_with_threshold(&self, id: u32, threshold: u32) -> IocResult<u32> {
        let mut args = drm_tegra_syncpoint_wait::zeroed();
        args.timeout_ns = 0;
        args.id = id;
        args.threshold = threshold;

        match unsafe { ioctl_syncpoint_wait(self.fd.as_raw_fd(), &mut args) } {
            Ok(_) => Ok(args.value),
            Err(Errno::EAGAIN) => Ok(args.value),
            Err(e) => Err(e),
        }
    }

    pub fn open_channel(&self, class: u32) -> IocResult<Channel<'_>> {
        let mut args = drm_tegra_channel_open::zeroed();
        args.host1x_class = class;

        args = self.open_channel_raw(args)?;

        Ok(Channel {
            drm: self,
            ctx: args.context,
            hw_version: args.version,
        })
    }

    pub fn channel_map(
        &self,
        channel: &Channel,
        gem: &Gem,
        offset: u64,
        _length: u64,
        rw: bool,
    ) -> IocResult<Mapping<'_>> {
        assert_eq!(offset, 0);

        let mut args = drm_tegra_channel_map::zeroed();
        args.context = channel.context();
        args.handle = gem.handle();
        args.flags = DRM_TEGRA_CHANNEL_MAP_READ;
        if rw {
            args.flags |= DRM_TEGRA_CHANNEL_MAP_WRITE;
        }

        args = self.channel_map_raw(args)?;

        Ok(Mapping {
            drm: self,
            id: args.mapping,
            context: channel.context(),
            iova: 0,
        })
    }

    pub fn gem_create(&self, size: u64) -> IocResult<Gem<'_>> {
        let mut args = drm_tegra_gem_create::zeroed();
        args.size = size;

        args = self.gem_create_raw(args)?;

        Ok(Gem {
            drm: self,
            handle: args.handle,
        })
    }
}
