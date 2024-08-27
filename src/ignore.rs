use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::PathBuf;

use globset::{Glob, GlobSet, GlobSetBuilder};

/// Gets ignore patterns from .rrignore file in current directory.
/// File should follow .gitignore format.
pub fn load_ignore_patterns(mut p: PathBuf) -> Result<GlobSet, Error> {
    // find .rrignore file
    p.push(".rrignore");
    if !p.exists() {
        return Ok(GlobSet::empty());
    }
    let file = File::open(p)?;
    let reader = BufReader::new(file);

    // parse .rrignore file for glob rules
    // let mut patterns = Vec::new();
    let mut builder = GlobSetBuilder::new();
    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() && !line.starts_with('#') {
            // patterns.push(line.clone());
            if let Ok(glob) = Glob::new(&line.trim()) {
                builder.add(glob);
            }
        }
    }
    match builder.build() {
        Ok(res) => Ok(res),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e.to_string())),
    }
}
