use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ChildStdin};
use std::io::{Write, Read};
use error::{Result, Error};
use std::fs::File;
use walkdir::WalkDir;

pub fn import_dir<P>(dir: P, branch: &str) -> Result<()>
    where P: AsRef<Path> + Clone
{
    println!("{:?} to {:?}", dir.as_ref(), branch);

    let mut cmd = try!(Command::new("git")
                           .arg("fast-import")
                           .arg("--date-format=now")
                           .arg("--quiet")
                           .stdin(Stdio::piped())
                           .current_dir(&dir)
                           .spawn());


    let mut stdin = match cmd.stdin.take() {
        Some(buf) => buf,
        None => return Err(Error::from("did not capture stdin")),
    };

    let mut import = Import::new(stdin, branch, &dir);
    try!(import.import());

    println!("done");
    try!(cmd.kill()); // TODO: Kill is so brutal. Should find a better way to end commit.

    Ok(())
}

struct Import {
    stdin: ChildStdin,
    branch: String,
    dir: PathBuf, // TODO: Should prob make this just a reference
}

impl Import {
    pub fn new<P>(stdin: ChildStdin, branch: &str, dir: P) -> Import
        where P: AsRef<Path>
    {
        Import {
            stdin: stdin,
            branch: branch.to_owned(),
            dir: dir.as_ref().to_owned(),
        }
    }

    pub fn import(&mut self) -> Result<()> {
        try!(self.start_commit());

        for entry in WalkDir::new(&self.dir) {
            let entry = entry.unwrap(); // TODO: Clean up unwrap
            if entry.metadata().unwrap().is_file() {
                // TODO: Should make this a trace log and not a just a println
                println!("adding {}", entry.path().display());
                self.add_file(entry.path());
            }
        }

        self.stdin.write("\n".as_bytes());

        Ok(())
    }

    fn start_commit(&mut self) -> Result<()> {
        let name = try!(self.get_config("user.name"));
        let email = try!(self.get_config("user.email"));
        let branch = self.branch.clone(); // TODO: find way to clean up clone

        self.write(&format!("commit refs/heads/{}\n", branch));
        self.write(&format!("committer {} <{}> now\n",
                            &*name.replace("\n", ""),
                            &*email.replace("\n", "")));

        let message = "Message for days.";
        self.write(&format!("data {}\n{}\n", message.len(), message));

        if let Ok(rev) = self.get_prev_commit() {
            if rev.len() > 0 {
                try!(self.write(&format!("from {}\n", &*rev.replace("\n", ""))));
            }
        }

        try!(self.write("deleteall\n"));

        Ok(())
    }

    fn add_file<P>(&mut self, filename: P) -> Result<()>
        where P: AsRef<Path>
    {
        // TODO: Clean up this method
        let filename = filename.as_ref();


        let dir = self.dir.clone();
        let filename_rel = match filename.strip_prefix(&dir) {
            Ok(path) => path,
            _ => filename,
        };

        let filename_str = match filename_rel.to_str() {
            Some(name) => name,
            None => return Err(Error::from("could not convert string to utf8")),
        };

        // TODO: need to allow the ability for executable files to be passed
        try!(self.write(&format!("M 100644 inline {}\n", filename_str)));

        let mut file = File::open(filename).unwrap();

        let len = file.metadata().unwrap().len();
        try!(self.write(&format!("data {}\n", len)));

        let mut bytes = vec![0u8; len as usize];
        try!(file.read(&mut bytes));
        try!(self.stdin.write(&mut bytes));

        try!(self.write(&format!("\n")));

        Ok(())
    }

    fn get_prev_commit(&self) -> Result<String> {
        let output = try!(Command::new("git")
                              .current_dir(&self.dir)
                              .arg("rev-list")
                              .arg("--max-count=1")
                              .arg(&self.branch)
                              .arg("--")
                              .output());

        Ok(try!(String::from_utf8(output.stdout)))
    }

    fn write(&mut self, val: &str) -> Result<usize> {
        Ok(try!(self.stdin.write(&val.as_bytes())))
    }

    fn get_config(&self, key: &str) -> Result<String> {
        let output = try!(Command::new("git")
                              .arg("config")
                              .arg(key)
                              .current_dir(&self.dir)
                              .output());

        Ok(try!(String::from_utf8(output.stdout)))
    }
}
