use std::path::Path;
use error::Result;

pub fn import_dir<P>(path: P, branch: &str) -> Result<()>
    where P: AsRef<Path>
{
    let path = path.as_ref();

    println!("{:?}", path);


    Ok(())
}
