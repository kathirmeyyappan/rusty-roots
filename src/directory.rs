use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

type DirectoryBody = HashMap<PathBuf, Vec<PathBuf>>;

const WALL: &str = "│   ";
const MIDDLE_CHILD: &str = "├—— ";
const LAST_CHILD: &str = "└—— ";

#[derive(Debug)]
pub struct Directory {
    root: PathBuf,
    body: DirectoryBody
}

impl Directory {

    pub fn new(root: &Path) -> Result<Self, Error> {
        let mut res = Self {
            root: root.to_path_buf(),
            body: HashMap::new()
        };
        res.walk_dir(root)?;
        Ok(res)
    }

    fn walk_dir(&mut self, from: &Path) -> Result<(), Error> {
        if !from.to_path_buf().is_dir() {
            let err_msg = stringify!("{} is not a directory", from.to_str());
            return Err(Error::new(ErrorKind::InvalidInput, err_msg));
        }
        
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();
            match self.body.get_mut(from) {
                Some(v) => v.push(path.clone()),
                None => { self.body.insert(from.to_path_buf(), vec![path.clone()]); }
            }
            if path.is_dir() {
                self.walk_dir(&path)?;
            } 
        }
        return Ok(())
    }

    pub fn print_dir(&self) -> Result<(), Error> {

    }
}
