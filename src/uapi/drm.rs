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

use crate::{IocResult, EResult, Errno};
use crate::tegra_drm::*;
use super::{ioctl, Syncpoint, Channel, Mapping, Gem};

use std::os::unix::io::AsRawFd;

pub struct Drm {
    pub fd: std::fs::File,
}

macro_rules! define_raw {
    ($($name:ident : $ioctl:ident -> $ty:ty),* $(,)?) => {
        $(
        #[allow(unused)]
        pub fn $name(&self, mut args: $ty) -> IocResult<$ty> {
            unsafe {
                let err = ioctl(self.fd.as_raw_fd(), $ioctl, &mut args);
                if err != 0 {
                    Err(Errno::get())?;
                }

                Ok(args)
            }
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
        allocate_syncpoint_raw: DRM_IOCTL_TEGRA_SYNCPOINT_ALLOCATE -> drm_tegra_syncpoint_allocate,
        free_syncpoint_raw: DRM_IOCTL_TEGRA_SYNCPOINT_FREE -> drm_tegra_syncpoint_free,
        wait_syncpoint_raw: DRM_IOCTL_TEGRA_SYNCPOINT_WAIT -> drm_tegra_syncpoint_wait,
        increment_syncpoint_raw: DRM_IOCTL_TEGRA_SYNCPT_INCR -> drm_tegra_syncpt_incr,
        open_channel_raw: DRM_IOCTL_TEGRA_CHANNEL_OPEN -> drm_tegra_channel_open,
        close_channel_raw: DRM_IOCTL_TEGRA_CHANNEL_CLOSE -> drm_tegra_channel_close,
        channel_map_raw: DRM_IOCTL_TEGRA_CHANNEL_MAP -> drm_tegra_channel_map,
        channel_unmap_raw: DRM_IOCTL_TEGRA_CHANNEL_UNMAP -> drm_tegra_channel_unmap,
        channel_submit_raw: DRM_IOCTL_TEGRA_CHANNEL_SUBMIT -> drm_tegra_channel_submit,
        gem_create_raw: DRM_IOCTL_TEGRA_GEM_CREATE -> drm_tegra_gem_create,
        gem_close_raw: DRM_IOCTL_GEM_CLOSE -> drm_gem_close,
        gem_mmap_raw: DRM_IOCTL_TEGRA_GEM_MMAP -> drm_tegra_gem_mmap,
        prime_handle_to_fd: DRM_IOCTL_PRIME_HANDLE_TO_FD -> drm_prime_handle,
    );

    pub fn allocate_syncpoint(&self) -> IocResult<Syncpoint> {
        unsafe {
            let mut args: drm_tegra_syncpoint_allocate = std::mem::zeroed();

            args = self.allocate_syncpoint_raw(args)?;

            Ok(Syncpoint { drm: self, id: args.id })
        }
    }

    pub fn wait_syncpoint(
        &self,
        id: u32,
        threshold: u32,
        timeout: std::time::Duration,
    ) -> IocResult<u32> {
        unsafe {
            let mut args: drm_tegra_syncpoint_wait = std::mem::zeroed();
            let mut time = std::mem::zeroed();

            libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut time);

            let timeout = time.tv_sec * 1_000_000_000 + time.tv_nsec + timeout.as_nanos() as i64;

            args.timeout_ns = timeout;
            args.id = id;
            args.threshold = threshold;

            args = self.wait_syncpoint_raw(args)?;

            Ok(args.value)
        }
    }

    pub fn increment_syncpoint(&self, id: u32) -> IocResult<()> {
        unsafe {
            let mut args: drm_tegra_syncpt_incr = std::mem::zeroed();

            args.id = id;

            self.increment_syncpoint_raw(args)?;

            Ok(())
        }
    }

    pub fn read_syncpoint(
        &self,
        id: u32,
    ) -> IocResult<u32> {
        self.read_syncpoint_with_threshold(id, 0)
    }

    pub fn read_syncpoint_with_threshold(
        &self,
        id: u32,
        threshold: u32,
    ) -> IocResult<u32> {
        unsafe {
            let mut args: drm_tegra_syncpoint_wait = std::mem::zeroed();

            args.timeout_ns = 0;
            args.id = id;
            args.threshold = threshold;

            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_TEGRA_SYNCPOINT_WAIT, &mut args);
            if err != 0 {
                let errno = Errno::get();
                if errno.0 != libc::EAGAIN {
                    return Err(errno);
                }
            }

            Ok(args.value)
        }
    }

    pub fn open_channel(&self, class: u32) -> IocResult<Channel> {
        unsafe {
            let mut args: drm_tegra_channel_open = std::mem::zeroed();

            args.host1x_class = class;
            args.flags = 0;

            args = self.open_channel_raw(args)?;

            Ok(Channel {
                drm: self,
                ctx: args.context,
                hw_version: args.version,
            })
        }
    }

    pub fn channel_map(
        &self,
        channel: &Channel,
        gem: &Gem,
        offset: u64,
        _length: u64,
        rw: bool,
    ) -> IocResult<Mapping> {
        assert_eq!(offset, 0);

        unsafe {
            let mut args: drm_tegra_channel_map = std::mem::zeroed();

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
    }

    pub fn gem_create(&self, size: u64) -> IocResult<Gem> {
        unsafe {
            let mut args: drm_tegra_gem_create = std::mem::zeroed();

            args.size = size;

            args = self.gem_create_raw(args)?;

            Ok(Gem {
                drm: self,
                handle: args.handle,
            })
        }
    }
}