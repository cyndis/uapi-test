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

use crate::tegra_drm::*;
use super::Drm;

pub struct Gem<'a> {
    pub(super) drm: &'a Drm,
    pub(super) handle: u32,
}

impl Gem<'_> {
    pub fn handle(&self) -> u32 {
        self.handle
    }

    pub fn map(&self, length: usize) -> anyhow::Result<memmap::MmapMut> {
        unsafe {
            let mut args: drm_tegra_gem_mmap = std::mem::zeroed();

            args.handle = self.handle;

            let args = self.drm.gem_mmap_raw(args)?;

            let mmap = memmap::MmapOptions::new()
                .offset(args.offset)
                .len(length)
                .map_mut(&self.drm.fd)?;

            Ok(mmap)
        }
    }

    #[allow(unused)]
    pub fn export(&self) -> anyhow::Result<i32> {
        unsafe {
            let mut args: drm_prime_handle = std::mem::zeroed();

            args.handle = self.handle;
            args.flags = 2;

            let args = self.drm.prime_handle_to_fd(args)?;

            Ok(args.fd)
        }
    }
}

impl Drop for Gem<'_> {
    fn drop(&mut self) {
        let _ = self.drm.gem_close_raw(drm_gem_close {
            handle: self.handle,
            pad: 0,
        });
    }
}

impl std::fmt::Debug for Gem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Gem#{}", self.handle)
    }
}