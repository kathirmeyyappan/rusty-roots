use std::path::Path;

use colored::{ColoredString, Colorize, CustomColor};
use mime::{APPLICATION, AUDIO, IMAGE, TEXT, VIDEO};
use mime_guess::MimeGuess;

pub fn get_path_end(p: &Path) -> &str {
    p.file_name().and_then(|name| name.to_str()).unwrap_or(".")
}

pub fn color_path(p: &Path) -> ColoredString {
    let name = get_path_end(p);
    if p.is_dir() {
        return name.blue().bold();
    }
    let filetype = MimeGuess::from_path(p).first();

    match filetype {
        Some(mime) => match mime.type_() {
            TEXT => name.yellow(),
            IMAGE => name.bright_magenta(),
            AUDIO => name.cyan(),
            VIDEO => name.bright_magenta(),
            APPLICATION => match mime.subtype().as_str() {
                "json" => name.yellow(),
                "pdf" => name.bright_red(),
                "zip" => name.bright_green().bold(),
                _ => name.bright_green(),
            },
            _ => name.custom_color(CustomColor::new(137, 207, 240)),
        },
        None => name.custom_color(CustomColor::new(137, 207, 240)),
    }
}

pub fn color_branch(branch_str: &str, d: u8, max_d: u8) -> ColoredString {
    let (max_r, max_g, max_b) = (255_u8, 255_u8, 255_u8);
    let (min_r, min_g, min_b) = (255_u8, 120_u8, 0_u8);
    // println!("{}, {}", d, max_d);
    let (r, g, b) = (
        ((min_r as f32 * d as f32 + max_r as f32 * (max_d - d) as f32) / max_d as f32) as u8,
        ((min_g as f32 * d as f32 + max_g as f32 * (max_d - d) as f32) / max_d as f32) as u8,
        ((min_b as f32 * d as f32 + max_b as f32 * (max_d - d) as f32) / max_d as f32) as u8,
    );
    branch_str.custom_color(CustomColor::new(r, g, b))
}
