use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, ChildStdin};
use std::io::Write;
use error::{Result, Error};

pub fn import_dir<P>(dir: P, branch: &str) -> Result<()>
    where P: AsRef<Path> + Clone
{
    println!("{:?} to {:?}", dir.as_ref(), branch);

    let mut cmd = try!(Command::new("git")
                           .arg("fast-import")
                           .arg("--date-format=raw")
                           .arg("--quiet")
                           .stdin(Stdio::piped())
                           .current_dir(&dir)
                           .spawn());


    let mut stdin = match cmd.stdin.take() {
        Some(buf) => buf,
        None => return Err(Error::from("did not capture stdin")),
    };

    let import = Import::new(stdin, &dir);

    try!(cmd.wait());

    Ok(())
}

struct Import {
    stdin: ChildStdin,
    dir: PathBuf, // TODO: Should prob make this just a reference
}

impl Import {
    fn new<P>(stdin: ChildStdin, dir: P) -> Import
        where P: AsRef<Path>
    {
        Import {
            stdin: stdin,
            dir: dir.as_ref().to_owned(),
        }
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
