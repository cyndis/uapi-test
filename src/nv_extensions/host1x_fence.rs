// SPDX-License-Identifier: MIT

pub mod sys {
    #![allow(non_camel_case_types)]

    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Pod, Zeroable)]
    pub struct host1x_create_fence {
        pub id: u32,
        pub threshold: u32,
        pub fence_fd: i32,
        pub reserved: [u32; 1],
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Pod, Zeroable)]
    pub struct host1x_fence_extract_fence {
        pub id: u32,
        pub threshold: u32,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Pod, Zeroable)]
    pub struct host1x_fence_extract {
        pub fence_fd: i32,
        pub num_fences: u32,
        pub fences_ptr: u64,
        pub reserved: [u32; 2],
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Pod, Zeroable)]
    pub struct host1x_create_pollfd {
        pub fd: i32,
        pub reserved: u32,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Pod, Zeroable)]
    pub struct host1x_trigger_pollfd {
        pub fd: i32,
        pub id: u32,
        pub threshold: u32,
        pub reserved: u32,
    }

    nix::ioctl_readwrite!(ioctl_create_fence, b'H', 0x02, host1x_create_fence);
    nix::ioctl_readwrite!(ioctl_fence_extract, b'H', 0x05, host1x_fence_extract);
    nix::ioctl_readwrite!(ioctl_create_pollfd, b'H', 0x10, host1x_create_pollfd);
    nix::ioctl_readwrite!(ioctl_trigger_pollfd, b'H', 0x11, host1x_trigger_pollfd);

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::mem::size_of;

        #[test]
        fn layout_check() {
            assert_eq!(size_of::<host1x_create_fence>(), 16);
            assert_eq!(size_of::<host1x_fence_extract_fence>(), 8);
            assert_eq!(size_of::<host1x_fence_extract>(), 24);
            assert_eq!(size_of::<host1x_create_pollfd>(), 8);
            assert_eq!(size_of::<host1x_trigger_pollfd>(), 16);
        }
    }
}

use sys::*;

use std::os::unix::io::AsRawFd;

use crate::{EResult, IocResult};

pub struct Host1xFenceInterface {
    pub fd: std::fs::File,
}

macro_rules! define_raw {
    ($($name:ident : $ioc_fn:ident, $ty:ty);* $(;)?) => {
        $(
        #[allow(unused)]
        pub fn $name(&self, mut args: $ty) -> IocResult<$ty> {
            unsafe { $ioc_fn(self.fd.as_raw_fd(), &mut args) }?;
            Ok(args)
        }
        )*
    }
}

impl Host1xFenceInterface {
    pub fn open() -> EResult<Host1xFenceInterface> {
        let fd = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/host1x-fence")?;

        Ok(Host1xFenceInterface { fd })
    }

    define_raw!(
        create_fence_raw: ioctl_create_fence, host1x_create_fence;
        fence_extract_raw: ioctl_fence_extract, host1x_fence_extract;
        create_pollfd_raw: ioctl_create_pollfd, host1x_create_pollfd;
        trigger_pollfd_raw: ioctl_trigger_pollfd, host1x_trigger_pollfd;
    );
}
