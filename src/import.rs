use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ChildStdin};
use std::io::{Write, Read};
use error::{Result, Error};
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use walkdir::WalkDir;

pub fn import_dir<P>(dir: P, branch: &str, message: &str) -> Result<()>
    where P: AsRef<Path> + Clone
{
    // println!("{:?} to {:?}", dir.as_ref(), branch);

    let mut cmd = try!(Command::new("git")
                           .arg("fast-import")
                           .arg("--date-format=now")
                           .arg("--quiet")
                           .stdin(Stdio::piped())
                           .spawn());


    let stdin = match cmd.stdin.take() {
        Some(buf) => buf,
        None => return Err(Error::from("did not capture stdin")),
    };

    try!(Import::new(stdin, branch, &dir, message).import());


    try!(cmd.kill());

    Ok(())
}

struct Import {
    stdin: ChildStdin,
    branch: String,
    message: String,
    dir: PathBuf,
}

impl Import {
    pub fn new<P>(stdin: ChildStdin, branch: &str, dir: P, message: &str) -> Import
        where P: AsRef<Path>
    {
        Import {
            stdin: stdin,
            branch: branch.to_owned(),
            message: message.to_owned(),
            dir: dir.as_ref().to_owned(),
        }
    }

    pub fn import(&mut self) -> Result<()> {
        // TODO: Check if in git repo

        try!(self.start_commit());

        for entry in WalkDir::new(&self.dir) {
            let entry = entry.unwrap(); // TODO: Clean up unwrap
            if entry.metadata().unwrap().is_file() {
                // TODO: Should make this a trace log and not a just a println
                // println!("adding {}", entry.path().display());
                try!(self.add_file(entry.path()));
            }
        }

        try!(self.stdin.write("\n".as_bytes()));

        Ok(())
    }

    fn start_commit(&mut self) -> Result<()> {
        let name = try!(self.get_config("user.name"));
        let email = try!(self.get_config("user.email"));
        let branch = self.branch.clone();
        let message = self.message.clone();

        try!(self.write(&format!("commit refs/heads/{}\n", branch)));
        try!(self.write(&format!("committer {} <{}> now\n",
                                 &*name.replace("\n", ""),
                                 &*email.replace("\n", ""))));

        try!(self.write(&format!("data {}\n{}\n", message.len(), message)));

        if let Ok(rev) = self.get_prev_commit() {
            if rev.len() > 4 {
                try!(self.write(&format!("from {}\n", &*rev.replace("\n", ""))));
            }
        }

        try!(self.write("deleteall\n"));

        Ok(())
    }

    fn add_file<P>(&mut self, filename: P) -> Result<()>
        where P: AsRef<Path>
    {
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

        let mut file = try!(File::open(filename));
        let metadata = try!(file.metadata());
        let permissions = metadata.permissions();

        if permissions.mode() & 0o700 == 0o700 {
            try!(self.write(&format!("M 100755 inline {}\n", filename_str)));
        } else {
            try!(self.write(&format!("M 100644 inline {}\n", filename_str)));
        }

        try!(self.write(&format!("data {}\n", metadata.len())));

        let bytes = {
            let mut bytes = vec![0u8; metadata.len() as usize];
            try!(file.read(&mut bytes));
            bytes
        };

        try!(self.stdin.write(&bytes));

        try!(self.write("\n"));

        Ok(())
    }

    fn get_prev_commit(&self) -> Result<String> {
        let output = try!(Command::new("git")
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
