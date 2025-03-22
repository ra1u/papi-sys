// Copyright 2018-2019 German Research Center for Artificial Intelligence (DFKI)
// Copyright 2019 Yeonsoo Kim
//
// Authors:
//   Clemens Lutz <clemens.lutz@dfki.de>
//   Yeonsoo Kim <alkorang@outlook.com>
//   Luka Rahne https://github.com/ra1u
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;
    static PAPI_INIT: OnceLock<i32> = OnceLock::new();

    fn do_papi_init() {   
        // init only once
        // PAPI_init is not thread safe, so we use OnceLock 
        // to  PAPI_library_init is only called once in each test
        let ver = *PAPI_INIT.get_or_init(|| {
            unsafe { PAPI_library_init(PAPI_VER_CURRENT) }
        });
        assert_eq!(ver, PAPI_VER_CURRENT);
        let initialised = unsafe { PAPI_is_initialized() };
        assert_ne!(initialised, PAPI_NOT_INITED as i32);
    }

    #[test]
    fn get_real_cyc() {
        do_papi_init();
        let cycles = unsafe { PAPI_get_real_cyc() };
        assert!(cycles >= 0);
    }

    #[test]
    fn get_num_counters() {
        do_papi_init();
        unsafe {
            let num_hwcntrs = PAPI_get_opt(PAPI_MAX_HWCTRS as i32, std::ptr::null_mut());
            assert!(num_hwcntrs >= 0);
        }
    }

    #[test]
    fn hw_info() {
        do_papi_init();
        let hw_info = unsafe {
            let hwi = PAPI_get_hardware_info();
            assert!(!hwi.is_null());
            &*hwi
        };
        assert!(hw_info.totalcpus >= 1);
    }
}
