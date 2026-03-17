use anyhow::{Context, Result};
use bytemuck::Zeroable;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

use crate::Main;

use crate::uapi_test;
use super::host1x_fence::{Host1xFenceInterface, sys};

uapi_test!("fence.create_extract_roundtrip", create_extract_fence_roundtrip);
pub fn create_extract_fence_roundtrip(_main: &Main) -> Result<()> {
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let create_fence = intf.create_fence_raw(sys::host1x_create_fence {
        id: 2,
        threshold: 3,
        ..Zeroable::zeroed()
    }).context("HOST1X_CREATE_FENCE failed")?;
    let fence_fd = unsafe { OwnedFd::from_raw_fd(create_fence.fence_fd) };

    let mut fence: sys::host1x_fence_extract_fence = Zeroable::zeroed();
    intf.fence_extract_raw(sys::host1x_fence_extract {
        fence_fd: fence_fd.as_raw_fd(),
        num_fences: 1,
        fences_ptr: &mut fence as *const _ as u64,
        ..Zeroable::zeroed()
    }).context("HOST1X_FENCE_EXTRACT failed")?;

    check_eq!(fence.id, 2, "expected 2 but got {left}");
    check_eq!(fence.threshold, 3, "expected 3 but got {left}");

    Ok(())
}

uapi_test!("fence.created_wait", created_fence_wait);
pub fn created_fence_wait(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_fence = intf.create_fence_raw(sys::host1x_create_fence {
        id: syncpoint.id(),
        threshold: value.wrapping_add(1),
        ..Zeroable::zeroed()
    }).context("HOST1X_CREATE_FENCE failed")?;
    let fence_fd = unsafe { OwnedFd::from_raw_fd(create_fence.fence_fd) };

    let mut pollfd = libc::pollfd {
        fd: fence_fd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    let err = unsafe { libc::poll(&mut pollfd as _, 1, 10) };
    check_eq!(err, 0, "expected 0 completed polls but got {left}");

    syncpoint.increment(1).context("Failed to increment syncpoint")?;

    let err = unsafe { libc::poll(&mut pollfd as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left}");

    Ok(())
}

uapi_test!("pollfd.create_destroy", create_destroy_pollfd);
pub fn create_destroy_pollfd(main: &Main) -> Result<()> {
    let _syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..Zeroable::zeroed()
    }).context("HOST1X_CREATE_POLLFD failed")?;
    let _pollfd = unsafe { OwnedFd::from_raw_fd(create_pollfd.fd) };

    Ok(())
}

uapi_test!("pollfd.basic", basic_pollfd);
pub fn basic_pollfd(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..Zeroable::zeroed()
    }).context("HOST1X_CREATE_POLLFD failed")?;
    let pollfd = unsafe { OwnedFd::from_raw_fd(create_pollfd.fd) };

    let mut poll_entry = libc::pollfd {
        fd: pollfd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 0, "expected 0 completed polls but got {left}");

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: pollfd.as_raw_fd(),
        id: syncpoint.id(),
        threshold: value.wrapping_add(1),
        ..Zeroable::zeroed()
    }).context("HOST1X_TRIGGER_POLLFD failed")?;

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 0, "expected 0 completed polls but got {left}");

    syncpoint.increment(1).context("Failed to increment syncpoint")?;

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left}");

    Ok(())
}

uapi_test!("pollfd.poll_during_trigger", poll_during_trigger_pollfd);
pub fn poll_during_trigger_pollfd(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let id = syncpoint.id();
    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..Zeroable::zeroed()
    }).context("HOST1X_CREATE_POLLFD failed")?;
    let pollfd = unsafe { OwnedFd::from_raw_fd(create_pollfd.fd) };

    std::thread::scope(|s| {
        let handle = s.spawn(|| {
            let mut poll_entry = libc::pollfd {
                fd: pollfd.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };

            let err = unsafe { libc::poll(&mut poll_entry as _, 1, 100) };
            check_eq!(err, 1, "expected 1 completed polls but got {left}");

            Ok(())
        });

        std::thread::sleep(std::time::Duration::from_millis(10));

        intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
            fd: pollfd.as_raw_fd(),
            id: id,
            threshold: value.wrapping_add(1),
            ..Zeroable::zeroed()
        }).context("HOST1X_TRIGGER_POLLFD failed")?;

        syncpoint.increment(1).context("failed to increment syncpoint")?;

        handle.join().unwrap()?;

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}

uapi_test!("pollfd.past_threshold", past_threshold_pollfd);
pub fn past_threshold_pollfd(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..Zeroable::zeroed()
    }).context("HOST1X_CREATE_POLLFD failed")?;
    let pollfd = unsafe { OwnedFd::from_raw_fd(create_pollfd.fd) };

    let mut poll_entry = libc::pollfd {
        fd: pollfd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };

    // Variant A: Threshold passed before trigger

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: pollfd.as_raw_fd(),
        id: syncpoint.id(),
        threshold: value,
        ..Zeroable::zeroed()
    }).context("HOST1X_TRIGGER_POLLFD failed")?;

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left}");

    Ok(())
}

uapi_test!("pollfd.retrigger", retrigger_pollfd);
pub fn retrigger_pollfd(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..Zeroable::zeroed()
    }).context("HOST1X_CREATE_POLLFD failed")?;
    let pollfd = unsafe { OwnedFd::from_raw_fd(create_pollfd.fd) };

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: pollfd.as_raw_fd(),
        id: syncpoint.id(),
        threshold: value.wrapping_add(1),
        ..Zeroable::zeroed()
    }).context("HOST1X_TRIGGER_POLLFD failed")?;

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: pollfd.as_raw_fd(),
        id: syncpoint.id(),
        threshold: value.wrapping_add(2),
        ..Zeroable::zeroed()
    }).context("second HOST1X_TRIGGER_POLLFD failed")?;

    syncpoint.increment(1).context("Failed to increment syncpoint")?;

    let mut poll_entry = libc::pollfd {
        fd: pollfd.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left}");

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: pollfd.as_raw_fd(),
        id: syncpoint.id(),
        threshold: value.wrapping_add(2),
        ..Zeroable::zeroed()
    }).context("HOST1X_TRIGGER_POLLFD failed during retrigger")?;

    syncpoint.increment(1).context("Failed to increment syncpoint")?;

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left} during retrigger");

    Ok(())
}
