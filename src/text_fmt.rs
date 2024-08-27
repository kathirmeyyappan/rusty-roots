use std::path::Path;

use clap::builder::OsStr;
use colored::{ColoredString, Colorize};
use mime::{APPLICATION, AUDIO, IMAGE, TEXT, VIDEO};
use mime_guess::MimeGuess;

pub fn color_path(p: &Path) -> ColoredString {
    let cur_dir = OsStr::from(".");
    let name = p.file_name().unwrap_or(&cur_dir);
    let name = name.to_str().unwrap();
    if p.is_dir() {
        return name.blue().bold();
    }
    let filetype = MimeGuess::from_path(p).first();

    match filetype {
        Some(mime) => match mime.type_() {
            TEXT => name.yellow(),
            IMAGE => name.bright_magenta(),
            AUDIO => name.cyan(),
            VIDEO => name.red(),
            APPLICATION => match mime.subtype().as_str() {
                "json" => name.yellow(),
                "pdf" => name.bright_magenta(),
                "zip" => name.bright_green().bold(),
                _ => name.bright_green(),
            },
            _ => name.normal(),
        },
        None => name.normal(),
    }
}
