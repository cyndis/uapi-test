// SPDX-License-Identifier: MIT

pub mod sys {
    #![allow(non_camel_case_types)]

    use std::mem::size_of;

    const fn ioc(dir: u32, ty: u32, nr: u32, size: usize) -> u64 {
        ((dir << 30) | ((size as u32) << 16) | (ty << 8) | nr) as u64
    }
    const fn iowr(ty: u32, nr: u32, size: usize) -> u64 { ioc(3, ty, nr, size) }

    const HOST1X: u32 = b'H' as u32;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct host1x_create_fence {
        pub id: u32,
        pub threshold: u32,
        pub fence_fd: i32,
        pub reserved: [u32; 1],
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct host1x_fence_extract_fence {
        pub id: u32,
        pub threshold: u32,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct host1x_fence_extract {
        pub fence_fd: i32,
        pub num_fences: u32,
        pub fences_ptr: u64,
        pub reserved: [u32; 2],
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct host1x_create_pollfd {
        pub fd: i32,
        pub reserved: u32,
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct host1x_trigger_pollfd {
        pub fd: i32,
        pub id: u32,
        pub threshold: u32,
        pub reserved: u32,
    }

    pub const HOST1X_IOCTL_CREATE_FENCE: u64 =
        iowr(HOST1X, 0x02, size_of::<host1x_create_fence>());
    pub const HOST1X_IOCTL_FENCE_EXTRACT: u64 =
        iowr(HOST1X, 0x05, size_of::<host1x_fence_extract>());
    pub const HOST1X_IOCTL_CREATE_POLLFD: u64 =
        iowr(HOST1X, 0x10, size_of::<host1x_create_pollfd>());
    pub const HOST1X_IOCTL_TRIGGER_POLLFD: u64 =
        iowr(HOST1X, 0x11, size_of::<host1x_trigger_pollfd>());

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

        #[test]
        fn ioctl_values() {
            assert_eq!(HOST1X_IOCTL_CREATE_FENCE, 3222296578);
            assert_eq!(HOST1X_IOCTL_FENCE_EXTRACT, 3222820869);
            assert_eq!(HOST1X_IOCTL_CREATE_POLLFD, 3221772304);
            assert_eq!(HOST1X_IOCTL_TRIGGER_POLLFD, 3222296593);
        }
    }
}

use sys::*;

use std::os::unix::io::AsRawFd;

use crate::uapi::ioctl;
use crate::{EResult, Errno, IocResult};

pub struct Host1xFenceInterface {
    pub fd: std::fs::File,
}

macro_rules! define_raw {
    ($($name:ident : $ioctl:ident -> $ty:ty),* $(,)?) => {
        $(
        #[allow(unused)]
        pub fn $name(&self, mut args: $ty) -> IocResult<$ty> {
            unsafe {
                let err = ioctl(self.fd.as_raw_fd(), $ioctl, &mut args);
                if err != 0 {
                    Err(Errno::get())?;
                }

                Ok(args)
            }
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
        create_fence_raw: HOST1X_IOCTL_CREATE_FENCE -> host1x_create_fence,
        fence_extract_raw: HOST1X_IOCTL_FENCE_EXTRACT -> host1x_fence_extract,
        create_pollfd_raw: HOST1X_IOCTL_CREATE_POLLFD -> host1x_create_pollfd,
        trigger_pollfd_raw: HOST1X_IOCTL_TRIGGER_POLLFD -> host1x_trigger_pollfd,
    );
}
