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

use crate::{tegra_drm, vic, Main, EResult, IocResult, Errno, EINVAL, ENODEV, ENOSPC, EFAULT, EPERM};
use crate::types::{Channel, Syncpoint, Fence, Mapping, VGem};

pub fn test_open_channel_invalid_ioctl(main: &Main) -> EResult<()> {
    let zero: tegra_drm::drm_tegra_channel_open = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.flags = 0xffff_ffff;
        check_eq!(main.drm.open_channel_raw(args).err(), Some(EINVAL));
    }

    {
        let mut args = zero;
        args.host1x_class = 0x0;
        check_eq!(main.drm.open_channel_raw(args).err(), Some(ENODEV));
    }

    {
        let mut args = zero;
        args.host1x_class = 0xff;
        check_eq!(main.drm.open_channel_raw(args).err(), Some(ENODEV));
    }

    Ok(())
}

pub fn test_open_close_channel(main: &Main) -> EResult<()> {
    let channel = check_unwrap!(main.drm.open_channel(main.engine_class));
    let zero: tegra_drm::drm_tegra_channel_close = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.channel_ctx = channel.context();
        check!(main.drm.close_channel_raw(args).is_ok());
        check_eq!(main.drm.close_channel_raw(args).err(), Some(EINVAL));
    }

    Ok(())
}

pub fn test_engine_metadata(main: &Main) -> EResult<()> {
    let channel = check_unwrap!(main.drm.open_channel(main.engine_class));

    check_eq!(channel.hw_version(), main.soc.chip_id());

    Ok(())
}

pub fn test_gem_create_invalid_ioctl(main: &Main) -> EResult<()> {
    let zero: tegra_drm::drm_tegra_gem_create = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.flags = 0xffff_ffff;
        check_eq!(main.drm.gem_create_raw(args).err(), Some(EINVAL));
    }

    {
        let mut args = zero;
        args.size = 0xffff_ffff_ffff;
        check_eq!(main.drm.gem_create_raw(args).err(), Some(ENOSPC));
    }

    Ok(())
}

pub fn test_gem_mmap_invalid_ioctl(main: &Main) -> EResult<()> {
    let zero: tegra_drm::drm_tegra_gem_mmap = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.pad = 0xffff_ffff;
        check_eq!(main.drm.gem_mmap_raw(args).err(), Some(EINVAL));
    }

    {
        let mut args = zero;
        args.handle = 0xdeadbeef;
        check_eq!(main.drm.gem_mmap_raw(args).err(), Some(EINVAL));
    }

    Ok(())
}

pub fn test_gem_mmap(main: &Main) -> EResult<()> {
    let gem = check_unwrap!(main.drm.gem_create(0x1000));

    {
        let mut mmap = check_unwrap!(gem.map(0x1000));
        mmap[16] = 0xda;

        check_eq!(mmap[16], 0xda);
    }

    {
        let mmap = check_unwrap!(gem.map(0x1000));

        check_eq!(mmap[16], 0xda);
    }

    Ok(())
}

pub fn test_channel_map_invalid_ioctl(main: &Main) -> EResult<()> {
    let channel = check_unwrap!(main.drm.open_channel(main.engine_class));

    let zero: tegra_drm::drm_tegra_channel_map = unsafe { std::mem::zeroed() };

    {
        let mut args = zero;
        args.channel_ctx = 0;
        check_eq!(main.drm.channel_map_raw(args).err(), Some(EINVAL));
    }

    {
        let mut args = zero;
        args.channel_ctx = channel.context();
        args.handle = 0xdeadbeef;
        check_eq!(main.drm.channel_map_raw(args).err(), Some(EINVAL));
    }

    let gem = check_unwrap!(main.drm.gem_create(0x1000));

    {
        let mut args = zero;
        args.channel_ctx = channel.context();
        args.flags = 0xffffffff;
        args.handle = gem.handle();
        check_eq!(main.drm.channel_map_raw(args).err(), Some(EINVAL));
    }

    Ok(())
}

pub fn test_channel_map_unmap(main: &Main) -> EResult<()> {
    let channel = check_unwrap!(main.drm.open_channel(main.engine_class));
    let gem = check_unwrap!(main.drm.gem_create(0x1000));

    let m = check_unwrap!(main.drm.channel_map(&channel, &gem, 0x0, 0x1000, false));

    {
        let mut args: tegra_drm::drm_tegra_channel_unmap = unsafe { std::mem::zeroed() };
        args.channel_ctx = channel.context();
        args.mapping_id = m.id();

        check!(main.drm.channel_unmap_raw(args).is_ok());
        check_eq!(main.drm.channel_unmap_raw(args).err(), Some(EINVAL));
        std::mem::forget(m);
    }

    Ok(())
}

pub fn test_channel_map_gem_close(main: &Main) -> EResult<()> {
    let channel = check_unwrap!(main.drm.open_channel(main.engine_class));
    let gem = check_unwrap!(main.drm.gem_create(0x1000));
    let m = check_unwrap!(main.drm.channel_map(&channel, &gem, 0x0, 0x1000, false));

    drop(gem);

    check_unwrap!(m.drop());

    Ok(())
}

struct SubmitTestCtx<'a> {
    main: &'a Main,
    channel: Channel<'a>,
    syncpt: Syncpoint,
    syncpt_id: u32,
    args: tegra_drm::drm_tegra_channel_submit,
    incr: Vec<tegra_drm::drm_tegra_submit_syncpt_incr>,
    cmd: Vec<tegra_drm::drm_tegra_submit_cmd>,
    buf: Vec<tegra_drm::drm_tegra_submit_buf>,
    gather_data: Vec<u32>,
}

impl SubmitTestCtx<'_> {
    fn setup_submit(&mut self) {
        assert!(self.incr.len() < 2);
        if self.incr.len() == 1 {
            self.args.syncpt_incr = self.incr[0];
        }

        self.args.num_cmds = self.cmd.len() as u32;
        self.args.cmds_ptr = self.cmd.as_ptr() as u64;

        self.args.num_bufs = self.buf.len() as u32;
        self.args.bufs_ptr = self.buf.as_ptr() as u64;

        self.args.gather_data_words = self.gather_data.len() as u32;
        self.args.gather_data_ptr = self.gather_data.as_ptr() as u64;
    }

    fn submit(&mut self, main: &Main) -> IocResult<()> {
        self.setup_submit();
        self.submit_raw(main)
    }

    fn submit_raw(&mut self, main: &Main) -> IocResult<()> {
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
        buf.mapping_id = mapping.id();
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
            if incr.syncpt_fd == self.syncpt.fd() {
                incr.num_incrs += 1;
                return;
            }
        }

        let mut incr: tegra_drm::drm_tegra_submit_syncpt_incr = unsafe { std::mem::zeroed() };
        incr.syncpt_fd = self.syncpt.fd();
        incr.num_incrs = 1;
        self.incr.push(incr);
    }
}

fn submit_test<T>(main: &Main, f: impl FnOnce(SubmitTestCtx) -> EResult<T>) -> EResult<T> {
    let channel = main.drm.open_channel(main.engine_class)?;
    let syncpt = main.host1x.allocate_syncpoint()?;
    let syncpt_id = syncpt.id()?;

    let mut args: tegra_drm::drm_tegra_channel_submit = unsafe { std::mem::zeroed() };
    let incr = Vec::new();
    let cmd = Vec::new();
    let buf = Vec::new();
    let gather_data = Vec::new();

    args.channel_ctx = channel.context();

    (f)(SubmitTestCtx { main, channel, syncpt, syncpt_id, args, incr, cmd, buf, gather_data })
}

pub fn test_channel_submit_invalid_ioctl(main: &Main) -> EResult<()> {
    fn submit(main: &Main, f: impl FnOnce(&mut SubmitTestCtx)) -> EResult<Option<Errno>> {
        submit_test(main, |mut ctx| {
            ctx.push_syncpt_incr(0);
            ctx.setup_submit();
            (f)(&mut ctx);
            Ok(ctx.submit_raw(main).err())
        })
    }

    /* Submit otherwise good jobs, but perturb them slightly to make them invalid. */

    check_eq!(submit(main, |_c| ())?, None);
    check_eq!(submit(main, |c| { c.args.gather_data_ptr = 0; c.args.gather_data_words = 1 })?, Some(EFAULT));
    check_eq!(submit(main, |c| { c.args.bufs_ptr = 0; c.args.num_bufs = 1 })?, Some(EFAULT));
    check_eq!(submit(main, |c| { c.args.cmds_ptr = 0; c.args.num_cmds = 1 })?, Some(EFAULT));

    check_eq!(submit(main, |c| c.cmd[0].__bindgen_anon_1.gather_uptr.words = 1000)?, Some(EINVAL));

    check_eq!(submit(main, |c| c.args.syncpt_incr.flags = 0xffffffff)?, Some(EINVAL));
    check_eq!(submit(main, |c| c.args.syncpt_incr.syncpt_fd = 0)?, Some(EINVAL));

    Ok(())
}

pub fn test_channel_submit_increment_syncpoint_twice(main: &Main) -> EResult<()> {
    submit_test(main, |mut ctx| {
        let base_value = main.host1x.read_syncpoint(ctx.syncpt_id)?;

        let mut incr: tegra_drm::drm_tegra_submit_syncpt_incr = unsafe { std::mem::zeroed() };
        incr.syncpt_fd = ctx.syncpt.fd();
        incr.num_incrs = 2;
        ctx.incr.push(incr);

        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);
        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = ctx.gather_data.len() as u32;
        ctx.cmd.push(cmd);

        ctx.submit(main)?;

        /*
         * The kernel is allowed to insert extra increments at the beginning of the job,
         * so allow for more than 2 increments.
         */
        check!(ctx.args.syncpt_incr.fence_value.wrapping_sub(base_value) >= 2);

        let fence = main.host1x.create_fence(ctx.syncpt_id, ctx.args.syncpt_incr.fence_value)?;
        check!(fence.wait(1000).is_ok());

        Ok(())
    })
}

pub fn test_channel_submit_vic_clear(main: &Main) -> EResult<()> {
    let cfg_gem = check_unwrap!(main.drm.gem_create(0x1000));
    let mut cfg_map = check_unwrap!(cfg_gem.map(0x1000));

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

    let surf_gem = check_unwrap!(main.drm.gem_create(32768));
    let filt_gem = check_unwrap!(main.drm.gem_create(0x3000));

    submit_test(main, |mut ctx| {
        let cfg_m = check_unwrap!(main.drm.channel_map(&ctx.channel, &cfg_gem, 0x0, 0x1000, false));
        let surf_m = check_unwrap!(main.drm.channel_map(&ctx.channel, &surf_gem, 0x0, 32768, true));
        let filt_m = check_unwrap!(main.drm.channel_map(&ctx.channel, &filt_gem, 0x0, 0x3000, false));

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

        let fence = main.host1x.create_fence(ctx.syncpt_id, ctx.args.syncpt_incr.fence_value)?;
        check!(fence.wait(1000).is_ok());

        Ok(())
    })?;

    let surf_map = check_unwrap!(surf_gem.map(0x1000));
    let pixels: &[u32] = bytemuck::cast_slice(&surf_map[0..0x1000]);
    check_eq!(pixels[0], 0xff00ff00);

    Ok(())
}

pub fn test_channel_submit_timeout(main: &Main) -> EResult<()> {
    submit_test(main, |mut ctx| {
        /* First, submit failing job */

        let mut incr: tegra_drm::drm_tegra_submit_syncpt_incr = unsafe { std::mem::zeroed() };
        incr.syncpt_fd = ctx.syncpt.fd();
        incr.num_incrs = 2;
        ctx.incr.push(incr);

        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = ctx.gather_data.len() as u32;
        ctx.cmd.push(cmd);

        ctx.submit(main)?;

        let a = ctx.args.syncpt_incr.fence_value;

        /* Then, submit OK job */

        ctx.incr[0].num_incrs = 1;
        ctx.submit(main)?;

        let b = ctx.args.syncpt_incr.fence_value;

        let fence_a = main.host1x.create_fence(ctx.syncpt_id, a)?;
        let fence_b = main.host1x.create_fence(ctx.syncpt_id, b)?;
        check!(fence_a.wait(20000).is_err());
        check!(fence_b.wait(1000).is_err());

        /* Further submission should be rejected */

        check_eq!(ctx.submit(main).err(), Some(EPERM));

        Ok(())
    })?;

    submit_test(main, |mut ctx| {
        /* Ensure channel is again usable (with a different syncpoint) */

        let mut incr: tegra_drm::drm_tegra_submit_syncpt_incr = unsafe { std::mem::zeroed() };
        incr.syncpt_fd = ctx.syncpt.fd();
        incr.num_incrs = 1;
        ctx.incr.push(incr);

        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = ctx.gather_data.len() as u32;
        ctx.cmd.push(cmd);

        ctx.submit(main)?;

        let fence = main.host1x.create_fence(ctx.syncpt_id, ctx.args.syncpt_incr.fence_value)?;
        check!(fence.wait(1000).is_ok());

        Ok(())
    })
}

pub fn test_channel_submit_wait(main: &Main) -> EResult<()> {
    let syncpt = main.host1x.allocate_syncpoint()?;
    let syncpt_id = syncpt.id()?;
    let value = main.host1x.read_syncpoint(syncpt_id)?;

    submit_test(main, |mut ctx| {
        let mut incr: tegra_drm::drm_tegra_submit_syncpt_incr = unsafe { std::mem::zeroed() };
        incr.syncpt_fd = ctx.syncpt.fd();
        incr.num_incrs = 2;
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
        cmd.__bindgen_anon_1.wait_syncpt.threshold = value.wrapping_add(1);
        ctx.cmd.push(cmd);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = 2;
        ctx.cmd.push(cmd);

        ctx.submit(main)?;

        let fence = main.host1x.create_fence(ctx.syncpt_id, ctx.args.syncpt_incr.fence_value-1)?;
        check!(fence.wait(1000).is_ok());

        let fence = main.host1x.create_fence(ctx.syncpt_id, ctx.args.syncpt_incr.fence_value)?;
        check!(fence.wait(100).is_err());

        syncpt.increment(1)?;

        let fence = main.host1x.create_fence(ctx.syncpt_id, ctx.args.syncpt_incr.fence_value)?;
        check!(fence.wait(1000).is_ok());

        Ok(())
    })
}

pub fn test_channel_buf_refcounting(main: &Main) -> EResult<()> {
    let syncpt = main.host1x.allocate_syncpoint()?;
    let syncpt_id = syncpt.id()?;
    let value = main.host1x.read_syncpoint(syncpt_id)?;

    let cfg_gem = check_unwrap!(main.drm.gem_create(0x1000));
    let mut cfg_map = check_unwrap!(cfg_gem.map(0x1000));

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

    let surf_gem = check_unwrap!(main.drm.gem_create(32768));
    let filt_gem = check_unwrap!(main.drm.gem_create(0x3000));

    submit_test(main, |mut ctx| {
        let cfg_m = check_unwrap!(main.drm.channel_map(&ctx.channel, &cfg_gem, 0x0, 0x1000, false));
        let surf_m = check_unwrap!(main.drm.channel_map(&ctx.channel, &surf_gem, 0x0, 32768, true));
        let filt_m = check_unwrap!(main.drm.channel_map(&ctx.channel, &filt_gem, 0x0, 0x3000, false));

        let mut incr: tegra_drm::drm_tegra_submit_syncpt_incr = unsafe { std::mem::zeroed() };
        incr.syncpt_fd = ctx.syncpt.fd();
        incr.num_incrs = 2;
        ctx.incr.push(incr);

        ctx.gather_data.extend_from_slice(&[0x1_000_0001, ctx.syncpt_id]);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR;
        cmd.__bindgen_anon_1.gather_uptr.words = 2;
        ctx.cmd.push(cmd);

        let mut cmd: tegra_drm::drm_tegra_submit_cmd = unsafe { std::mem::zeroed() };
        cmd.type_ = tegra_drm::DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT;
        cmd.__bindgen_anon_1.wait_syncpt.id = syncpt_id;
        cmd.__bindgen_anon_1.wait_syncpt.threshold = value.wrapping_add(1);
        ctx.cmd.push(cmd);

        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x704>>2, (2960 / 16) << 16]);
        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x708>>2, 0x0]);

        let mut buf: tegra_drm::drm_tegra_submit_buf = unsafe { std::mem::zeroed() };
        buf.mapping_id = cfg_m.id();
        buf.reloc.gather_offset_words = (ctx.gather_data.len()-1) as _;
        buf.reloc.target_offset = 0;
        buf.reloc.shift = 8;
        ctx.buf.push(buf);

        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x720>>2, 0x0]);

        let mut buf: tegra_drm::drm_tegra_submit_buf = unsafe { std::mem::zeroed() };
        buf.mapping_id = surf_m.id();
        buf.reloc.gather_offset_words = (ctx.gather_data.len()-1) as _;
        buf.reloc.target_offset = 0;
        buf.reloc.shift = 8;
        ctx.buf.push(buf);

        ctx.gather_data.extend_from_slice(&[0x1_010_0002, 0x70c>>2, 0x0]);

        let mut buf: tegra_drm::drm_tegra_submit_buf = unsafe { std::mem::zeroed() };
        buf.mapping_id = filt_m.id();
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

        let fence = main.host1x.create_fence(ctx.syncpt_id, ctx.args.syncpt_incr.fence_value-1)?;
        check!(fence.wait(1000).is_ok());

        drop(cfg_m);
        drop(filt_m);
        drop(surf_m);
        drop(cfg_gem);
        drop(filt_gem);

        syncpt.increment(1)?;

        let fence = main.host1x.create_fence(ctx.syncpt_id, ctx.args.syncpt_incr.fence_value)?;
        check!(fence.wait(1000).is_ok());

        Ok(())
    })?;

    let surf_map = check_unwrap!(surf_gem.map(0x1000));
    let pixels: &[u32] = bytemuck::cast_slice(&surf_map[0..0x1000]);
    check_eq!(pixels[0], 0xff00ff00);

    Ok(())
}
