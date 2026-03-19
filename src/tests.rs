/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2022 NVIDIA Corporation
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

use crate::{
    tegra_drm,
    uapi::{Channel, Mapping, Syncpoint},
    Main,
};

use anyhow::{Context, Result};
use bytemuck::Zeroable;

pub mod channels;
pub mod gem;
pub mod nvdec;
pub mod submit;
pub mod syncpoints;
pub mod vic;

struct SubmitTestCtx<'a> {
    main: &'a Main,
    channel: Channel<'a>,
    syncpt: Syncpoint<'a>,
    syncpt_id: u32,
    args: tegra_drm::drm_tegra_channel_submit,
    incr: Vec<tegra_drm::drm_tegra_submit_syncpt>,
    cmd: Vec<tegra_drm::drm_tegra_submit_cmd>,
    buf: Vec<tegra_drm::drm_tegra_submit_buf>,
    gather_data: Vec<u32>,
}

impl SubmitTestCtx<'_> {
    fn setup_submit(&mut self) {
        assert!(self.incr.len() < 2);
        if self.incr.len() == 1 {
            self.args.syncpt = self.incr[0];
        }

        self.args.num_cmds = self.cmd.len() as u32;
        self.args.cmds_ptr = self.cmd.as_ptr() as u64;

        self.args.num_bufs = self.buf.len() as u32;
        self.args.bufs_ptr = self.buf.as_ptr() as u64;

        self.args.gather_data_words = self.gather_data.len() as u32;
        self.args.gather_data_ptr = self.gather_data.as_ptr() as u64;
    }

    fn submit(&mut self, main: &Main) -> crate::IocResult<()> {
        self.setup_submit();
        self.submit_raw(main)
    }

    fn submit_raw(&mut self, main: &Main) -> crate::IocResult<()> {
        self.args = main.drm.channel_submit_raw(self.args)?;

        Ok(())
    }

    fn push(&mut self, words: &[u32]) {
        self.gather_data.extend_from_slice(words);

        if let Some(cmd) = self.cmd.last_mut() {
            if cmd.type_ == tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR {
                unsafe {
                    cmd.data.gather_uptr.words += words.len() as u32;
                }
                return;
            }
        }

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = Zeroable::zeroed();
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.data.gather_uptr.words = words.len() as u32;
        self.cmd.push(cmd);
    }

    fn push_buf(&mut self, mapping: &Mapping) {
        self.push_buf_offset(mapping, 0)
    }

    fn push_buf_offset(&mut self, mapping: &Mapping, offset: usize) {
        let mut buf: tegra_drm::drm_tegra_submit_buf = Zeroable::zeroed();
        buf.mapping = mapping.id();
        buf.reloc.gather_offset_words = self.gather_data.len() as _;
        buf.reloc.target_offset = offset as _;
        buf.reloc.shift = 8;
        self.buf.push(buf);

        self.push(&[0xdead0000]);
    }

    fn set_increments(&mut self, id: u32, count: u32) {
        for incr in &mut self.incr {
            if incr.id == id {
                incr.increments = count;
                return;
            }
        }
        let mut incr: tegra_drm::drm_tegra_submit_syncpt = Zeroable::zeroed();
        incr.id = id;
        incr.increments = count;
        self.incr.push(incr);
    }

    fn push_wait_syncpt(&mut self, id: u32, value: u32) {
        let mut cmd: tegra_drm::drm_tegra_submit_cmd = Zeroable::zeroed();
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT;
        cmd.data.wait_syncpt.id = id;
        cmd.data.wait_syncpt.value = value;
        self.cmd.push(cmd);
    }

    fn push_incr_syncpt(&mut self, id: u32, condition: u32) {
        self.push(&[
            0x1_000_0001,
            id | (condition << self.main.soc.condition_shift()),
        ]);
    }

    fn push_syncpt_incr(&mut self, condition: u32) {
        self.push_incr_syncpt(self.syncpt_id, condition);

        for incr in &mut self.incr {
            if incr.id == self.syncpt_id {
                incr.increments += 1;
                return;
            }
        }

        let mut incr: tegra_drm::drm_tegra_submit_syncpt = Zeroable::zeroed();
        incr.id = self.syncpt_id;
        incr.increments = 1;
        self.incr.push(incr);
    }

    fn push_method(&mut self, reg: u32, value: u32) {
        self.push(&[0x1_010_0002, reg >> 2, value]);
    }

    fn push_method_buf(&mut self, reg: u32, mapping: &Mapping) {
        self.push(&[0x1_010_0002, reg >> 2]);
        self.push_buf(mapping);
    }

    fn push_method_buf_offset(&mut self, reg: u32, mapping: &Mapping, offset: usize) {
        self.push(&[0x1_010_0002, reg >> 2]);
        self.push_buf_offset(mapping, offset);
    }
}

fn submit_test<T>(
    main: &Main,
    class: u32,
    f: impl FnOnce(SubmitTestCtx) -> Result<T>,
) -> Result<T> {
    let channel = main.drm.open_channel(class)
        .context(format!("failed to open channel for class 0x{class:x}"))?;
    let syncpt = main.drm.allocate_syncpoint()
        .context("failed to allocate syncpoint")?;
    let syncpt_id = syncpt.id();

    let mut args: tegra_drm::drm_tegra_channel_submit = Zeroable::zeroed();
    let incr = Vec::new();
    let cmd = Vec::new();
    let buf = Vec::new();
    let gather_data = Vec::new();

    args.context = channel.context();

    (f)(SubmitTestCtx {
        main,
        channel,
        syncpt,
        syncpt_id,
        args,
        incr,
        cmd,
        buf,
        gather_data,
    })
}
