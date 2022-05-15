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

use std::time::Duration;
use anyhow::Result;

use crate::{EFAULT, EINVAL, EPERM, Errno, Main, tegra_drm, uapi::{Channel, Mapping, Syncpoint}, vic};

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
                    cmd.__bindgen_anon_1.gather_uptr.words += words.len() as u32;
                }
                return;
            }
        }

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = words.len() as u32;
        self.cmd.push(cmd);
    }

    fn push_buf(&mut self, mapping: &Mapping) {
        let mut buf: tegra_drm::drm_tegra_submit_buf = unsafe { std::mem::zeroed() };
        buf.mapping = mapping.id();
        buf.reloc.gather_offset_words = self.gather_data.len() as _;
        buf.reloc.target_offset = 0;
        buf.reloc.shift = 8;
        self.buf.push(buf);

        self.push(&[0xdead0000]);
    }

    fn push_syncpt_incr(&mut self, condition: u32) {
        /* TODO this needs to check chip */
        self.push(&[0x1_000_0001,
            self.syncpt_id|(condition << self.main.soc.condition_shift())]);

        for incr in &mut self.incr {
            if incr.id == self.syncpt_id {
                incr.increments += 1;
                return;
            }
        }

        let mut incr: tegra_drm::drm_tegra_submit_syncpt = unsafe { std::mem::zeroed() };
        incr.id = self.syncpt_id;
        incr.increments = 1;
        self.incr.push(incr);
    }
}

fn submit_test<T>(main: &Main, f: impl FnOnce(SubmitTestCtx) -> Result<T>) -> Result<T> {
    let channel = main.drm.open_channel(main.engine_class)?;
    let syncpt = main.drm.allocate_syncpoint()?;
    let syncpt_id = syncpt.id();

    let mut args: tegra_drm::drm_tegra_channel_submit = unsafe { std::mem::zeroed() };
    let incr = Vec::new();
    let cmd = Vec::new();
    let buf = Vec::new();
    let gather_data = Vec::new();

    args.context = channel.context();

    (f)(SubmitTestCtx { main, channel, syncpt, syncpt_id, args, incr, cmd, buf, gather_data })
}

pub fn test_channel_submit_invalid_ioctl(main: &Main) -> Result<()> {
    fn submit(main: &Main, f: impl FnOnce(&mut SubmitTestCtx)) -> Result<Option<Errno>> {
        submit_test(main, |mut ctx| {
            ctx.push_syncpt_incr(0);
            ctx.setup_submit();
            (f)(&mut ctx);
            Ok(ctx.submit_raw(main).err())
        })
    }

    /* Submit otherwise good jobs, but perturb them slightly to make them invalid. */

    check_eq!(submit(main, |_c| ())?, None, "expected success but got {left:?}");
    check_eq!(submit(main, |c| { c.args.gather_data_ptr = 0; c.args.gather_data_words = 1 })?, Some(EFAULT), "expected EFAULT but got {left:?}");
    check_eq!(submit(main, |c| { c.args.bufs_ptr = 0; c.args.num_bufs = 1 })?, Some(EFAULT), "expected EFAULT but got {left:?}");
    check_eq!(submit(main, |c| { c.args.cmds_ptr = 0; c.args.num_cmds = 1 })?, Some(EFAULT), "expected EFAULT but got {left:?}");

    check_eq!(submit(main, |c| c.cmd[0].__bindgen_anon_1.gather_uptr.words = 1000)?, Some(EINVAL), "expected EINVAL but got {left:?}");

    check_eq!(submit(main, |c| c.args.syncpt.flags = 0xffffffff)?, Some(EINVAL), "expected EINVAL but got {left:?}");
    check_eq!(submit(main, |c| c.args.syncpt.id = 0)?, Some(EINVAL), "expected EINVAL but got {left:?}");

    Ok(())
}

pub fn test_channel_submit_increment_syncpoint_twice(main: &Main) -> Result<()> {
    submit_test(main, |mut ctx| {
        let base_value = main.drm.read_syncpoint(ctx.syncpt_id)?;

        ctx.push_syncpt_incr(0);
        ctx.push_syncpt_incr(0);

        ctx.submit(main)?;

        /*
         * The kernel is allowed to insert extra increments at the beginning of the job,
         * so allow for more than 2 increments.
         */
        check!(ctx.args.syncpt.value.wrapping_sub(base_value) >= 2, "job fence value wasn't incremented by at least 2");

        main.drm.wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_secs(1))?;

        Ok(())
    })
}

pub fn test_channel_submit_vic_clear(main: &Main) -> Result<()> {
    let cfg_gem = main.drm.gem_create(0x1000)?;
    let mut cfg_map = cfg_gem.map(0x1000)?;

    {
        let b: &mut vic::OutputConfig = bytemuck::from_bytes_mut(&mut cfg_map[16..32]);
        b.set_TargetRectRight(31);
        b.set_TargetRectBottom(31);
        b.set_BackgroundAlpha(1023);
        b.set_BackgroundR(0);
        b.set_BackgroundG(1023);
        b.set_BackgroundB(0);
    }

    {
        let b: &mut vic::OutputSurfaceConfig = bytemuck::from_bytes_mut(&mut cfg_map[32..48]);
        b.set_OutPixelFormat(32 /* ARGB8888 */);
        b.set_OutSurfaceHeight(31);
        b.set_OutSurfaceWidth(31);
        b.set_OutLumaWidth(255);
        b.set_OutLumaHeight(31);
        b.set_OutChromaWidth(16383);
        b.set_OutChromaHeight(16383);
    }

    let surf_gem = main.drm.gem_create(32768)?;
    let filt_gem = main.drm.gem_create(0x3000)?;

    submit_test(main, |mut ctx| {
        let cfg_m = main.drm.channel_map(&ctx.channel, &cfg_gem, 0x0, 0x1000, false)?;
        let surf_m = main.drm.channel_map(&ctx.channel, &surf_gem, 0x0, 32768, true)?;
        let filt_m = main.drm.channel_map(&ctx.channel, &filt_gem, 0x0, 0x3000, false)?;

        ctx.push(&[0x1_010_0002, 0x200>>2, 1]);
        ctx.push(&[0x1_010_0002, 0x704>>2, (2960 / 16) << 16]);
        ctx.push(&[0x1_010_0002, 0x708>>2]);
        ctx.push_buf(&cfg_m);
        ctx.push(&[0x1_010_0002, 0x720>>2]);
        ctx.push_buf(&surf_m);
        ctx.push(&[0x1_010_0002, 0x70c>>2]);
        ctx.push_buf(&filt_m);
        ctx.push(&[0x1_010_0002, 0x300>>2, 1<<8]);
        ctx.push_syncpt_incr(1);

        ctx.submit(main)?;

        main.drm.wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_secs(1))?;

        Ok(())
    })?;

    let surf_map = surf_gem.map(0x1000)?;
    let pixels: &[u32] = bytemuck::cast_slice(&surf_map[0..0x1000]);
    check_eq!(pixels[0], 0xff00ff00, "Expected 0xff00ff00, got 0x{left:08x}");

    Ok(())
}

pub fn test_channel_submit_timeout(main: &Main) -> Result<()> {
    submit_test(main, |mut ctx| {
        /* First, submit failing job */

        let mut incr: tegra_drm::drm_tegra_submit_syncpt = unsafe { std::mem::zeroed() };
        incr.id = ctx.syncpt.id();
        incr.increments = 3;
        ctx.incr.push(incr);

        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = ctx.gather_data.len() as u32;
        ctx.cmd.push(cmd);

        ctx.submit(main)?;

        let a = ctx.args.syncpt.value;

        /* Then, submit OK job */

        ctx.incr[0].increments = 1;
        ctx.submit(main)?;

        let b = ctx.args.syncpt.value;

        check!(main.drm.wait_syncpoint(ctx.syncpt_id, a, Duration::from_secs(20)).is_err(), "syncpoint was incremented unexpectedly");
        check!(main.drm.wait_syncpoint(ctx.syncpt_id, b, Duration::from_secs(1)).is_err(), "follow-up job succeeded though channel should be in error state");

        /* Further submission should be rejected */

        check_err!(ctx.submit(main), EPERM, "expected submission to fail with locked syncpoint, but got {left:?}");

        Ok(())
    })?;

    submit_test(main, |mut ctx| {
        /* Ensure channel is again usable (with a different syncpoint) */

        let mut incr: tegra_drm::drm_tegra_submit_syncpt = unsafe { std::mem::zeroed() };
        incr.id = ctx.syncpt.id();
        incr.increments = 1;
        ctx.incr.push(incr);

        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = ctx.gather_data.len() as u32;
        ctx.cmd.push(cmd);

        ctx.submit(main)?;

        main.drm.wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_secs(1))?;

        Ok(())
    })
}

pub fn test_channel_submit_wait(main: &Main) -> Result<()> {
    let syncpt = main.drm.allocate_syncpoint()?;
    let syncpt_id = syncpt.id();
    let value = main.drm.read_syncpoint(syncpt_id)?;

    submit_test(main, |mut ctx| {
        let mut incr: tegra_drm::drm_tegra_submit_syncpt = unsafe { std::mem::zeroed() };
        incr.id = ctx.syncpt.id();
        incr.increments = 2;
        ctx.incr.push(incr);

        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);
        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = 2;
        ctx.cmd.push(cmd);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT;
        cmd.__bindgen_anon_1.wait_syncpt.id = syncpt_id;
        cmd.__bindgen_anon_1.wait_syncpt.value = value.wrapping_add(1);
        ctx.cmd.push(cmd);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = 2;
        ctx.cmd.push(cmd);

        ctx.submit(main)?;

        main.drm.wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value-1, Duration::from_millis(1000))?;
        check!(main.drm.wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_millis(100)).is_err(), "expected error");

        syncpt.increment(1)?;

        main.drm.wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_millis(1000))?;

        Ok(())
    })
}

pub fn test_channel_buf_refcounting(main: &Main) -> Result<()> {
    let syncpt = main.drm.allocate_syncpoint()?;
    let syncpt_id = syncpt.id();
    let value = main.drm.read_syncpoint(syncpt_id)?;

    let cfg_gem = main.drm.gem_create(0x1000)?;
    let mut cfg_map = cfg_gem.map(0x1000)?;

    {
        let b: &mut vic::OutputConfig = bytemuck::from_bytes_mut(&mut cfg_map[16..32]);
        b.set_TargetRectRight(31);
        b.set_TargetRectBottom(31);
        b.set_BackgroundAlpha(1023);
        b.set_BackgroundR(0);
        b.set_BackgroundG(1023);
        b.set_BackgroundB(0);
    }

    {
        let b: &mut vic::OutputSurfaceConfig = bytemuck::from_bytes_mut(&mut cfg_map[32..48]);
        b.set_OutPixelFormat(32);
        b.set_OutSurfaceHeight(31);
        b.set_OutSurfaceWidth(31);
        b.set_OutLumaWidth(255);
        b.set_OutLumaHeight(31);
        b.set_OutChromaWidth(16383);
        b.set_OutChromaHeight(16383);
    }

    let surf_gem = main.drm.gem_create(32768)?;
    let filt_gem = main.drm.gem_create(0x3000)?;

    submit_test(main, |mut ctx| {
        let cfg_m = main.drm.channel_map(&ctx.channel, &cfg_gem, 0x0, 0x1000, false)?;
        let surf_m = main.drm.channel_map(&ctx.channel, &surf_gem, 0x0, 32768, true)?;
        let filt_m = main.drm.channel_map(&ctx.channel, &filt_gem, 0x0, 0x3000, false)?;

        let mut incr: tegra_drm::drm_tegra_submit_syncpt = unsafe { std::mem::zeroed() };
        incr.id = ctx.syncpt.id();
        incr.increments = 2;
        ctx.incr.push(incr);

        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = 2;
        ctx.cmd.push(cmd);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT;
        cmd.__bindgen_anon_1.wait_syncpt.id = syncpt_id;
        cmd.__bindgen_anon_1.wait_syncpt.value = value.wrapping_add(1);
        ctx.cmd.push(cmd);

        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x200>>2, 1]);
        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x704>>2, (2960 / 16) << 16]);
        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x708>>2, 0x0]);

        let mut buf: tegra_drm::drm_tegra_submit_buf = unsafe { std::mem::zeroed() };
        buf.mapping = cfg_m.id();
        buf.reloc.gather_offset_words = (ctx.gather_data.len()-1) as _;
        buf.reloc.target_offset = 0;
        buf.reloc.shift = 8;
        ctx.buf.push(buf);

        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x720>>2, 0x0]);

        let mut buf: tegra_drm::drm_tegra_submit_buf = unsafe { std::mem::zeroed() };
        buf.mapping = surf_m.id();
        buf.reloc.gather_offset_words = (ctx.gather_data.len()-1) as _;
        buf.reloc.target_offset = 0;
        buf.reloc.shift = 8;
        ctx.buf.push(buf);

        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x70c>>2, 0x0]);

        let mut buf: tegra_drm::drm_tegra_submit_buf = unsafe { std::mem::zeroed() };
        buf.mapping = filt_m.id();
        buf.reloc.gather_offset_words = (ctx.gather_data.len()-1) as _;
        buf.reloc.target_offset = 0;
        buf.reloc.shift = 8;
        ctx.buf.push(buf);

        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x300>>2, 1<<8]);
        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id|(1<<10)]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = ctx.gather_data.len() as u32 - 2;
        ctx.cmd.push(cmd);

        ctx.submit(main)?;

        main.drm.wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value-1, Duration::from_millis(1000))?;

        drop(cfg_m);
        drop(filt_m);
        drop(surf_m);
        drop(cfg_gem);
        drop(filt_gem);

        syncpt.increment(1)?;

        main.drm.wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_millis(1000))?;

        Ok(())
    })?;

    let surf_map = surf_gem.map(0x1000)?;
    let pixels: &[u32] = bytemuck::cast_slice(&surf_map[0..0x1000]);
    check_eq!(pixels[0], 0xff00ff00, "Expected 0xff00ff00, got 0x{left:08x}");

    Ok(())
}
