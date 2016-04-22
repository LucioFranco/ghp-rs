# ghp-rs
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

## Programtic Usage
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