#include <linux/types.h>
#include <linux/ioctl.h>

#define DRM_IOCTL_BASE			'd'
#define DRM_IO(nr)			_IO(DRM_IOCTL_BASE,nr)
#define DRM_IOR(nr,type)		_IOR(DRM_IOCTL_BASE,nr,type)
#define DRM_IOW(nr,type)		_IOW(DRM_IOCTL_BASE,nr,type)
#define DRM_IOWR(nr,type)		_IOWR(DRM_IOCTL_BASE,nr,type)
#define DRM_COMMAND_BASE                0x40

#include "kernel-hdr/tegra_drm.h"

#define MAKE_CONST(req_name) const __u64 MK_##req_name = req_name;

MAKE_CONST(DRM_IOCTL_TEGRA_SYNCPOINT_ALLOCATE)
MAKE_CONST(DRM_IOCTL_TEGRA_SYNCPOINT_FREE)
MAKE_CONST(DRM_IOCTL_TEGRA_SYNCPOINT_WAIT)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_OPEN)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_CLOSE)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_MAP)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_UNMAP)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_SUBMIT)
MAKE_CONST(DRM_IOCTL_TEGRA_GEM_CREATE)
MAKE_CONST(DRM_IOCTL_TEGRA_GEM_MMAP)
MAKE_CONST(DRM_IOCTL_TEGRA_SYNCPT_INCR)

struct drm_gem_close {
        __u32 handle;
        __u32 pad;
};

struct drm_syncobj_create {
        __u32 handle;
#define DRM_SYNCOBJ_CREATE_SIGNALED (1 << 0)
        __u32 flags;
};

struct drm_syncobj_destroy {
        __u32 handle;
        __u32 pad;
};

#define DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL (1 << 0)
#define DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT (1 << 1)
#define DRM_SYNCOBJ_WAIT_FLAGS_WAIT_AVAILABLE (1 << 2)
struct drm_syncobj_wait {
        __u64 handles;
        __s64 timeout_nsec;
        __u32 count_handles;
        __u32 flags;
        __u32 first_signaled;
        __u32 pad;
};

struct drm_prime_handle {
        __u32 handle;
        __u32 flags;
        __s32 fd;
};

#define DRM_IOCTL_GEM_CLOSE             DRM_IOW (0x09, struct drm_gem_close)
#define DRM_IOCTL_PRIME_HANDLE_TO_FD    DRM_IOWR(0x2d, struct drm_prime_handle)
#define DRM_IOCTL_PRIME_FD_TO_HANDLE    DRM_IOWR(0x2e, struct drm_prime_handle)
#define DRM_IOCTL_SYNCOBJ_CREATE        DRM_IOWR(0xBF, struct drm_syncobj_create)
#define DRM_IOCTL_SYNCOBJ_DESTROY       DRM_IOWR(0xC0, struct drm_syncobj_destroy)
#define DRM_IOCTL_SYNCOBJ_WAIT          DRM_IOWR(0xC3, struct drm_syncobj_wait)

MAKE_CONST(DRM_IOCTL_GEM_CLOSE)
MAKE_CONST(DRM_IOCTL_PRIME_HANDLE_TO_FD)
MAKE_CONST(DRM_IOCTL_PRIME_FD_TO_HANDLE)
MAKE_CONST(DRM_IOCTL_SYNCOBJ_CREATE)
MAKE_CONST(DRM_IOCTL_SYNCOBJ_DESTROY)
MAKE_CONST(DRM_IOCTL_SYNCOBJ_WAIT)

#define DRM_VGEM_FENCE_ATTACH   0x1
#define DRM_VGEM_FENCE_SIGNAL   0x2

#define DRM_IOCTL_VGEM_FENCE_ATTACH     DRM_IOWR( DRM_COMMAND_BASE + DRM_VGEM_FENCE_ATTACH, struct drm_vgem_fence_attach)
#define DRM_IOCTL_VGEM_FENCE_SIGNAL     DRM_IOW( DRM_COMMAND_BASE + DRM_VGEM_FENCE_SIGNAL, struct drm_vgem_fence_signal)

struct drm_vgem_fence_attach {
        __u32 handle;
        __u32 flags;
#define VGEM_FENCE_WRITE        0x1
        __u32 out_fence;
        __u32 pad;
};

struct drm_vgem_fence_signal {
        __u32 fence;
        __u32 flags;
};

MAKE_CONST(DRM_IOCTL_VGEM_FENCE_ATTACH)
MAKE_CONST(DRM_IOCTL_VGEM_FENCE_SIGNAL)
