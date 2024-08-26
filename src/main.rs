mod directory;
mod text_fmt;

use std::path::Path;

use clap::{Arg, Command};
use directory::Directory;

fn main() {
    let matches = Command::new("rusty-roots")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Path to get tree from"),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("path");
    let target_path = match input_path {
        Some(s) => Path::new(s),
        None => Path::new("."),
    };

    let dir = Directory::new(target_path).unwrap();
    dir.print_body().unwrap();
}
