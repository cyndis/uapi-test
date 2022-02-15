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

use crate::EResult;

#[derive(Copy, Clone)]
pub struct Soc(u32);
impl Soc {
    pub fn detect() -> EResult<Soc> {
        let soc = std::fs::read_to_string("/sys/devices/soc0/soc_id")?
            .trim_end()
            .parse::<u32>()?;

        Ok(Soc(soc))
    }

    pub fn num_syncpoints(self) -> u32 {
        match self.0 {
            0x20 | 0x30 | 0x35 => 32,
            0x40 | 0x13 => 192,
            0x18 => 576,
            0x19 => 704,
            0x23 => 1024,
            _    => unimplemented!()
        }
    }

    pub fn condition_shift(self) -> u32 {
        if self.num_syncpoints() > 0xff {
            10
        } else {
            8
        }
    }

    pub fn chip_id(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for Soc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self.0 {
            0x20 => "Tegra20",
            0x30 => "Tegra30",
            0x35 => "Tegra114",
            0x40 => "Tegra124",
            0x13 => "Tegra132",
            0x21 => "Tegra210",
            0x18 => "Tegra186",
            0x19 => "Tegra194",
            0x23 => "Tegra234",
            _ => unimplemented!()
        };

        write!(f, "{}", s)
    }
}
