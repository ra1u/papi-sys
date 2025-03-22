papi-sys
========

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
papi-sys = "0.1.0"
```

Before building, ensure that PAPI is installed on your system.

## What is papi-sys?

The purpose of this crate is to provide 1:1 bindings for papi.h.
PAPI is a library that provides a consistent interface to hardware performance
counters. Visit the [PAPI website](http://icl.utk.edu/papi) for more information.

Note that this crate does not provide a high-level interface to PAPI.

## Building

tested with papi version 7.2

By default pkg-config is used to detect libraray paths.  
Install papi and pkg-config with your distro package manager.

```bash
apt install libpapi-dev pkg-config
```

build and test this library

```bash
$ cargo test
```

# Custom library path

If papi libraray is not installed where pkg-config can discover it automaticaly
pass variable `PKG_CONFIG_PATH` to installed folder accordingly. 

For example if you compiled and installed papi libraray as

```bash
$ ./configure --prefix=<INSTALL_DIR> && make && make install
```

You should find pkg-config folder in  `<INSTALL_DIR>/lib/pkgconfig`

```bash
$ PKG_CONFIG_PATH=<INSTALL_DIR>/lib/pkgconfig cargo test --features static-linkage
```
Feature flag static-linkage enables static linkage, or else you will need to set
`LD_LIBRARY_PATH` accordingly for each run of binary

```bash
$ PKG_CONFIG_PATH=<INSTALL_DIR>/lib/pkgconfig LD_LIBRARY_PATH=<INSTALL_DIR>/lib/ cargo test
```

## Platforms

The following platforms are currently tested:

* `x86_64-unknown-linux-gnu`
* `powerpc64le-unknown-linux-gnu`

## Dependencies

The following dependency versions are currently required:

* `rustc / cargo` >= 1.70
* `clang` 

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
