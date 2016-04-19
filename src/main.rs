extern crate getopts;

use getopts::Options;

use std::{env, process};

mod import;
mod error;

fn print_version() {
    println!("0.1.0");
}

fn print_usage(opt: Options) {
    let usage = "ghp [OPTIONS] DIRECTORY";
    println!("{}", opt.usage(usage));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optopt("b",
                "branch",
                "set branch that the files will be imported to",
                "gh-pages");

    opts.optflag("h", "help", "print the help menu");
    opts.optflag("v", "version", "print current version number");

    // TODO: Handle this with a better error message that
    //         will direct people to the gh issues of the project.
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


    let branch = match matches.opt_str("b") {
        Some(path) => path,
        None => String::from("gh-pages"),
    };

    if matches.free.is_empty() {
        print_usage(opts);
        process::exit(1);
    }

    import::import_dir(&matches.free[0], &branch);
}
