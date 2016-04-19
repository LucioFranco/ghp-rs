extern crate git2;

mod import;
mod error;

pub use import::import_dir;
pub use error::{Result, Error};
