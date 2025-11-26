// MIT License
//
// Copyright (c) 2017 Toshiki Teramura
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use anyhow::Result;

fn main() -> Result<()> {
    let mkl = format!(
        "mkl-{}-{}-{}",
        match (cfg!(feature = "static"), cfg!(feature = "dynamic")) {
            (true, false) => "static",
            (false, true) => "dynamic",
            (false, false) => "static",
            _ => {
                panic!("conflicting features: both 'static' and 'dynamic' are enabled")
            }
        },
        match (cfg!(feature = "ilp64"), cfg!(feature = "lp64")) {
            (true, false) => "ilp64",
            (false, true) => "lp64",
            (false, false) => "ilp64",
            _ => {
                panic!("conflicting features: both 'ilp64' and 'lp64' are enabled")
            }
        },
        match (cfg!(feature = "iomp"), cfg!(feature = "seq")) {
            (true, false) => "iomp",
            (false, true) => "seq",
            (false, false) => "iomp",
            _ => {
                panic!("conflicting features: both 'iomp' and 'seq' are enabled")
            }
        }
    );
    let lib = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe(&mkl)?;
    //println!("cargo:rerun-if-env-changed=MKLROOT");

    for path in lib.link_paths {
        println!("cargo::rustc-link-search={}", path.display());
    }
    for staticlib in lib.link_files {
        println!(
            "cargo::rustc-link-lib=static:+verbatim={}",
            staticlib.display()
        );
    }
    for ld_arg in lib.ld_args {
        println!("cargo::rustc-link-arg=-Wl,{}", ld_arg.join(","));
    }
    if cfg!(feature = "no-as-needed") {
        for dylib in lib.libs {
            println!("cargo::rustc-link-lib=dylib:-as-needed={}", dylib);
        }
    } else {
        for dylib in &lib.libs {
            println!("cargo::rustc-link-lib=dylib={}", dylib);
        }
        println!(
            "cargo::metadata=LINKARG=-Wl,--no-as-needed,-l{},--as-needed",
            lib.libs.join(",-l")
        );
    }

    Ok(())
}
