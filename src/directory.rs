use std::cmp::max;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use globset::GlobSet;

use crate::ignore::load_ignore_patterns;
use crate::text_fmt::{color_branch, color_path};

type DirectoryBody = HashMap<PathBuf, Vec<PathBuf>>;

const WALL: &str = "│   ";
const MIDDLE_CHILD: &str = "├── ";
const LAST_CHILD: &str = "└── ";
const SPACE: &str = "    ";

#[derive(Debug)]
pub struct Directory {
    root: PathBuf,
    body: DirectoryBody,
    max_depth: u8,
}

impl Directory {
    pub fn new(root: &Path, ignore: bool) -> Result<Self, Error> {
        let mut res = Self {
            root: root.to_path_buf(),
            body: HashMap::new(),
            max_depth: 0,
        };
        let ignore_patterns = if ignore {
            load_ignore_patterns(root.to_path_buf())?
        } else {
            GlobSet::empty()
        };
        res.walk_dir(root, &ignore_patterns, 1)?;
        Ok(res)
    }

    /// Populates the Directory object by recursively walking filepaths
    fn walk_dir(&mut self, from: &Path, ignore_patterns: &GlobSet, depth: u8) -> Result<(), Error> {
        if !from.to_path_buf().is_dir() {
            let err_msg = format!("{} is not a directory", from.display());
            return Err(Error::new(ErrorKind::InvalidInput, err_msg));
        }
        self.max_depth = max(self.max_depth, depth);

        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();
            // add info to directory body
            // let trimmed_path = path.to_str().unwrap().trim_start_matches("./");
            if !ignore_patterns.is_match(&path) {
                match self.body.get_mut(from) {
                    Some(v) => v.push(path.clone()),
                    None => {
                        self.body.insert(from.to_path_buf(), vec![path.clone()]);
                    }
                }
                // dfs if path is directory
                if path.is_dir() {
                    self.walk_dir(&path, ignore_patterns, depth + 1)?;
                }
            }
        }
        return Ok(());
    }

    /// Wrapper function to print Directory body's structure.
    pub fn print_body(&self, color: bool) -> Result<(), Error> {
        self.print_dir(None, &mut Vec::new(), None, color)?;
        println!();
        Ok(())
    }

    /// Takes a dfs approach to printing the directory structure.
    /// For each iteration we print the current path, then alphabetically print children.
    fn print_dir(
        &self,
        cur: Option<&Path>,
        wall_list: &mut Vec<bool>,
        pos: Option<&str>,
        color: bool,
    ) -> Result<(), Error> {
        let cur = cur.unwrap_or(&self.root);
        let pos = pos.unwrap_or(LAST_CHILD);

        // print line for given entry
        println!();
        for (i, w) in wall_list.iter().enumerate() {
            if *w {
                print!("{}", color_branch(WALL, color, i as u8, self.max_depth));
            } else {
                print!("{}", color_branch(SPACE, color, i as u8, self.max_depth))
            }
        }
        print!(
            "{}",
            color_branch(pos, color, wall_list.len() as u8, self.max_depth)
        );
        print!("{}", color_path(cur, color));

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
            self.print_dir(Some(&child), wall_list, Some(child_pos), color)?;
        }

        // update wall list for next call (rm from stack)
        wall_list.pop();

        Ok(())
    }
}
