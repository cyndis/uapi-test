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
pub enum Engine {
    Gr2d,
    Gr3d,
    Vic,
    Nvdec,
    Ofa,
    Nvenc,
}

impl Engine {
    pub fn class(self) -> u32 {
        match self {
            Engine::Gr2d => 0x51,
            Engine::Gr3d => 0x60,
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
            "gr2d" => Engine::Gr2d,
            "gr3d" => Engine::Gr3d,
            "vic" => Engine::Vic,
            "nvdec" => Engine::Nvdec,
            "ofa" => Engine::Ofa,
            "nvenc" => Engine::Nvenc,
            _ => return Err("unsupported engine"),
        })
    }
}

#[derive(Copy, Clone)]
pub struct Soc(u32);
impl Soc {
    pub fn detect() -> EResult<Soc> {
        let soc_id = std::fs::read_to_string("/sys/devices/soc0/soc_id")?;
        let soc_id = soc_id.trim_end();

        // soc_id can be a plain decimal number or a JEP106 string like
        // "jep106:036b:0264". In the latter case, parse the last field as hex
        // and drop the last nibble to get the legacy chip id (e.g. 0x0264 >> 4 = 0x26).
        let soc = if let Some(id_hex) = soc_id.rsplit(':').next().filter(|_| soc_id.contains(':')) {
            u32::from_str_radix(id_hex, 16)? >> 4
        } else {
            soc_id.parse::<u32>()?
        };

        Ok(Soc(soc))
    }

    pub fn num_syncpoints(self) -> u32 {
        match self.0 {
            0x20 | 0x30 | 0x35 => 32,
            0x40 | 0x13 | 0x21 => 192,
            0x18 => 576,
            0x19 => 704,
            0x23 | 0x26 => 1024,
            _ => unimplemented!(),
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

    /// The hardware version the kernel reports for this SoC's engines. Up to
    /// Tegra234 that is the chip id; from Tegra264 it is the full one (0x264)
    /// so that minor variants of a chip can be told apart.
    pub fn engine_version(self) -> u32 {
        match self.0 {
            0x26 => 0x264,
            id => id,
        }
    }

    pub fn default_engine(self) -> Engine {
        match self.0 {
            // Pre-Tegra124: GR2D
            0x20 | 0x30 | 0x35 => Engine::Gr2d,
            // Tegra124+: VIC
            _ => Engine::Vic,
        }
    }

    pub fn is_vic5(self) -> bool {
        // Tegra264 has VIC 5.1 (CEB6) with a different config struct layout
        self.0 == 0x26
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
            0x26 => "Tegra264",
            _ => unimplemented!(),
        };

        write!(f, "{}", s)
    }
}
