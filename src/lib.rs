#![deny(warnings)]
#![cfg_attr(feature="nightly", plugin(clippy))] // clippy runs in nightly

extern crate walkdir;

mod import;
mod error;

pub use import::import_dir;
pub use error::{Result, Error};
