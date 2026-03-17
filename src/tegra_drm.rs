// SPDX-License-Identifier: MIT
//
// Manually written Tegra DRM UAPI definitions.
// Matches the kernel's include/uapi/drm/tegra_drm.h and include/uapi/drm/drm.h.

#![allow(non_camel_case_types)]

use std::mem::size_of;

// ioctl number encoding (same on ARM32 and ARM64)
const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u64 {
    ((dir << 30) | ((size as u32) << 16) | (ty << 8) | nr) as u64
}
const fn iowr(ty: u32, nr: u32, size: usize) -> u64 { ioc(3, ty, nr, size) }
const fn iow(ty: u32, nr: u32, size: usize) -> u64 { ioc(1, ty, nr, size) }

const DRM: u32 = b'd' as u32;
const DRM_COMMAND_BASE: u32 = 0x40;

// Tegra-specific DRM command offsets
const TEGRA_GEM_CREATE: u32 = 0x00;
const TEGRA_GEM_MMAP: u32 = 0x01;
const TEGRA_SYNCPT_INCR: u32 = 0x03;
const TEGRA_CHANNEL_OPEN: u32 = 0x10;
const TEGRA_CHANNEL_CLOSE: u32 = 0x11;
const TEGRA_CHANNEL_MAP: u32 = 0x12;
const TEGRA_CHANNEL_UNMAP: u32 = 0x13;
const TEGRA_CHANNEL_SUBMIT: u32 = 0x14;
const TEGRA_SYNCPOINT_ALLOCATE: u32 = 0x20;
const TEGRA_SYNCPOINT_FREE: u32 = 0x21;
const TEGRA_SYNCPOINT_WAIT: u32 = 0x22;

// DRM generic command offsets
const DRM_GEM_CLOSE: u32 = 0x09;
const DRM_PRIME_HANDLE_TO_FD: u32 = 0x2D;

// --- IOCTL numbers ---

pub const DRM_IOCTL_TEGRA_GEM_CREATE: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_GEM_CREATE, size_of::<drm_tegra_gem_create>());
pub const DRM_IOCTL_TEGRA_GEM_MMAP: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_GEM_MMAP, size_of::<drm_tegra_gem_mmap>());
pub const DRM_IOCTL_TEGRA_SYNCPT_INCR: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_SYNCPT_INCR, size_of::<drm_tegra_syncpt_incr>());
pub const DRM_IOCTL_TEGRA_CHANNEL_OPEN: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_CHANNEL_OPEN, size_of::<drm_tegra_channel_open>());
pub const DRM_IOCTL_TEGRA_CHANNEL_CLOSE: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_CHANNEL_CLOSE, size_of::<drm_tegra_channel_close>());
pub const DRM_IOCTL_TEGRA_CHANNEL_MAP: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_CHANNEL_MAP, size_of::<drm_tegra_channel_map>());
pub const DRM_IOCTL_TEGRA_CHANNEL_UNMAP: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_CHANNEL_UNMAP, size_of::<drm_tegra_channel_unmap>());
pub const DRM_IOCTL_TEGRA_CHANNEL_SUBMIT: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_CHANNEL_SUBMIT, size_of::<drm_tegra_channel_submit>());
pub const DRM_IOCTL_TEGRA_SYNCPOINT_ALLOCATE: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_SYNCPOINT_ALLOCATE, size_of::<drm_tegra_syncpoint_allocate>());
pub const DRM_IOCTL_TEGRA_SYNCPOINT_FREE: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_SYNCPOINT_FREE, size_of::<drm_tegra_syncpoint_free>());
pub const DRM_IOCTL_TEGRA_SYNCPOINT_WAIT: u64 =
    iowr(DRM, DRM_COMMAND_BASE + TEGRA_SYNCPOINT_WAIT, size_of::<drm_tegra_syncpoint_wait>());
pub const DRM_IOCTL_GEM_CLOSE: u64 =
    iow(DRM, DRM_GEM_CLOSE, size_of::<drm_gem_close>());
pub const DRM_IOCTL_PRIME_HANDLE_TO_FD: u64 =
    iowr(DRM, DRM_PRIME_HANDLE_TO_FD, size_of::<drm_prime_handle>());

// --- Flags ---

pub const DRM_TEGRA_CHANNEL_MAP_READ: u32 = 1;
pub const DRM_TEGRA_CHANNEL_MAP_WRITE: u32 = 2;

pub const DRM_TEGRA_SUBMIT_CMD_GATHER_UPTR: u32 = 0;
pub const DRM_TEGRA_SUBMIT_CMD_WAIT_SYNCPT: u32 = 1;

// --- Structures ---

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_gem_create {
    pub size: u64,
    pub flags: u32,
    pub handle: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_gem_mmap {
    pub handle: u32,
    pub pad: u32,
    pub offset: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_syncpt_incr {
    pub id: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_channel_open {
    pub host1x_class: u32,
    pub flags: u32,
    pub context: u32,
    pub version: u32,
    pub capabilities: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_channel_close {
    pub context: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_channel_map {
    pub context: u32,
    pub handle: u32,
    pub flags: u32,
    pub mapping: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_channel_unmap {
    pub context: u32,
    pub mapping: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_submit_buf_reloc {
    pub target_offset: u64,
    pub gather_offset_words: u32,
    pub shift: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_submit_buf {
    pub mapping: u32,
    pub flags: u32,
    pub reloc: drm_tegra_submit_buf_reloc,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_submit_cmd_gather_uptr {
    pub words: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_submit_cmd_wait_syncpt {
    pub id: u32,
    pub value: u32,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_tegra_submit_cmd_data {
    pub gather_uptr: drm_tegra_submit_cmd_gather_uptr,
    pub wait_syncpt: drm_tegra_submit_cmd_wait_syncpt,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_tegra_submit_cmd {
    pub type_: u32,
    pub flags: u32,
    pub data: drm_tegra_submit_cmd_data,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_submit_syncpt {
    pub id: u32,
    pub flags: u32,
    pub increments: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_channel_submit {
    pub context: u32,
    pub num_bufs: u32,
    pub num_cmds: u32,
    pub gather_data_words: u32,
    pub bufs_ptr: u64,
    pub cmds_ptr: u64,
    pub gather_data_ptr: u64,
    pub syncobj_in: u32,
    pub syncobj_out: u32,
    pub syncpt: drm_tegra_submit_syncpt,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_syncpoint_allocate {
    pub id: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_syncpoint_free {
    pub id: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_tegra_syncpoint_wait {
    pub timeout_ns: i64,
    pub id: u32,
    pub threshold: u32,
    pub value: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_gem_close {
    pub handle: u32,
    pub pad: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_prime_handle {
    pub handle: u32,
    pub flags: u32,
    pub fd: i32,
}

// --- Layout verification ---

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn layout_check() {
        assert_eq!(size_of::<drm_tegra_gem_create>(), 16);
        assert_eq!(size_of::<drm_tegra_gem_mmap>(), 16);
        assert_eq!(size_of::<drm_tegra_syncpt_incr>(), 8);
        assert_eq!(size_of::<drm_tegra_channel_open>(), 24);
        assert_eq!(size_of::<drm_tegra_channel_close>(), 8);
        assert_eq!(size_of::<drm_tegra_channel_map>(), 16);
        assert_eq!(size_of::<drm_tegra_channel_unmap>(), 8);
        assert_eq!(size_of::<drm_tegra_submit_buf>(), 24);
        assert_eq!(size_of::<drm_tegra_submit_cmd>(), 24);
        assert_eq!(size_of::<drm_tegra_submit_syncpt>(), 16);
        assert_eq!(size_of::<drm_tegra_channel_submit>(), 64);
        assert_eq!(size_of::<drm_tegra_syncpoint_allocate>(), 8);
        assert_eq!(size_of::<drm_tegra_syncpoint_free>(), 8);
        assert_eq!(size_of::<drm_tegra_syncpoint_wait>(), 24);
        assert_eq!(size_of::<drm_gem_close>(), 8);
        assert_eq!(size_of::<drm_prime_handle>(), 12);
    }

    #[test]
    fn ioctl_values() {
        assert_eq!(DRM_IOCTL_TEGRA_SYNCPOINT_ALLOCATE, 3221775456);
        assert_eq!(DRM_IOCTL_TEGRA_SYNCPOINT_FREE, 3221775457);
        assert_eq!(DRM_IOCTL_TEGRA_SYNCPOINT_WAIT, 3222824034);
        assert_eq!(DRM_IOCTL_TEGRA_CHANNEL_OPEN, 3222824016);
        assert_eq!(DRM_IOCTL_TEGRA_CHANNEL_CLOSE, 3221775441);
        assert_eq!(DRM_IOCTL_TEGRA_CHANNEL_MAP, 3222299730);
        assert_eq!(DRM_IOCTL_TEGRA_CHANNEL_UNMAP, 3221775443);
        assert_eq!(DRM_IOCTL_TEGRA_CHANNEL_SUBMIT, 3225445460);
        assert_eq!(DRM_IOCTL_TEGRA_GEM_CREATE, 3222299712);
        assert_eq!(DRM_IOCTL_TEGRA_GEM_MMAP, 3222299713);
        assert_eq!(DRM_IOCTL_TEGRA_SYNCPT_INCR, 3221775427);
        assert_eq!(DRM_IOCTL_GEM_CLOSE, 1074291721);
        assert_eq!(DRM_IOCTL_PRIME_HANDLE_TO_FD, 3222037549);
    }
}
