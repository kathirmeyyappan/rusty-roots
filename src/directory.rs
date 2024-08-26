use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::text_fmt::color_path;

type DirectoryBody = HashMap<PathBuf, Vec<PathBuf>>;

const WALL: &str = "│   ";
const MIDDLE_CHILD: &str = "├── ";
const LAST_CHILD: &str = "└── ";
const SPACE: &str = "    ";

#[derive(Debug)]
pub struct Directory {
    root: PathBuf,
    body: DirectoryBody,
}

impl Directory {
    pub fn new(root: &Path) -> Result<Self, Error> {
        let mut res = Self {
            root: root.to_path_buf(),
            body: HashMap::new(),
        };
        res.walk_dir(root)?;
        Ok(res)
    }

    /// populates the Directory object by recursively walking filepaths
    fn walk_dir(&mut self, from: &Path) -> Result<(), Error> {
        if !from.to_path_buf().is_dir() {
            let err_msg = format!("{} is not a directory", from.display());
            return Err(Error::new(ErrorKind::InvalidInput, err_msg));
        }

        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();
            // add info to directory body
            match self.body.get_mut(from) {
                Some(v) => v.push(path.clone()),
                None => {
                    self.body.insert(from.to_path_buf(), vec![path.clone()]);
                }
            }
            // dfs if path is directory
            if path.is_dir() {
                self.walk_dir(&path)?;
            }
        }
        return Ok(());
    }

    /// wrapper function to print Directory body's structure.
    pub fn print_body(&self) -> Result<(), Error> {
        self.print_dir(None, &mut Vec::new(), None)?;
        println!();
        Ok(())
    }

    /// takes a dfs approach to printing the directory structure.
    /// for each iteration, prints only the parents in the current path.
    fn print_dir(
        &self,
        cur: Option<&Path>,
        wall_list: &mut Vec<bool>,
        pos: Option<&str>,
    ) -> Result<(), Error> {
        let cur = cur.unwrap_or(&self.root);
        let pos = pos.unwrap_or(LAST_CHILD);

        // print line for given entry
        println!();
        for w in wall_list.iter() {
            if *w {
                print!("{}", WALL);
            } else {
                print!("{}", SPACE)
            }
        }
        print!("{}", pos);
        print!("{}", color_path(cur));

        // update wall list for next call (add to stack)
        wall_list.push(pos != LAST_CHILD);

        // dfs
        let mut children = self.body.get(cur).unwrap_or(&Vec::new()).clone();
        children.sort_by(|a, b| a.to_str().unwrap().cmp(b.to_str().unwrap()));
        for (i, child) in children.iter().enumerate() {
            let child_pos = if i == children.len() - 1 {
                LAST_CHILD
            } else {
                MIDDLE_CHILD
            };
            self.print_dir(Some(&child), wall_list, Some(child_pos))?;
        }

        // update wall list for next call (rm from stack)
        wall_list.pop();

        Ok(())
    }
}
