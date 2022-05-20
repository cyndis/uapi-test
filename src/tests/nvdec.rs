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

use std::time::Duration;

use anyhow::Result;

use crate::{nvdec, tests::submit_test, Main};

static SLICE_DATA: &[u8] = &[
    0x00, 0x00, 0x01, 0x01, 0x13, 0xf2, 0x14, 0xa5, 0x2f, 0x99, 0xbf, 0x70, 0x80,
];
static TERMINATION_SEQUENCE: &[u8] = &[
    0x00, 0x00, 0x01, 0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xb7, 0x00, 0x00, 0x00, 0x00,
];

const QUANT_MAT_8X8INTRA: [u8; 64] = [
    8, 16, 19, 22, 26, 27, 29, 34, 16, 16, 22, 24, 27, 29, 34, 37, 19, 22, 26, 27, 29, 34, 34, 38,
    22, 22, 26, 27, 29, 34, 37, 40, 22, 26, 27, 29, 32, 35, 40, 48, 26, 27, 29, 32, 35, 40, 48, 58,
    26, 27, 29, 34, 38, 46, 56, 69, 27, 29, 35, 38, 46, 56, 69, 83,
];

const QUANT_MAT_8X8NONINTRA: [u8; 64] = [
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
];

pub fn test_nvdec_mpeg2(main: &Main) -> Result<()> {
    let cfg_gem = main.drm.gem_create(0x1000)?;
    let mut cfg_map = cfg_gem.map(0x1000)?;
    {
        let b: &mut nvdec::nvdec_mpeg2_pic_s = bytemuck::from_bytes_mut(
            &mut cfg_map[0..std::mem::size_of::<nvdec::nvdec_mpeg2_pic_s>()],
        );
        b.stream_len = SLICE_DATA.len() as u32 + TERMINATION_SEQUENCE.len() as u32;
        b.slice_count = 2;
        b.FrameHeight = 16;
        b.FrameWidth = 16;
        b.picture_structure = 3;
        b.picture_coding_type = 1;
        b.intra_dc_precision = 0;
        b.frame_pred_frame_dct = 1;
        b.concealment_motion_vectors = 0;
        b.intra_vlc_format = 0;
        b.f_code = [0xf, 0xf, 0xf, 0xf];
        b.PicWidthInMbs = 1;
        b.FrameHeightInMbs = 1;
        b.pitch_luma = 16;
        b.pitch_chroma = 16;
        b.alternate_scan = 0;
        b.secondfield = 0;
        b.q_scale_type = 0;
        b.top_field_first = 0;
        b.quant_mat_8x8intra = QUANT_MAT_8X8INTRA;
        b.quant_mat_8x8nonintra = QUANT_MAT_8X8NONINTRA;
    }

    let slicedata_gem = main.drm.gem_create(0x1000)?;
    let mut slicedata_map = slicedata_gem.map(0x1000)?;
    slicedata_map[0..SLICE_DATA.len()].copy_from_slice(SLICE_DATA);
    slicedata_map[SLICE_DATA.len()..SLICE_DATA.len() + TERMINATION_SEQUENCE.len()]
        .copy_from_slice(TERMINATION_SEQUENCE);

    let sliceoffset_gem = main.drm.gem_create(0x1000)?;
    let mut sliceoffset_map = sliceoffset_gem.map(0x1000)?;
    {
        let b: &mut [u32] = bytemuck::cast_slice_mut(&mut sliceoffset_map[..]);
        b[0] = 0;
        b[1] = SLICE_DATA.len() as u32;
    }

    let output_gem = main.drm.gem_create(0x1000)?;

    let status_gem = main.drm.gem_create(0x1000)?;

    submit_test(main, 0xf0, |mut ctx| {
        let cfg_m = main
            .drm
            .channel_map(&ctx.channel, &cfg_gem, 0x0, 0x1000, false)?;
        let slicedata_m = main
            .drm
            .channel_map(&ctx.channel, &slicedata_gem, 0x0, 0x1000, false)?;
        let sliceoffset_m =
            main.drm
                .channel_map(&ctx.channel, &sliceoffset_gem, 0x0, 0x1000, false)?;
        let output_m = main
            .drm
            .channel_map(&ctx.channel, &output_gem, 0x0, 0x1000, true)?;
        let status_m = main.drm.channel_map(&ctx.channel, &status_gem, 0x0, 0x1000, true)?;

        const SET_APPLICATION_ID: u32 = 0x200;
        const APPLICATION_ID_MPEG12: u32 = 0x1;
        const SET_CONTROL_PARAMS: u32 = 0x400;
        const CODEC_TYPE_MPEG2: u32 = 1 << 0;
        const GPTIMER_ON: u32 = 1 << 4;
        const SET_DRV_PIC_SETUP_OFFSET: u32 = 0x404;
        const SET_IN_BUF_BASE_OFFSET: u32 = 0x408;
        const SET_SLICE_OFFSETS_BUF_OFFSET: u32 = 0x410;
        const SET_PICTURE_LUMA_OFFSET0: u32 = 0x430;
        const SET_PICTURE_CHROMA_OFFSET0: u32 = 0x474;
        const EXECUTE: u32 = 0x300;
        const AWAKEN_ENABLE: u32 = 1 << 8;

        ctx.push(&[0x1_010_0002, SET_APPLICATION_ID >> 2, APPLICATION_ID_MPEG12]);
        ctx.push(&[
            0x1_010_0002,
            SET_CONTROL_PARAMS >> 2,
            CODEC_TYPE_MPEG2 | GPTIMER_ON,
        ]);
        ctx.push(&[0x1_010_0002, SET_DRV_PIC_SETUP_OFFSET >> 2]);
        ctx.push_buf(&cfg_m);
        ctx.push(&[0x1_010_0002, SET_IN_BUF_BASE_OFFSET >> 2]);
        ctx.push_buf(&slicedata_m);
        ctx.push(&[0x1_010_0002, SET_SLICE_OFFSETS_BUF_OFFSET >> 2]);
        ctx.push_buf(&sliceoffset_m);
        ctx.push(&[0x1_010_0002, SET_PICTURE_LUMA_OFFSET0 >> 2]);
        ctx.push_buf_offset(&output_m, 0);
        ctx.push(&[0x1_010_0002, SET_PICTURE_CHROMA_OFFSET0 >> 2]);
        ctx.push_buf_offset(&output_m, 16 * 16);
        ctx.push(&[0x1_010_0002, 0x424>>2]);
        ctx.push_buf(&status_m);
        ctx.push(&[0x1_010_0002, EXECUTE >> 2, AWAKEN_ENABLE]);
        ctx.push_syncpt_incr(1);

        ctx.submit(main)?;

        main.drm
            .wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_secs(1))?;

        Ok(())
    })?;

    let output_map = output_gem.map(0x1000)?;
    check_eq!(output_map[0], 0x51, "expected 0x51 but got {left}");

    let status_map = status_gem.map(0x1000)?;
    {
        let b: &[u32] = bytemuck::cast_slice(&status_map[..]);
        println!("status MbOk={} MbErr={} Err={} SliceHdrErr={}", b[0], b[1], b[3], b[13]);
    }

    Ok(())
}
