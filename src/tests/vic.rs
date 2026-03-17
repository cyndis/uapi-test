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

use anyhow::Result;
use std::time::Duration;

use crate::{
    tests::{submit_test, SubmitTestCtx},
    uapi::Mapping,
    Main,
};

/// VIC 4.x config struct size (used in SET_CONTROL_PARAMS).
const VIC4_CONFIG_STRUCT_SIZE: u32 = 2960;

/// Config buffer size: VIC 5.x CompositorConfigStruct is ~5376 bytes,
/// use 0x2000 (8192) for sufficient room.
const VIC5_CONFIG_BUF_SIZE: u64 = 0x2000;

/// VIC host1x class ID.
const VIC_CLASS: u32 = 0x5d;

fn cfg_buf_size(is_vic5: bool) -> u64 {
    if is_vic5 { VIC5_CONFIG_BUF_SIZE } else { 0x1000 }
}

/// Fill config buffer with VIC 4.x (B0B6/B1B6/C8B6) clear configuration.
/// Config struct layout: PipeConfig(16B) + OutputConfig(16B) + OutputSurfaceConfig(16B) + ...
///
/// OutputConfig is two packed u64 bitfield words (see engine-hdr/vic.h):
///   Word 0: AlphaFillMode(3), AlphaFillSlot(3), BackgroundAlpha(10), BackgroundR(10),
///           BackgroundG(10), BackgroundB(10), ...
///   Word 1: TargetRectLeft(14), reserved(2), TargetRectRight(14), reserved(2),
///           TargetRectTop(14), reserved(2), TargetRectBottom(14), reserved(2)
///
/// OutputSurfaceConfig is two packed u64 bitfield words:
///   Word 0: OutPixelFormat(7), ..., OutSurfaceWidth(14), OutSurfaceHeight(14), ...
///   Word 1: OutLumaWidth(14), OutLumaHeight(14), ..., OutChromaWidth(14), OutChromaHeight(14), ...
fn fill_vic4_clear_config(cfg_map: &mut [u8]) {
    // OutputConfig at offset 16
    // Word 0: BackgroundAlpha=1023 (bits 6-15), BackgroundG=1023 (bits 26-35)
    let word0: u64 = (1023 << 6) | (1023 << 26);
    cfg_map[16..24].copy_from_slice(&word0.to_le_bytes());

    // Word 1: TargetRectRight=31 (bits 80-93, i.e. offset 16 in word), TargetRectBottom=31 (bits 112-125, i.e. offset 48)
    let word1: u64 = (31 << 16) | (31 << 48);
    cfg_map[24..32].copy_from_slice(&word1.to_le_bytes());

    // OutputSurfaceConfig at offset 32
    // Word 0: OutPixelFormat=32 (bits 0-6), OutSurfaceWidth=31 (bits 32-45), OutSurfaceHeight=31 (bits 46-59)
    let word0: u64 = 32 | (31 << 32) | (31 << 46);
    cfg_map[32..40].copy_from_slice(&word0.to_le_bytes());

    // Word 1: OutLumaWidth=255 (bits 0-13), OutLumaHeight=31 (bits 14-27),
    //         OutChromaWidth=16383 (bits 32-45), OutChromaHeight=16383 (bits 46-59)
    let word1: u64 = 255 | (31 << 14) | (16383 << 32) | (16383 << 46);
    cfg_map[40..48].copy_from_slice(&word1.to_le_bytes());
}

/// Fill config buffer with VIC 5.x (CEB6) clear configuration.
/// Config struct layout: OutConfig(448B) + ClearRect[8](64B) + SlotConfig[8](4096B) + ...
/// OutConfig fields written directly at byte offsets matching nvceb6::OutConfig in vic_structs.h.
fn fill_vic5_clear_config(cfg_map: &mut [u8]) {
    // Dword 0 (offset 0): control bits
    // Bit 7: OutGInputU16 = 1 (background colors in U0.16 format)
    let control0: u32 = 1 << 7;
    cfg_map[0..4].copy_from_slice(&control0.to_le_bytes());

    // Dword 1 (offset 4): pixel format and surface config
    // Bits 9:0: OutPixelFormat = 32 (T_A8R8G8B8)
    let format: u32 = 32;
    cfg_map[4..8].copy_from_slice(&format.to_le_bytes());

    // Target rectangle (uint16_t fields at offsets 8-15)
    cfg_map[8..10].copy_from_slice(&0u16.to_le_bytes());   // TargetRectLeft
    cfg_map[10..12].copy_from_slice(&31u16.to_le_bytes());  // TargetRectRight
    cfg_map[12..14].copy_from_slice(&0u16.to_le_bytes());   // TargetRectTop
    cfg_map[14..16].copy_from_slice(&31u16.to_le_bytes());  // TargetRectBottom

    // Output image dimensions, minus-1 encoded (uint16_t fields at offsets 16-27)
    cfg_map[16..18].copy_from_slice(&31u16.to_le_bytes());    // OutImgWidth
    cfg_map[18..20].copy_from_slice(&31u16.to_le_bytes());    // OutImgHeight
    cfg_map[20..22].copy_from_slice(&255u16.to_le_bytes());   // OutLumaSfcWPad
    cfg_map[22..24].copy_from_slice(&31u16.to_le_bytes());    // OutLumaSfcHPad
    cfg_map[24..26].copy_from_slice(&16383u16.to_le_bytes()); // OutChromaSfcWPad
    cfg_map[26..28].copy_from_slice(&16383u16.to_le_bytes()); // OutChromaSfcHPad

    // Alpha fill mode (offset 28): AlphaFillMode = 0 (OPAQUE), all alpha set to 1.0
    // Already zero from GEM allocation.

    // Background colors in U0.16 format (uint16_t fields at offsets 32-39)
    cfg_map[32..34].copy_from_slice(&0xFFFFu16.to_le_bytes()); // BackgroundA (1.0)
    cfg_map[34..36].copy_from_slice(&0x0000u16.to_le_bytes()); // BackgroundR (0.0)
    cfg_map[36..38].copy_from_slice(&0xFFFFu16.to_le_bytes()); // BackgroundG (1.0)
    cfg_map[38..40].copy_from_slice(&0x0000u16.to_le_bytes()); // BackgroundB (0.0)
}

fn fill_clear_config(is_vic5: bool, cfg_map: &mut [u8]) {
    if is_vic5 {
        fill_vic5_clear_config(cfg_map);
    } else {
        fill_vic4_clear_config(cfg_map);
    }
}

/// Push the VIC clear method sequence: set application ID, control params,
/// config/surface/filter buffers, and execute.
fn push_vic_clear_methods(
    ctx: &mut SubmitTestCtx,
    is_vic5: bool,
    cfg_m: &Mapping,
    surf_m: &Mapping,
    filt_m: &Mapping,
) {
    // SET_APPLICATION_ID: VIC 4.x = 1 (compose), VIC 5.x = 0
    let app_id = if is_vic5 { 0 } else { 1 };
    ctx.push_method(0x200, app_id);

    // SET_CONTROL_PARAMS: VIC 4.x encodes config struct size, VIC 5.x = 0
    let control_params = if is_vic5 { 0 } else { (VIC4_CONFIG_STRUCT_SIZE / 16) << 16 };
    ctx.push_method(0x704, control_params);

    ctx.push_method_buf(0x708, cfg_m);
    ctx.push_method_buf(0x720, surf_m);
    ctx.push_method_buf(0x70c, filt_m);
    ctx.push_method(0x300, 1 << 8);
}

/// Verify the output surface contains the expected green clear color.
fn check_clear_result(surf_gem: &crate::uapi::Gem) -> Result<()> {
    let surf_map = surf_gem.map(0x1000)?;
    let pixel = u32::from_le_bytes(surf_map[0..4].try_into().unwrap());
    check_eq!(
        pixel,
        0xff00ff00,
        "Expected 0xff00ff00, got 0x{left:08x}"
    );
    Ok(())
}

pub fn channel_submit_vic_clear(main: &Main) -> Result<()> {
    let is_vic5 = main.soc.is_vic5();
    let cfg_buf_size = cfg_buf_size(is_vic5);

    let cfg_gem = main.drm.gem_create(cfg_buf_size)?;
    let mut cfg_map = cfg_gem.map(cfg_buf_size as usize)?;
    fill_clear_config(is_vic5, &mut cfg_map);

    let surf_gem = main.drm.gem_create(32768)?;
    let filt_gem = main.drm.gem_create(0x3000)?;

    submit_test(main, VIC_CLASS, |mut ctx| {
        let cfg_m = main
            .drm
            .channel_map(&ctx.channel, &cfg_gem, 0x0, cfg_buf_size, false)?;
        let surf_m = main
            .drm
            .channel_map(&ctx.channel, &surf_gem, 0x0, 32768, true)?;
        let filt_m = main
            .drm
            .channel_map(&ctx.channel, &filt_gem, 0x0, 0x3000, false)?;

        push_vic_clear_methods(&mut ctx, is_vic5, &cfg_m, &surf_m, &filt_m);
        ctx.push_syncpt_incr(1);

        ctx.submit(main)?;

        main.drm
            .wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_secs(1))?;

        Ok(())
    })?;

    check_clear_result(&surf_gem)
}

pub fn channel_buf_refcounting(main: &Main) -> Result<()> {
    let is_vic5 = main.soc.is_vic5();
    let cfg_buf_size = cfg_buf_size(is_vic5);

    let syncpt = main.drm.allocate_syncpoint()?;
    let syncpt_id = syncpt.id();
    let value = main.drm.read_syncpoint(syncpt_id)?;

    let cfg_gem = main.drm.gem_create(cfg_buf_size)?;
    let mut cfg_map = cfg_gem.map(cfg_buf_size as usize)?;
    fill_clear_config(is_vic5, &mut cfg_map);

    let surf_gem = main.drm.gem_create(32768)?;
    let filt_gem = main.drm.gem_create(0x3000)?;

    submit_test(main, VIC_CLASS, |mut ctx| {
        let cfg_m = main
            .drm
            .channel_map(&ctx.channel, &cfg_gem, 0x0, cfg_buf_size, false)?;
        let surf_m = main
            .drm
            .channel_map(&ctx.channel, &surf_gem, 0x0, 32768, true)?;
        let filt_m = main
            .drm
            .channel_map(&ctx.channel, &filt_gem, 0x0, 0x3000, false)?;

        ctx.set_increments(ctx.syncpt.id(), 2);
        ctx.push_incr_syncpt(ctx.syncpt_id, 0);
        ctx.push_wait_syncpt(syncpt_id, value.wrapping_add(1));

        push_vic_clear_methods(&mut ctx, is_vic5, &cfg_m, &surf_m, &filt_m);
        ctx.push_incr_syncpt(ctx.syncpt_id, 1);

        ctx.submit(main)?;

        main.drm.wait_syncpoint(
            ctx.syncpt_id,
            ctx.args.syncpt.value - 1,
            Duration::from_millis(1000),
        )?;

        drop(cfg_m);
        drop(filt_m);
        drop(surf_m);
        drop(cfg_gem);
        drop(filt_gem);

        syncpt.increment(1)?;

        main.drm.wait_syncpoint(
            ctx.syncpt_id,
            ctx.args.syncpt.value,
            Duration::from_millis(1000),
        )?;

        Ok(())
    })?;

    check_clear_result(&surf_gem)
}
