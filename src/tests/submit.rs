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

use anyhow::{Context, Result};
use std::time::Duration;

use crate::{
    tests::{submit_test, SubmitTestCtx},
    Errno, Main, EFAULT, EINVAL, EPERM,
};

uapi_test!("submit.invalid_ioctl", channel_submit_invalid_ioctl);
pub fn channel_submit_invalid_ioctl(main: &Main) -> Result<()> {
    fn submit(main: &Main, f: impl FnOnce(&mut SubmitTestCtx)) -> Result<Option<Errno>> {
        submit_test(main, main.engine_class, |mut ctx| {
            ctx.push_syncpt_incr(0);
            ctx.setup_submit();
            (f)(&mut ctx);
            Ok(ctx.submit_raw(main).err())
        })
    }

    /* Submit otherwise good jobs, but perturb them slightly to make them invalid. */

    check_eq!(
        submit(main, |_c| ())?,
        None,
        "expected success but got {left:?}"
    );
    check_eq!(
        submit(main, |c| {
            c.args.gather_data_ptr = 0;
            c.args.gather_data_words = 1
        })?,
        Some(EFAULT),
        "expected EFAULT but got {left:?}"
    );
    check_eq!(
        submit(main, |c| {
            c.args.bufs_ptr = 0;
            c.args.num_bufs = 1
        })?,
        Some(EFAULT),
        "expected EFAULT but got {left:?}"
    );
    check_eq!(
        submit(main, |c| {
            c.args.cmds_ptr = 0;
            c.args.num_cmds = 1
        })?,
        Some(EFAULT),
        "expected EFAULT but got {left:?}"
    );

    check_eq!(
        submit(main, |c| c.cmd[0].data.gather_uptr.words = 1000)?,
        Some(EINVAL),
        "expected EINVAL but got {left:?}"
    );

    check_eq!(
        submit(main, |c| c.args.syncpt.flags = 0xffffffff)?,
        Some(EINVAL),
        "expected EINVAL but got {left:?}"
    );
    check_eq!(
        submit(main, |c| c.args.syncpt.id = 0)?,
        Some(EINVAL),
        "expected EINVAL but got {left:?}"
    );

    Ok(())
}

uapi_test!("submit.incr_syncpoint_twice", channel_submit_increment_syncpoint_twice);
pub fn channel_submit_increment_syncpoint_twice(main: &Main) -> Result<()> {
    submit_test(main, main.engine_class, |mut ctx| {
        let base_value = main.drm.read_syncpoint(ctx.syncpt_id)
            .context("failed to read initial syncpoint value")?;

        ctx.push_syncpt_incr(0);
        ctx.push_syncpt_incr(0);

        ctx.submit(main).context("failed to submit job")?;

        /*
         * The kernel is allowed to insert extra increments at the beginning of the job,
         * so allow for more than 2 increments.
         */
        check!(
            ctx.args.syncpt.value.wrapping_sub(base_value) >= 2,
            "job fence value wasn't incremented by at least 2 (base={}, fence={})",
            base_value, ctx.args.syncpt.value
        );

        main.drm
            .wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_secs(1))
            .context(format!(
                "failed to wait for syncpoint {} to reach {} (base={})",
                ctx.syncpt_id, ctx.args.syncpt.value, base_value
            ))?;

        Ok(())
    })
}

uapi_test!("submit.timeout", channel_submit_timeout);
pub fn channel_submit_timeout(main: &Main) -> Result<()> {
    submit_test(main, main.engine_class, |mut ctx| {
        /* First, submit failing job */

        ctx.set_increments(ctx.syncpt.id(), 3);
        ctx.push_incr_syncpt(ctx.syncpt_id, 0);

        ctx.submit(main)?;

        let a = ctx.args.syncpt.value;

        /* Then, submit OK job */

        ctx.incr[0].increments = 1;
        ctx.submit(main)?;

        let b = ctx.args.syncpt.value;

        check!(
            main.drm
                .wait_syncpoint(ctx.syncpt_id, a, Duration::from_secs(20))
                .is_err(),
            "syncpoint was incremented unexpectedly"
        );
        check!(
            main.drm
                .wait_syncpoint(ctx.syncpt_id, b, Duration::from_secs(1))
                .is_err(),
            "follow-up job succeeded though channel should be in error state"
        );

        /* Further submission should be rejected */

        check_err!(
            ctx.submit(main),
            EPERM,
            "expected submission to fail with locked syncpoint, but got {left:?}"
        );

        Ok(())
    })?;

    submit_test(main, main.engine_class, |mut ctx| {
        /* Ensure channel is again usable (with a different syncpoint) */

        ctx.set_increments(ctx.syncpt.id(), 1);
        ctx.push_incr_syncpt(ctx.syncpt_id, 0);

        ctx.submit(main)?;

        main.drm
            .wait_syncpoint(ctx.syncpt_id, ctx.args.syncpt.value, Duration::from_secs(1))?;

        Ok(())
    })
}

uapi_test!("submit.wait_middle", channel_submit_wait_middle);
pub fn channel_submit_wait_middle(main: &Main) -> Result<()> {
    let syncpt = main.drm.allocate_syncpoint()?;
    let syncpt_id = syncpt.id();
    let value = main.drm.read_syncpoint(syncpt_id)?;

    submit_test(main, main.engine_class, |mut ctx| {
        ctx.set_increments(ctx.syncpt.id(), 2);
        ctx.push_incr_syncpt(ctx.syncpt_id, 0);
        ctx.push_wait_syncpt(syncpt_id, value.wrapping_add(1));
        ctx.push_incr_syncpt(ctx.syncpt_id, 0);

        ctx.submit(main)?;

        main.drm.wait_syncpoint(
            ctx.syncpt_id,
            ctx.args.syncpt.value - 1,
            Duration::from_millis(1000),
        )?;
        check!(
            main.drm
                .wait_syncpoint(
                    ctx.syncpt_id,
                    ctx.args.syncpt.value,
                    Duration::from_millis(100)
                )
                .is_err(),
            "expected error"
        );

        syncpt.increment(1)?;

        main.drm.wait_syncpoint(
            ctx.syncpt_id,
            ctx.args.syncpt.value,
            Duration::from_millis(1000),
        )?;

        Ok(())
    })
}

uapi_test!("submit.wait_pre", channel_submit_wait_pre);
pub fn channel_submit_wait_pre(main: &Main) -> Result<()> {
    let syncpt = main.drm.allocate_syncpoint()?;
    let syncpt_id = syncpt.id();
    let value = main.drm.read_syncpoint(syncpt_id)?;

    submit_test(main, main.engine_class, |mut ctx| {
        ctx.set_increments(ctx.syncpt.id(), 1);
        ctx.push_wait_syncpt(syncpt_id, value.wrapping_add(1));
        ctx.push_incr_syncpt(ctx.syncpt_id, 0);

        ctx.submit(main)?;

        check!(
            main.drm
                .wait_syncpoint(
                    ctx.syncpt_id,
                    ctx.args.syncpt.value,
                    Duration::from_millis(100)
                )
                .is_err(),
            "expected error"
        );

        syncpt.increment(1)?;

        main.drm.wait_syncpoint(
            ctx.syncpt_id,
            ctx.args.syncpt.value,
            Duration::from_millis(1000),
        )?;

        Ok(())
    })
}

