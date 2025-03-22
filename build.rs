// Copyright 2018-2019 German Research Center for Artificial Intelligence (DFKI)
// Copyright 2019 Yeonsoo Kim
//
// Authors:
//   Clemens Lutz <clemens.lutz@dfki.de>
//   Yeonsoo Kim <alkorang@outlook.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::vec::Vec;

fn assert_command_ok(output: &Output, error_message: &str) -> std::io::Result<()> {
    if !output.status.success() {
        let to_str = |s: &[u8]| String::from_utf8_lossy(s).to_string();
        panic!(
            "{}: {} {}",
            error_message,
            to_str(&output.stdout),
            to_str(&output.stderr)
        );
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let library = pkg_config::probe_library("papi")?;
    let clang_args = library
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.to_string_lossy()))
        .collect::<Vec<_>>();

   #[cfg(feature = "static-linkage")]    
    println!("cargo:rustc-link-lib=static=papi");
    #[cfg(not(feature = "static-linkage"))]
    println!("cargo:rustc-link-lib=papi");

    bindgen::builder()
        .header("src/wrapper.h")
        .clang_args(clang_args.iter())
        .allowlist_recursively(false)
        .allowlist_type("^PAPI_[[:alpha:]_]+")
        .allowlist_type("^_papi_[[:alpha:]_]+")
        .allowlist_function("^PAPI_[[:alpha:]_]+")
        .allowlist_function("^_papi_[[:alpha:]_]+")
        .allowlist_var("^PAPI_[[:alpha:]_]+")
        .allowlist_var("^_papi_[[:alpha:]_]+")
        .allowlist_type("caddr_t")
        .allowlist_type("__caddr_t")
        .allowlist_type("_dmem_t")
        .allowlist_type("event_info")
        .allowlist_type("vptr_t")
        .generate()
        .expect("Unable to generate PAPI bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write PAPI bindings");

    // generate trivial binary that exposes versions of PAPI
    // use clang to generate codegen
    let codegen_binary_path = out_path.join("codegen");
    let codegen = Command::new("clang")
        .args(clang_args.iter())
        .args(["-o", codegen_binary_path.to_str().unwrap()])
        .arg("src/codegen.c")
        .output()
        .expect("failed to execute process");
    assert_command_ok(&codegen, "Codegen failed")?;

    // run codegen and fetch output
    let codegen_run = Command::new(codegen_binary_path)
        .output()
        .expect("failed to execute process");
    assert_command_ok(&codegen_run, "Codegen run failed")?;

    let codegen_stdout = codegen_run.stdout;

    let mut file = File::create(out_path.join("codegen.rs"))?;
    file.write_all(&codegen_stdout)?;
    file.sync_all()?;

    Ok(())
}
