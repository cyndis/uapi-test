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

pub mod tegra_drm;

macro_rules! check {
    ($a:expr, $($msg:tt)+) => {
        {
            let a = $a;

            if !a {
                anyhow::bail!(format!($($msg)+));
            }
        }
    }
}

macro_rules! check_ok {
    ($a:expr, $($msg:tt)+) => {
        {
            let a = $a;

            if let Err(e) = a {
                anyhow::bail!(format!($($msg)+, err=e));
            }
        }
    }
}

macro_rules! check_eq {
    ($a:expr, $b:expr, $($msg:tt)+) => {
        {
            let a = $a;
            let b = $b;

            if a != b {
                anyhow::bail!(format!($($msg)+, left=a));
            }
        }
    }
}

macro_rules! check_err {
    ($a:expr, $b:expr, $($msg:tt)+) => {
        {
            let a = $a;
            let b = $b;

            if a.err() != Some(b) {
                anyhow::bail!(format!(concat!($($msg)+, "{left:.0?}"), left=a.err()));
            }
        }
    }
}

mod kmemleak;
mod soc;
mod uapi;

mod nv_extensions;

use soc::Soc;
use uapi::Drm;

pub use nix::errno::Errno;

pub type EResult<T> = anyhow::Result<T>;
pub type IocResult<T> = Result<T, Errno>;

pub const EINVAL: Errno = Errno::EINVAL;
pub const EBUSY: Errno = Errno::EBUSY;
pub const EINTR: Errno = Errno::EINTR;
pub const ENODEV: Errno = Errno::ENODEV;
pub const ENOMEM: Errno = Errno::ENOMEM;
pub const ENOSPC: Errno = Errno::ENOSPC;
pub const EFAULT: Errno = Errno::EFAULT;
pub const EPERM: Errno = Errno::EPERM;

pub struct Main {
    soc: Soc,
    drm: Drm,

    engine_class: u32,
}

mod tests;

#[derive(Copy, Clone)]
enum Engine {
    Vic,
    Nvdec,
    Ofa,
    Nvenc,
}

impl Engine {
    fn class(self) -> u32 {
        match self {
            Engine::Vic => 0x5d,
            Engine::Nvdec => 0xf0,
            Engine::Ofa => 0xf8,
            Engine::Nvenc => 0x21,
        }
    }
}

impl std::str::FromStr for Engine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "vic" => Engine::Vic,
            "nvdec" => Engine::Nvdec,
            "ofa" => Engine::Ofa,
            "nvenc" => Engine::Nvenc,
            _ => return Err("unsupported engine"),
        })
    }
}

#[derive(clap::Parser)]
#[command(name = "uapi-test", about = "Host1x UAPI test")]
struct Args {
    /// Only list the available tests.
    #[arg(short, long)]
    list: bool,

    /// Engine to use for engine-independent tests. Defaults to 'vic'. Options: vic, nvdec, ofa, nvenc
    #[arg(short, long)]
    engine: Option<Engine>,

    /// Test filter. Only run the specified test.
    filter: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    let args = Args::parse();

    println!("Host1x UAPI test");

    let soc = Soc::detect()?;

    println!("Detected SoC: {}", soc);
    println!("---------------------------------------------------------");

    let drm = Drm::open()?;

    let engine = args.engine.unwrap_or(Engine::Vic);

    let main = Main {
        soc,
        drm,
        engine_class: engine.class(),
    };

    type Test = dyn Fn(&Main) -> anyhow::Result<()>;
    let mut tests: Vec<(&str, Box<Test>)> = vec![];

    macro_rules! test {
        ($e:expr) => {
            (stringify!($e), Box::new($e))
        };
    }

    {
        use tests::{channels::*, gem::*, nvdec::*, submit::*, syncpoints::*};
        tests.push(test!(read_syncpoints));
        tests.push(test!(incr_and_read_syncpoint));
        tests.push(test!(allocate_syncpoint));
        tests.push(test!(wait_syncpoint_multiple));
        tests.push(test!(timeout_correctness));
        tests.push(test!(wait_all_syncpoints));

        tests.push(test!(open_channel_invalid_ioctl));
        tests.push(test!(open_close_channel));
        tests.push(test!(engine_metadata));
        tests.push(test!(channel_map_invalid_ioctl));
        tests.push(test!(channel_map_unmap));
        tests.push(test!(channel_map_gem_close));

        tests.push(test!(gem_mmap_invalid_ioctl));
        tests.push(test!(gem_mmap));

        tests.push(test!(channel_submit_invalid_ioctl));
        tests.push(test!(channel_submit_increment_syncpoint_twice));
        tests.push(test!(channel_submit_wait_middle));
        tests.push(test!(channel_submit_wait_pre));

        if soc.chip_id() == 0x18
            || soc.chip_id() == 0x19
            || soc.chip_id() == 0x23
            || soc.chip_id() == 0x26
        {
            tests.push(test!(channel_buf_refcounting));
            tests.push(test!(channel_submit_vic_clear));
        }

        tests.push(test!(nvdec_mpeg2));

        tests.push(test!(channel_submit_timeout));

        /* NVIDIA internal tests */
        use nv_extensions::tests as nv_extensions;
        tests.push(test!(nv_extensions::create_extract_fence_roundtrip));
        tests.push(test!(nv_extensions::created_fence_wait));
        tests.push(test!(nv_extensions::create_destroy_pollfd));
        tests.push(test!(nv_extensions::basic_pollfd));
        tests.push(test!(nv_extensions::poll_during_trigger_pollfd));
        tests.push(test!(nv_extensions::past_threshold_pollfd));
        tests.push(test!(nv_extensions::retrigger_pollfd));
    }

    if args.list {
        for (name, _) in tests {
            println!("{}", name);
        }

        return Ok(());
    }

    let mut num_total = 0;
    let mut num_run = 0;
    let mut num_passed = 0;

    let kmemleak = match kmemleak::Session::start() {
        Ok(session) => Some(session),
        Err(kmemleak::Error::NotSupported) => {
            println!("NOTE: kmemleak interface is not available");
            None
        }
        Err(e) => {
            println!("ERROR: Failed to start kmemleak session: {}", e);
            None
        }
    };

    for (name, test) in tests {
        use std::io::Write;

        num_total += 1;

        if let Some(ref filter) = args.filter {
            if !name.contains(filter) {
                continue;
            }
        }

        num_run += 1;

        print!("{:<50} ", name);
        std::io::stdout().lock().flush().unwrap();
        match (test)(&main) {
            Ok(()) => {
                println!("    OK");
                num_passed += 1;
            }
            Err(e) => {
                println!("Failed");
                println!("  {:?}", e);
            }
        }
    }

    println!("---------------------------------------------------------");
    println!(
        "{} tests total. {} run, {} skipped",
        num_total,
        num_run,
        num_total - num_run
    );
    println!("{} passed, {} failed", num_passed, num_run - num_passed);

    if let Some(kmemleak) = kmemleak {
        match kmemleak.finish()? {
            Some(report) => {
                println!("Memory leak(s) found:");
                println!("{}", report);
            }
            None => {
                println!("No memory leaks detected.");
            }
        }
    }

    if num_run != num_passed {
        Err("Tests failed".to_string())?;
    }

    Ok(())
}
