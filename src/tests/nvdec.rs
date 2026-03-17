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

use crate::{tests::submit_test, Main};

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

/// Fill the nvdec_mpeg2_pic_s config buffer at byte offsets.
/// Field offsets derived from engine-hdr/nvdec.h (see struct _nvdec_mpeg2_pic_s).
/// Total struct size: 344 bytes.
fn fill_nvdec_mpeg2_config(cfg_map: &mut [u8]) {
    let stream_len = SLICE_DATA.len() as u32 + TERMINATION_SEQUENCE.len() as u32;

    // stream_len: offset 72 (u32)
    cfg_map[72..76].copy_from_slice(&stream_len.to_le_bytes());
    // slice_count: offset 76 (u32)
    cfg_map[76..80].copy_from_slice(&2u32.to_le_bytes());
    // FrameWidth: offset 84 (u16)
    cfg_map[84..86].copy_from_slice(&16u16.to_le_bytes());
    // FrameHeight: offset 86 (u16)
    cfg_map[86..88].copy_from_slice(&16u16.to_le_bytes());
    // picture_structure: offset 88 (u8)
    cfg_map[88] = 3;
    // picture_coding_type: offset 89 (u8)
    cfg_map[89] = 1;
    // intra_dc_precision: offset 90 (u8) = 0 (default)
    // frame_pred_frame_dct: offset 91 (i8)
    cfg_map[91] = 1;
    // concealment_motion_vectors: offset 92 (i8) = 0 (default)
    // intra_vlc_format: offset 93 (i8) = 0 (default)
    // f_code[4]: offset 96 (4 x i8)
    cfg_map[96..100].copy_from_slice(&[0xf, 0xf, 0xf, 0xf]);
    // PicWidthInMbs: offset 100 (u16)
    cfg_map[100..102].copy_from_slice(&1u16.to_le_bytes());
    // FrameHeightInMbs: offset 102 (u16)
    cfg_map[102..104].copy_from_slice(&1u16.to_le_bytes());
    // pitch_luma: offset 104 (u32)
    cfg_map[104..108].copy_from_slice(&16u32.to_le_bytes());
    // pitch_chroma: offset 108 (u32)
    cfg_map[108..112].copy_from_slice(&16u32.to_le_bytes());
    // alternate_scan: offset 142 (u16) = 0 (default)
    // secondfield: offset 144 (u16) = 0 (default)
    // q_scale_type: offset 152 (u32) = 0 (default)
    // top_field_first: offset 156 (u32) = 0 (default)
    // quant_mat_8x8intra[64]: offset 168
    cfg_map[168..232].copy_from_slice(&QUANT_MAT_8X8INTRA);
    // quant_mat_8x8nonintra[64]: offset 232
    cfg_map[232..296].copy_from_slice(&QUANT_MAT_8X8NONINTRA);
}

fn read_u32_le(buf: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
}

pub fn nvdec_mpeg2(main: &Main) -> Result<()> {
    let cfg_gem = main.drm.gem_create(0x1000)?;
    let mut cfg_map = cfg_gem.map(0x1000)?;
    fill_nvdec_mpeg2_config(&mut cfg_map);

    let slicedata_gem = main.drm.gem_create(0x1000)?;
    let mut slicedata_map = slicedata_gem.map(0x1000)?;
    slicedata_map[0..SLICE_DATA.len()].copy_from_slice(SLICE_DATA);
    slicedata_map[SLICE_DATA.len()..SLICE_DATA.len() + TERMINATION_SEQUENCE.len()]
        .copy_from_slice(TERMINATION_SEQUENCE);

    let sliceoffset_gem = main.drm.gem_create(0x1000)?;
    let mut sliceoffset_map = sliceoffset_gem.map(0x1000)?;
    sliceoffset_map[0..4].copy_from_slice(&0u32.to_le_bytes());
    sliceoffset_map[4..8].copy_from_slice(&(SLICE_DATA.len() as u32).to_le_bytes());

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
    println!(
        "status MbOk={} MbErr={} Err={} SliceHdrErr={}",
        read_u32_le(&status_map, 0),
        read_u32_le(&status_map, 4),
        read_u32_le(&status_map, 12),
        read_u32_le(&status_map, 52),
    );

    Ok(())
}
