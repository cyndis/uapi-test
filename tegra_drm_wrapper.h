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

MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_OPEN)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_CLOSE)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_MAP)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_UNMAP)
MAKE_CONST(DRM_IOCTL_TEGRA_CHANNEL_SUBMIT)
MAKE_CONST(DRM_IOCTL_TEGRA_GEM_CREATE)
MAKE_CONST(DRM_IOCTL_TEGRA_GEM_MMAP)

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

#define DRM_IOCTL_GEM_CLOSE             DRM_IOW (0x09, struct drm_gem_close)
#define DRM_IOCTL_SYNCOBJ_CREATE        DRM_IOWR(0xBF, struct drm_syncobj_create)
#define DRM_IOCTL_SYNCOBJ_DESTROY       DRM_IOWR(0xC0, struct drm_syncobj_destroy)
#define DRM_IOCTL_SYNCOBJ_WAIT          DRM_IOWR(0xC3, struct drm_syncobj_wait)

MAKE_CONST(DRM_IOCTL_GEM_CLOSE)
MAKE_CONST(DRM_IOCTL_SYNCOBJ_CREATE)
MAKE_CONST(DRM_IOCTL_SYNCOBJ_DESTROY)
MAKE_CONST(DRM_IOCTL_SYNCOBJ_WAIT)
