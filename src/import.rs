use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ChildStdin};
use std::io::{Write, Read};
use error::{Result, Error};
use std::fs::File;

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
    import.import();

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
        self.add_file("");
        Ok(())
    }

    fn add_file<P>(&mut self, file: P) -> Result<()>
        where P: AsRef<Path>
    {
        try!(self.write(&format!("M 100644 inline index.html\n")));
        let mut file = File::open("build/index.html").unwrap();

        let len = file.metadata().unwrap().len();
        try!(self.write(&format!("data {}\n", len)));

        let mut bytes = vec![0u8; len as usize];
        try!(file.read(&mut bytes));
        try!(self.stdin.write(&mut bytes));

        try!(self.write(&format!("\n")));

        Ok(())
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
