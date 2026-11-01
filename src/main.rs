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

/// Returned by a test that cannot run on this system, for example because the
/// engine it exercises is not present. Reported as SKIPPED rather than FAILED.
#[derive(Debug)]
pub struct Skip(pub String);

impl std::fmt::Display for Skip {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Skip {}

pub struct Test {
    pub name: &'static str,
    pub func: fn(&Main) -> anyhow::Result<()>,
    pub soc: &'static [u32],
}

inventory::collect!(Test);

#[macro_export]
macro_rules! uapi_test {
    ($name:literal, $func:ident) => {
        inventory::submit!(crate::Test {
            name: $name,
            func: $func,
            soc: &[],
        });
    };
    ($name:literal, $func:ident, soc = [$($soc:literal),* $(,)?]) => {
        inventory::submit!(crate::Test {
            name: $name,
            func: $func,
            soc: &[$($soc),*],
        });
    };
}

mod tests;

use soc::Engine;

#[derive(clap::Parser)]
#[command(name = "uapi-test", about = "Host1x UAPI test")]
struct Args {
    /// Only list the available tests.
    #[arg(short, long)]
    list: bool,

    /// Engine to use for engine-independent tests. Defaults to 'vic'. Options: vic, nvdec, ofa, nvenc
    #[arg(short, long)]
    engine: Option<Engine>,

    /// Disable colored output and spinner (automatic when not a terminal).
    #[arg(long)]
    plain: bool,

    /// Comma-separated list of staged test names. These tests run but failures are ignored.
    #[arg(short, long, value_delimiter = ',')]
    staged: Vec<String>,

    /// Test filter. Only run the specified test.
    filter: Option<String>,
}

struct Output {
    color: bool,
}

struct SpinnerHandle {
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Drop for SpinnerHandle {
    fn drop(&mut self) {
        use std::sync::atomic::Ordering;
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.thread.take() {
            let _ = handle.join();
        }
    }
}

impl Output {
    fn new(plain: bool) -> Self {
        let is_tty = unsafe { libc::isatty(libc::STDOUT_FILENO) } != 0;
        Output { color: !plain && is_tty }
    }

    fn green(&self, text: &str) -> String {
        if self.color { format!("\x1b[32m{text}\x1b[0m") } else { text.to_string() }
    }

    fn red(&self, text: &str) -> String {
        if self.color { format!("\x1b[31m{text}\x1b[0m") } else { text.to_string() }
    }

    fn yellow(&self, text: &str) -> String {
        if self.color { format!("\x1b[33m{text}\x1b[0m") } else { text.to_string() }
    }

    fn bold(&self, text: &str) -> String {
        if self.color { format!("\x1b[1m{text}\x1b[0m") } else { text.to_string() }
    }

    fn start_spinner(&self, name: &str) -> Option<SpinnerHandle> {
        if !self.color {
            return None;
        }

        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stop_clone = stop.clone();
        let name = name.to_string();

        let thread = std::thread::spawn(move || {
            use std::io::Write;
            use std::sync::atomic::Ordering;

            const FRAMES: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let mut i = 0;

            while !stop_clone.load(Ordering::Relaxed) {
                eprint!("\r{:<50} \x1b[33m{}\x1b[0m", name, FRAMES[i % FRAMES.len()]);
                let _ = std::io::stderr().flush();
                i += 1;
                std::thread::sleep(std::time::Duration::from_millis(80));
            }

            // Clear spinner line
            eprint!("\r{:55}\r", "");
            let _ = std::io::stderr().flush();
        });

        Some(SpinnerHandle { stop, thread: Some(thread) })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    let args = Args::parse();
    let out = Output::new(args.plain);

    println!("{}", out.bold("Host1x UAPI test"));

    let soc = Soc::detect()?;

    println!("Detected SoC: {}", soc);
    println!("{}", "-".repeat(64));

    let drm = Drm::open()?;

    let engine_class = args.engine.unwrap_or(soc.default_engine()).class();

    let main = Main {
        soc,
        drm,
        engine_class,
    };

    let has_host1x_fence = std::path::Path::new("/dev/host1x-fence").exists();

    const SUITE_ORDER: &[&str] = &[
        "syncpoint", "channel", "gem", "submit", "engine", "fence", "pollfd",
    ];

    let mut tests: Vec<&Test> = Vec::new();
    for suite in SUITE_ORDER {
        if !has_host1x_fence && (*suite == "fence" || *suite == "pollfd") {
            continue;
        }
        for test in inventory::iter::<Test> {
            if test.name.starts_with(suite)
                && test.name.as_bytes().get(suite.len()) == Some(&b'.')
            {
                tests.push(test);
            }
        }
    }

    if args.list {
        for test in &tests {
            println!("{}", test.name);
        }

        return Ok(());
    }

    let mut num_total = 0;
    let mut num_run = 0;
    let mut num_passed = 0;
    let mut num_staged_failed = 0;
    let mut num_skipped = 0;

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

    let total_start = std::time::Instant::now();

    for test in &tests {
        num_total += 1;

        if let Some(ref filter) = args.filter {
            if !test.name.contains(filter.as_str()) {
                continue;
            }
        }

        if !test.soc.is_empty() && !test.soc.contains(&soc.chip_id()) {
            continue;
        }

        num_run += 1;

        let staged = args.staged.iter().any(|s| s == test.name);

        let spinner = out.start_spinner(test.name);
        let start = std::time::Instant::now();
        let result = (test.func)(&main);
        let elapsed = start.elapsed();
        drop(spinner);

        let staged_suffix = if staged { " (staged)" } else { "" };
        match result {
            Ok(()) => {
                println!(
                    "{:<50} {} ({:.2}s)",
                    test.name,
                    out.green(&format!("OK{staged_suffix}")),
                    elapsed.as_secs_f64()
                );
                num_passed += 1;
            }
            Err(e) if e.downcast_ref::<Skip>().is_some() => {
                println!(
                    "{:<50} {} ({:.2}s)",
                    test.name,
                    out.yellow("SKIPPED"),
                    elapsed.as_secs_f64()
                );
                println!("  {}", e);
                num_skipped += 1;
            }
            Err(e) => {
                println!(
                    "{:<50} {} ({:.2}s)",
                    test.name,
                    if staged { out.yellow(&format!("FAILED{staged_suffix}")) } else { out.red("FAILED") },
                    elapsed.as_secs_f64()
                );
                println!("  {:?}", e);
                if staged {
                    num_staged_failed += 1;
                }
            }
        }
    }

    let total_elapsed = total_start.elapsed();

    println!("{}", "-".repeat(64));
    println!(
        "{} tests total. {} run, {} skipped ({:.2}s)",
        num_total,
        num_run - num_skipped,
        num_total - num_run + num_skipped,
        total_elapsed.as_secs_f64()
    );

    let num_failed = num_run - num_passed - num_staged_failed - num_skipped;
    let passed_str = format!("{} passed", num_passed);
    let failed_str = format!("{} failed", num_failed);
    let staged_str = format!("{} failed (staged)", num_staged_failed);
    let mut summary = format!(
        "{}, {}",
        out.green(&passed_str),
        if num_failed > 0 { out.red(&failed_str) } else { out.green(&failed_str) }
    );
    if num_staged_failed > 0 {
        summary += &format!(", {}", out.yellow(&staged_str));
    }
    println!("{}", summary);

    if let Some(kmemleak) = kmemleak {
        match kmemleak.finish()? {
            Some(report) => {
                println!("{}", out.red("Memory leak(s) found:"));
                println!("{}", report);
            }
            None => {
                println!("{}", out.green("No memory leaks detected."));
            }
        }
    }

    if num_failed > 0 {
        Err("Tests failed".to_string())?;
    }

    Ok(())
}
