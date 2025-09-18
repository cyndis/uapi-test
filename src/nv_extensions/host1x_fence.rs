pub mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    #![allow(deref_nullptr)]

    include!("../generated-bindings/host1x-fence_bindings.rs");
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
