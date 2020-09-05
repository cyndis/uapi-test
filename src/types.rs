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

use std::os::unix::io::AsRawFd;

use crate::host1x::*;
use crate::sync_file::*;
use crate::tegra_drm::*;
use crate::{EResult, IocResult, Errno};

pub struct Host1x {
    fd: std::fs::File,
}

#[cfg(target_env = "musl")]
unsafe fn ioctl<T>(fd: i32, ioc: u64, data: T) -> i32 {
    /*
     * We want to pass an unsigned int in through an int parameter,
     * so need to transmute for a bit-exact copy..
     */
    libc::ioctl(fd, std::mem::transmute(ioc as u32), data)
}
#[cfg(not(target_env = "musl"))]
unsafe fn ioctl<T>(fd: i32, ioc: u64, data: T) -> i32 {
    libc::ioctl(fd, ioc, data)
}

impl Host1x {
    pub fn open() -> EResult<Host1x> {
        let fd = std::fs::File::open("/dev/host1x")?;

        Ok(Host1x { fd })
    }

    pub fn read_syncpoint(&self, id: u32) -> IocResult<u32> {
        unsafe {
            let mut args: host1x_read_syncpoint = std::mem::zeroed();
            args.id = id;

            let err = ioctl(self.fd.as_raw_fd(), HOST1X_IOCTL_READ_SYNCPOINT, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(args.value)
        }
    }

    pub fn allocate_syncpoint_raw(
        &self,
        mut args: host1x_allocate_syncpoint,
    ) -> IocResult<Syncpoint> {
        unsafe {
            let err = ioctl(
                self.fd.as_raw_fd(),
                HOST1X_IOCTL_ALLOCATE_SYNCPOINT,
                &mut args,
            );
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(Syncpoint(args.fd))
        }
    }

    pub fn allocate_syncpoint(&self) -> IocResult<Syncpoint> {
        unsafe {
            let args: host1x_allocate_syncpoint = std::mem::zeroed();

            self.allocate_syncpoint_raw(args)
        }
    }

    pub fn create_fence_raw(&self, mut args: host1x_create_fence) -> IocResult<Fence> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), HOST1X_IOCTL_CREATE_FENCE, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(Fence(args.fence_fd))
        }
    }

    pub fn create_fence(&self, id: u32, threshold: u32) -> IocResult<Fence> {
        unsafe {
            let mut args: host1x_create_fence = std::mem::zeroed();

            args.id = id;
            args.threshold = threshold;

            self.create_fence_raw(args)
        }
    }

    pub fn extract_fence_raw(&self, args: &mut host1x_fence_extract) -> IocResult<()> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), HOST1X_IOCTL_FENCE_EXTRACT, args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(())
        }
    }

    pub fn extract_fence(&self, fd: i32) -> IocResult<Vec<host1x_fence_extract_fence>> {
        unsafe {
            let mut fences: Vec<host1x_fence_extract_fence>;
            let mut args: host1x_fence_extract = std::mem::zeroed();

            args.fence_fd = fd;
            args.num_fences = 0;

            self.extract_fence_raw(&mut args)?;

            fences = vec![std::mem::zeroed(); args.num_fences as usize];
            args.fences_ptr = fences.as_mut_ptr() as u64;

            self.extract_fence_raw(&mut args)?;

            Ok(fences)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Syncpoint(i32);
impl Drop for Syncpoint {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

impl Syncpoint {
    pub fn fd(&self) -> i32 {
        self.0
    }

    pub fn id(&self) -> IocResult<u32> {
        unsafe {
            let mut args: host1x_syncpoint_info = std::mem::zeroed();

            let err = ioctl(self.0, HOST1X_IOCTL_SYNCPOINT_INFO, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(args.id)
        }
    }

    pub fn increment(&self, count: u32) -> IocResult<()> {
        unsafe {
            let mut args: host1x_syncpoint_increment = std::mem::zeroed();

            args.count = count;

            let err = ioctl(self.0, HOST1X_IOCTL_SYNCPOINT_INCREMENT, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Fence(i32);
impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.0);
        }
    }
}

impl Fence {
    pub fn from_fd(fd: i32) -> Fence {
        Fence(fd)
    }

    pub fn fd(&self) -> i32 {
        self.0
    }

    pub fn wait(&self, timeout_ms: i32) -> EResult<()> {
        unsafe {
            let mut pfd = libc::pollfd {
                fd: self.0,
                events: libc::POLLIN,
                revents: 0,
            };

            let err = libc::poll(&mut pfd as *mut _, 1, timeout_ms);

            if err == 1 {
                Ok(())
            } else {
                Err("Wait timed out".into())
            }
        }
    }

    pub fn status(&self) -> IocResult<i32> {
        unsafe {
            let mut args: sync_file_info = std::mem::zeroed();

            let err = ioctl(self.0, SYNC_IOC_FILE_INFO, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(args.status)
        }
    }
}

pub struct Drm {
    fd: std::fs::File,
}

impl Drm {
    pub fn open() -> EResult<Drm> {
        let fd = std::fs::OpenOptions::new().read(true).write(true).open("/dev/dri/card0")?;

        Ok(Drm { fd })
    }

    pub fn open_channel_raw(
        &self,
        mut args: drm_tegra_channel_open,
    ) -> IocResult<drm_tegra_channel_open> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_TEGRA_CHANNEL_OPEN, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(args)
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
                ctx: args.channel_ctx,
                hw_version: args.hardware_version
            })
        }
    }

    pub fn close_channel_raw(&self, mut args: drm_tegra_channel_close) -> IocResult<()> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_TEGRA_CHANNEL_CLOSE, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(())
        }
    }

    pub fn channel_map_raw(&self, mut args: drm_tegra_channel_map) -> IocResult<drm_tegra_channel_map> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_TEGRA_CHANNEL_MAP, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(args)
        }
    }

    pub fn channel_map(&self, channel: &Channel, gem: &Gem, offset: u64, length: u64, rw: bool) -> IocResult<Mapping> {
        unsafe {
            let mut args: drm_tegra_channel_map = std::mem::zeroed();

            args.channel_ctx = channel.context();
            args.handle = gem.handle();
            args.offset = offset;
            args.length = length;
            if rw {
                args.flags = DRM_TEGRA_CHANNEL_MAP_READWRITE;
            }

            args = self.channel_map_raw(args)?;

            Ok(Mapping {
                drm: self,
                id: args.mapping_id,
                context: channel.context(),
                iova: args.iova,
            })
        }
    }

    pub fn channel_unmap_raw(&self, mut args: drm_tegra_channel_unmap) -> IocResult<()> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_TEGRA_CHANNEL_UNMAP, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(())
        }
    }

    pub fn gem_create_raw(&self, mut args: drm_tegra_gem_create) -> IocResult<drm_tegra_gem_create> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_TEGRA_GEM_CREATE, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(args)
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

    pub fn gem_close_raw(&self, mut args: drm_gem_close) -> IocResult<()> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_GEM_CLOSE, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(())
        }
    }

    pub fn gem_mmap_raw(&self, mut args: drm_tegra_gem_mmap) -> IocResult<drm_tegra_gem_mmap> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_TEGRA_GEM_MMAP, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(args)
        }
    }

    pub fn channel_submit_raw(&self, mut args: drm_tegra_channel_submit) -> IocResult<drm_tegra_channel_submit> {
        unsafe {
            let err = ioctl(self.fd.as_raw_fd(), DRM_IOCTL_TEGRA_CHANNEL_SUBMIT, &mut args);
            if err != 0 {
                Err(Errno::get())?;
            }

            Ok(args)
        }
    }
}

pub struct Channel<'a> {
    drm: &'a Drm,
    ctx: u32,
    hw_version: u32,
}

impl Channel<'_> {
    pub fn context(&self) -> u32 {
        self.ctx
    }

    pub fn hw_version(&self) -> u32 {
        self.hw_version
    }
}

impl Drop for Channel<'_> {
    fn drop(&mut self) {
        let _ = self.drm.close_channel_raw(
            drm_tegra_channel_close {
                channel_ctx: self.ctx,
                ..unsafe { std::mem::zeroed() }
            });
    }
}

impl std::fmt::Debug for Channel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Channel({}, ver={:x})", self.ctx, self.hw_version)
    }
}

pub struct Gem<'a> {
    drm: &'a Drm,
    handle: u32,
}

impl Gem<'_> {
    pub fn handle(&self) -> u32 {
        self.handle
    }

    pub fn map(&self, length: usize) -> EResult<memmap::MmapMut> {
        unsafe {
            let mut args: drm_tegra_gem_mmap = std::mem::zeroed();

            args.handle = self.handle;

            let args = check_unwrap!(self.drm.gem_mmap_raw(args));

            let mmap = memmap::MmapOptions::new()
                .offset(args.offset)
                .len(length)
                .map_mut(&self.drm.fd)?;

            Ok(mmap)
        }
    }
}

impl Drop for Gem<'_> {
    fn drop(&mut self) {
        let _ = self.drm.gem_close_raw(drm_gem_close { handle: self.handle, pad: 0 });
    }
}

impl std::fmt::Debug for Gem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Gem#{}", self.handle)
    }
}

pub struct Mapping<'a> {
    drm: &'a Drm,
    id: u32,
    context: u32,
    #[allow(unused)]
    iova: u64,
}

impl Mapping<'_> {
    pub fn id(&self) -> u32 {
        self.id
    }

    #[allow(unused)]
    pub fn iova(&self) -> u64 {
        self.iova
    }

    pub fn drop(mut self) -> IocResult<()> {
        let r = self.drm.channel_unmap_raw(
            drm_tegra_channel_unmap {
                channel_ctx: self.context,
                mapping_id: self.id,
                ..unsafe { std::mem::zeroed() }
            });
        self.id = 0;
        r
    }
}

impl Drop for Mapping<'_> {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }
        let _ = self.drm.channel_unmap_raw(
            drm_tegra_channel_unmap {
                channel_ctx: self.context,
                mapping_id: self.id,
                ..unsafe { std::mem::zeroed() }
            });
    }
}

impl std::fmt::Debug for Mapping<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mapping(ctx={}, id={})", self.context, self.id)
    }
}
