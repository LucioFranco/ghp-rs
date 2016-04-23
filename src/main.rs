#![crate_name = "ghp"]

extern crate getopts;
extern crate walkdir;

use getopts::Options;

use std::{env, process};
use std::io::{Write, stderr};
use std::error::Error;

mod import;
mod push;
mod error;

fn print_version() {
    println!("0.1.0");
}

fn print_usage(opt: Options) {
    let usage = "ghp [OPTIONS] DIRECTORY";
    println!("{}", opt.usage(usage));
}

fn write_stderr(err: &str) {
    stderr().write_fmt(format_args!("Error: {}\n", err)).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.optopt("b",
                "branch",
                "set branch that the files will be imported to. Default: gh-pages",
                "BRANCH");
    opts.optopt("p",
                "push",
                "push to specific remote. Default: origin",
                "REMOTE");
    opts.optopt("m",
                "message",
                "git commit message. Default: ghp import",
                "MESSAGE");

    opts.optflag("f", "force", "git force push to branch");
    opts.optflag("h", "help", "print the help menu");
    opts.optflag("v", "version", "print current version number");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            write_stderr(&format!("This error should not be happening. Please submit and issue \
                                   on github :)\n\"{}\"",
                                  f));
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        process::exit(0);
    }

    if matches.opt_present("v") {
        print_version();
        process::exit(0);
    }

    let message = match matches.opt_str("m") {
        Some(msg) => msg,
        None => String::from("ghp import"),
    };

    let branch = match matches.opt_str("b") {
        Some(path) => path,
        None => String::from("gh-pages"),
    };

    if matches.free.is_empty() {
        print_usage(opts);
        process::exit(1);
    }

    match import::import_dir(&matches.free[0], &branch, &message) {
        Ok(_) => {
            // TODO: add push

            process::exit(0);
        }

        Err(ref err) => {
            write_stderr(err.description());
            process::exit(1);
        }
    }
}
