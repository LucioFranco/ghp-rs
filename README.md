# ghp-rs [![Build Status](https://travis-ci.org/LucioFranco/ghp-rs.svg?branch=master)](https://travis-ci.org/LucioFranco/ghp-rs) [![Crates.io](https://img.shields.io/crates/v/ghp.svg?maxAge=2592000)]() [![apache-license](https://img.shields.io/github/license/LucioFranco/ghp-rs.svg?maxAge=2592000)](https://github.com/LucioFranco/ghp-rs/blob/master/LICENSE-APACHE) [![mit-license](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/LucioFranco/ghp-rs/blob/master/LICENSE-MIT)
Easily import a folder to your gh-pages branch!

## Why?
I wrote this tool to help people who want to host documentation or anything on a `gh-pages` branch. This was heavily influenced by [ghp-import](https://github.com/davisp/ghp-import) and has a similar implementation.

## Install

### Via Cargo

```
cargo install ghp
```

### Via Binary
Coming Soon!

### Via Git

```
git clone https://github.com/LucioFranco/ghp-rs
cargo build
```

## Command Line Usage
```
ghp [OPTIONS] DIRECTORY

# Example

ghp build # move content of build folder to gh-pages branch

ghp --branch website build # will move the contents of the build folder to the website branch
```

## Programatic Usage
```
extern crate ghp;
use ghp::import_dir;

import_dir("build", "gh-pages").unwrap();
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
