extern crate getopts;
extern crate git2;

use getopts::Options;

use std::{env, process};

mod import;

fn print_version() {
    println!("0.1.0");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optopt("p",
                "path",
                "set path that will be transplanted to the gh-pages branch",
                "");

    opts.optflag("h", "help", "print the help menu");
    opts.optflag("v", "version", "print current version number");

    // TODO: Handle this with a better error message that
    //         will direct people to the gh issues.
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        println!("help message");
        process::exit(0);
    }

    if matches.opt_present("v") {
        print_version();
        process::exit(0);
    }

    if let Some(path) = matches.opt_str("p") {
        import::import_dir(&path);
    }
}
