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

pub mod drm;
pub mod channel;
pub mod gem;
pub mod mapping;
pub mod syncpoint;

pub use drm::Drm;
pub use channel::Channel;
pub use gem::Gem;
pub use mapping::Mapping;
pub use syncpoint::Syncpoint;

 #[cfg(target_env = "musl")]
 unsafe fn ioctl<T>(fd: i32, ioc: u64, data: T) -> i32 {
     /*
      * We want to pass an unsigned int in through an int parameter,
      * so need to transmute for a bit-exact copy..
      */
     libc::ioctl(fd, std::mem::transmute(ioc as u32), data)
 }
 #[cfg(not(target_env = "musl"))]
 unsafe fn ioctl<T>(fd: i32, ioc: u64, data: T) -> i32 {
     libc::ioctl(fd, ioc, data)
 }
