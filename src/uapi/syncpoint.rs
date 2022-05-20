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

use super::Drm;
use crate::tegra_drm::*;
use crate::IocResult;

pub struct Syncpoint<'a> {
    pub(super) drm: &'a Drm,
    pub(super) id: u32,
}

impl Syncpoint<'_> {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn increment(&self, n: usize) -> IocResult<()> {
        for _ in 0..n {
            self.drm.increment_syncpoint(self.id)?;
        }

        Ok(())
    }
}

impl Drop for Syncpoint<'_> {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }
        let _ = self.drm.free_syncpoint_raw(drm_tegra_syncpoint_free {
            id: self.id,
            ..unsafe { std::mem::zeroed() }
        });
    }
}

impl std::fmt::Debug for Syncpoint<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Syncpoint({})", self.id)
    }
}
