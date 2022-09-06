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

pub mod tegra_drm {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    #![allow(deref_nullptr)]

    include!(concat!(env!("OUT_DIR"), "/tegra_drm_bindings.rs"));
}

pub mod vic {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    #![allow(deref_nullptr)]

    include!(concat!(env!("OUT_DIR"), "/vic_bindings.rs"));

    unsafe impl bytemuck::Pod for OutputConfig {}
    unsafe impl bytemuck::Zeroable for OutputConfig {}

    unsafe impl bytemuck::Pod for OutputSurfaceConfig {}
    unsafe impl bytemuck::Zeroable for OutputSurfaceConfig {}
}

pub mod nvdec {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]
    #![allow(deref_nullptr)]

    include!(concat!(env!("OUT_DIR"), "/nvdec_bindings.rs"));

    unsafe impl bytemuck::Pod for nvdec_mpeg2_pic_s {}
    unsafe impl bytemuck::Zeroable for nvdec_mpeg2_pic_s {}
}

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

mod soc;
mod uapi;

use soc::Soc;
use uapi::Drm;

pub type EResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type IocResult<T> = Result<T, Errno>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Errno(i32);

impl Errno {
    pub fn get() -> Errno {
        Errno(errno::errno().0)
    }
}

impl std::fmt::Display for Errno {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "<Errno {} '{}'>",
            self.0,
            errno::Errno(self.0).to_string()
        )
    }
}

impl std::error::Error for Errno {}

pub const EINVAL: Errno = Errno(libc::EINVAL);
pub const EBUSY: Errno = Errno(libc::EBUSY);
pub const EINTR: Errno = Errno(libc::EINTR);
pub const ENODEV: Errno = Errno(libc::ENODEV);
pub const ENOMEM: Errno = Errno(libc::ENOMEM);
pub const ENOSPC: Errno = Errno(libc::ENOSPC);
pub const EFAULT: Errno = Errno(libc::EFAULT);
pub const EPERM: Errno = Errno(libc::EPERM);

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
}

impl Engine {
    fn class(self) -> u32 {
        match self {
            Engine::Vic => 0x5d,
            Engine::Nvdec => 0xf0,
        }
    }
}

impl std::str::FromStr for Engine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "vic" => Engine::Vic,
            "nvdec" => Engine::Nvdec,
            _ => return Err("unsupported engine"),
        })
    }
}

#[derive(structopt::StructOpt)]
#[structopt(name = "uapi-test", about = "Host1x UAPI test")]
struct Args {
    /// Only list the available tests.
    #[structopt(short, long)]
    list: bool,

    /// Engine to use for engine-independent tests. Defaults to 'vic'. Options: vic, nvdec
    #[structopt(short, long)]
    engine: Option<Engine>,

    /// Test filter. Only run the specified test.
    #[structopt(name = "FILTER")]
    filter: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use structopt::StructOpt;
    let args = Args::from_args();

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
        tests.push(test!(test_read_syncpoints));
        tests.push(test!(test_incr_and_read_syncpoint));
        tests.push(test!(test_allocate_syncpoint));

        tests.push(test!(test_open_channel_invalid_ioctl));
        tests.push(test!(test_open_close_channel));
        tests.push(test!(test_engine_metadata));
        tests.push(test!(test_channel_map_invalid_ioctl));
        tests.push(test!(test_channel_map_unmap));
        tests.push(test!(test_channel_map_gem_close));

        tests.push(test!(test_gem_mmap_invalid_ioctl));
        tests.push(test!(test_gem_mmap));

        tests.push(test!(test_channel_submit_invalid_ioctl));
        tests.push(test!(test_channel_submit_increment_syncpoint_twice));
        tests.push(test!(test_channel_submit_wait));

        if soc.chip_id() == 0x18 || soc.chip_id() == 0x19 || soc.chip_id() == 0x23 {
            tests.push(test!(test_channel_buf_refcounting));
            tests.push(test!(test_channel_submit_vic_clear));
        }

        tests.push(test!(test_nvdec_mpeg2));

        tests.push(test!(test_channel_submit_timeout));
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

    for (name, test) in tests {
        use std::io::Write;

        num_total += 1;

        if let Some(ref filter) = args.filter {
            if filter != name {
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

    if num_run != num_passed {
        Err("Tests failed".to_string())?;
    }

    Ok(())
}
