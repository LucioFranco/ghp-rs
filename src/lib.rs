#![crate_name = "ghp"]
#![doc(html_root_url = "https://luciofran.co/ghp-rs")]

#![deny(warnings)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate walkdir;

mod import;
mod error;

pub use import::import_dir;
pub use error::{Result, Error};
