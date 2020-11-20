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

pub mod host1x {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/host1x_bindings.rs"));
}

pub mod sync_file {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/sync_file_bindings.rs"));
}

pub mod tegra_drm {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/tegra_drm_bindings.rs"));
}

pub mod vic {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/vic_bindings.rs"));

    unsafe impl bytemuck::Pod for OutputConfig {}
    unsafe impl bytemuck::Zeroable for OutputConfig {}

    unsafe impl bytemuck::Pod for OutputSurfaceConfig {}
    unsafe impl bytemuck::Zeroable for OutputSurfaceConfig {}
}

macro_rules! check {
    ($e:expr) => {
        if !($e) {
            Err(format!("Check failed on {}:{}: {}", file!(),
                        line!(), stringify!($e)))?;
        }
    }
}

macro_rules! check_eq {
    ($a:expr, $b:expr) => {
        let a_val = $a;
        let b_val = $b;
        if !(a_val == b_val) {
            Err(format!("Check failed on {}:{}: Expected '{}' to be '{:?}', but it was '{:?}'",
                        file!(), line!(),
                        stringify!($a), b_val, a_val))?;
        }
    }
}

macro_rules! check_unwrap {
    ($e:expr) => {
        {
            let val = $e;
            if val.is_err() {
                return Err(format!("Check failed on {}:{}: Expected '{}' to succeed, but it failed with '{:?}'",
                            file!(), line!(), stringify!($e), val.unwrap_err()).into());
            } else {
                val.unwrap()
            }
        }
    }
}

mod soc;
mod types;

use soc::Soc;
use types::{Host1x, Drm};

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
        write!(f, "<Errno {} '{}'>", self.0, errno::Errno(self.0).to_string())
    }
}

impl std::error::Error for Errno {
}

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
    host1x: Host1x,
    drm: Drm,

    engine_class: u32,
}

mod test_syncpoints;
mod test_channels;

use test_syncpoints::*;
use test_channels::*;

#[derive(structopt::StructOpt)]
#[structopt(name = "uapi-test", about = "Host1x UAPI test")]
struct Args {
    /// Only list the available tests.
    #[structopt(short, long)]
    list: bool,

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

    let host1x = Host1x::open()?;
    let drm = Drm::open()?;

    let engine_class = match soc.chip_id() {
        0x12 | 0x21 | 0x18 | 0x19 | 0x23 => 0x5d /* VIC */,
        _                                => unimplemented!(),
    };

    let main = Main { soc, host1x, drm, engine_class };

    type Test = dyn Fn(&Main) -> EResult<()>;
    let mut tests: Vec<(&str, Box<Test>)> = vec![];

    macro_rules! test {
        ($e:expr) => ((stringify!($e), Box::new($e)))
    }

    tests.push(test!(test_read_syncpoints));
    tests.push(test!(test_allocate_syncpoint_invalid_ioctl));
    tests.push(test!(test_allocate_syncpoint));
    tests.push(test!(test_increment_syncpoint));
    tests.push(test!(test_increment_syncpoint_intr));
    tests.push(test!(test_create_fence_invalid_ioctl));
    tests.push(test!(test_create_fence));
    tests.push(test!(test_create_fence_and_signal));
    tests.push(test!(test_extract_fence));

    tests.push(test!(test_open_channel_invalid_ioctl));
    tests.push(test!(test_open_close_channel));
    tests.push(test!(test_engine_metadata));

    tests.push(test!(test_gem_create_invalid_ioctl));
    tests.push(test!(test_gem_mmap_invalid_ioctl));
    tests.push(test!(test_gem_mmap));

    tests.push(test!(test_channel_map_invalid_ioctl));
    tests.push(test!(test_channel_map_unmap));
    tests.push(test!(test_channel_map_gem_close));

    tests.push(test!(test_channel_submit_invalid_ioctl));
    tests.push(test!(test_channel_submit_increment_syncpoint_twice));

    tests.push(test!(test_channel_submit_wait));

    if soc.chip_id() == 0x18 {
        tests.push(test!(test_channel_buf_refcounting));
        tests.push(test!(test_channel_submit_vic_clear));
    }

    tests.push(test!(test_channel_submit_timeout));

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
                println!("  {}", e.to_string());
            }
        }
    }

    println!("---------------------------------------------------------");
    println!("{} tests total. {} run, {} skipped", num_total, num_run, num_total-num_run);
    println!("{} passed, {} failed", num_passed, num_run-num_passed);

    if num_run != num_passed {
        Err("Tests failed".to_string())?;
    }

    Ok(())
}
