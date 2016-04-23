# ghp-rs [![Build Status](https://travis-ci.org/LucioFranco/ghp-rs.svg?branch=master)](https://travis-ci.org/LucioFranco/ghp-rs) [![Crates.io](https://img.shields.io/crates/v/ghp.svg?maxAge=2592000)]() [![apache-license](https://img.shields.io/github/license/LucioFranco/ghp-rs.svg?maxAge=2592000)](https://github.com/LucioFranco/ghp-rs/blob/master/LICENSE-APACHE) [![mit-license](https://img.shields.io/badge/license-MIT%20License-blue.svg)](https://github.com/LucioFranco/ghp-rs/blob/master/LICENSE-MIT)
Easily import a folder to your gh-pages branch!

## Why?
I wrote this tool to help people who want to host documentation or anything on a `gh-pages` branch. This was heavily influenced by [ghp-import](https://github.com/davisp/ghp-import) and has a similar implementation.

## Documentation
The documentation can be found [here](https://luciofran.co/ghp-rs). The documentation is powered by `cargo doc` and `ghp-rs`. Check out the [.travis.yml](https://github.com/LucioFranco/ghp-rs/blob/master/.travis.yml) and the [travis setup](https://github.com/LucioFranco/ghp-rs#travis-setup) for an example of how to use `ghp-rs` with `rustdoc`.

## Install

### Via Cargo

```bash
cargo install ghp
```

### Via Binary
Coming Soon!

### Via Git

```bash
git clone https://github.com/LucioFranco/ghp-rs
cargo build --release
export PATH="$PATH:/path/to/ghp-rs"
```

## Command Line Usage
```bash
ghp [OPTIONS] DIRECTORY

# Example

ghp build # move content of build folder to gh-pages branch

ghp --branch website build # will move the contents of the build folder to the website branch

ghp --message "This is a git commit message" build
```

## Programatic Usage
`Cargo.toml`
```toml
[dependencies]
ghp = "0.1"
```

```rust
extern crate ghp;
use ghp::import_dir;

import_dir("build", "gh-pages", "commit message").unwrap();
```

## Travis Setup
For this you will need to have a `GH_TOKEN` variable with your github personal token. Read more [here](http://www.hoverbear.org/2015/03/07/rust-travis-github-pages/).

```yaml
after_success: |
  [ $TRAVIS_RUST_VERSION = stable ] &&
  [ $TRAVIS_BRANCH = master ] &&
  [ $TRAVIS_PULL_REQUEST = false ] &&  
  ghp target/doc &&
  git config user.name "Travis Documentation" &&
  git config user.email "name@example.com" &&
  git push -fq https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
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
