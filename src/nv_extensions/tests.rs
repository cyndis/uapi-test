use anyhow::{Context, Result};
use crate::Main;

use super::host1x_fence::{Host1xFenceInterface, sys};

pub fn create_extract_fence_roundtrip(_main: &Main) -> Result<()> {
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let create_fence = intf.create_fence_raw(sys::host1x_create_fence {
        id: 2,
        threshold: 3,
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_CREATE_FENCE failed")?;

    let mut fence: sys::host1x_fence_extract_fence = unsafe { std::mem::zeroed() };
    intf.fence_extract_raw(sys::host1x_fence_extract {
        fence_fd: create_fence.fence_fd,
        num_fences: 1,
        fences_ptr: &mut fence as *const _ as u64,
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_FENCE_EXTRACT failed")?;

    check_eq!(fence.id, 2, "expected 2 but got {left}");
    check_eq!(fence.threshold, 3, "expected 3 but got {left}");

    Ok(())
}

pub fn created_fence_wait(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_fence = intf.create_fence_raw(sys::host1x_create_fence {
        id: syncpoint.id(),
        threshold: value.wrapping_add(1),
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_CREATE_FENCE failed")?;

    let mut pollfd = libc::pollfd {
        fd: create_fence.fence_fd,
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

pub fn create_destroy_pollfd(main: &Main) -> Result<()> {
    let _syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_CREATE_POLLFD failed")?;

    unsafe { libc::close(create_pollfd.fd) };

    Ok(())
}

pub fn basic_pollfd(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_CREATE_POLLFD failed")?;

    let mut poll_entry = libc::pollfd {
        fd: create_pollfd.fd,
        events: libc::POLLIN,
        revents: 0,
    };

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 0, "expected 0 completed polls but got {left}");

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: create_pollfd.fd,
        id: syncpoint.id(),
        threshold: value.wrapping_add(1),
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_TRIGGER_POLLFD failed")?;

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 0, "expected 0 completed polls but got {left}");

    syncpoint.increment(1).context("Failed to increment syncpoint")?;

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left}");

    unsafe { libc::close(create_pollfd.fd) };

    Ok(())
}

pub fn poll_during_trigger_pollfd(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let id = syncpoint.id();
    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_CREATE_POLLFD failed")?;

    std::thread::scope(|s| {
        let handle = s.spawn(|| {
            let mut poll_entry = libc::pollfd {
                fd: create_pollfd.fd,
                events: libc::POLLIN,
                revents: 0,
            };
        
            let err = unsafe { libc::poll(&mut poll_entry as _, 1, 100) };
            check_eq!(err, 1, "expected 1 completed polls but got {left}");

            Ok(())
        });

        std::thread::sleep(std::time::Duration::from_millis(10));

        intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
            fd: create_pollfd.fd,
            id: id,
            threshold: value.wrapping_add(1),
            ..unsafe { std::mem::zeroed() }
        }).context("HOST1X_TRIGGER_POLLFD failed")?;

        syncpoint.increment(1).context("failed to increment syncpoint")?;

        handle.join().unwrap()?;

        Ok::<(), anyhow::Error>(())
    })?;

    unsafe { libc::close(create_pollfd.fd) };

    Ok(())
}

pub fn past_threshold_pollfd(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_CREATE_POLLFD failed")?;

    let mut poll_entry = libc::pollfd {
        fd: create_pollfd.fd,
        events: libc::POLLIN,
        revents: 0,
    };

    // Variant A: Threshold passed before trigger

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: create_pollfd.fd,
        id: syncpoint.id(),
        threshold: value,
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_TRIGGER_POLLFD failed")?;

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left}");

    unsafe { libc::close(create_pollfd.fd) };

    Ok(())
}

pub fn retrigger_pollfd(main: &Main) -> Result<()> {
    let syncpoint = main.drm.allocate_syncpoint().context("Failed to allocate syncpoint")?;
    let intf = Host1xFenceInterface::open().context("Failed to open host1x-fence interface")?;

    let value = main.drm.read_syncpoint(syncpoint.id()).context("Failed to read syncpoint value")?;

    let create_pollfd = intf.create_pollfd_raw(sys::host1x_create_pollfd {
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_CREATE_POLLFD failed")?;

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: create_pollfd.fd,
        id: syncpoint.id(),
        threshold: value.wrapping_add(1),
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_TRIGGER_POLLFD failed")?;

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: create_pollfd.fd,
        id: syncpoint.id(),
        threshold: value.wrapping_add(2),
        ..unsafe { std::mem::zeroed() }
    }).context("second HOST1X_TRIGGER_POLLFD failed")?;

    syncpoint.increment(1).context("Failed to increment syncpoint")?;

    let mut poll_entry = libc::pollfd {
        fd: create_pollfd.fd,
        events: libc::POLLIN,
        revents: 0,
    };

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left}");

    intf.trigger_pollfd_raw(sys::host1x_trigger_pollfd {
        fd: create_pollfd.fd,
        id: syncpoint.id(),
        threshold: value.wrapping_add(2),
        ..unsafe { std::mem::zeroed() }
    }).context("HOST1X_TRIGGER_POLLFD failed during retrigger")?;

    syncpoint.increment(1).context("Failed to increment syncpoint")?;

    let err = unsafe { libc::poll(&mut poll_entry as _, 1, 10) };
    check_eq!(err, 1, "expected 1 completed polls but got {left} during retrigger");

    unsafe { libc::close(create_pollfd.fd) };

    Ok(())
}
