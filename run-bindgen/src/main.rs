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

use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct MakeMacroConstDefs;
impl bindgen::callbacks::ParseCallbacks for MakeMacroConstDefs {
    fn item_name(&self, original: &str) -> Option<String> {
        Some(original.trim_start_matches("MK_").to_owned())
    }
}

fn generate_bindings(wrapper: &str, out: &str) {
    let bindings = bindgen::Builder::default()
        .header(wrapper)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(MakeMacroConstDefs))
        .generate()
        .expect("Failed to generate IOCTL bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join(out))
        .expect("Failed to write IOCTL bindings");
}

fn main() {
    generate_bindings("tegra_drm_wrapper.h", "tegra_drm_bindings.rs");
    generate_bindings("engine-hdr/vic.h", "vic_bindings.rs");
    generate_bindings("engine-hdr/nvdec.h", "nvdec_bindings.rs");
}
