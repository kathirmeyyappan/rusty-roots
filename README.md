# [rusty-roots](https://crates.io/crates/rusty-roots)
![crates.io](https://img.shields.io/crates/v/rusty-roots.svg)
![](https://img.shields.io/crates/d/rusty-roots)

## Overview

-  CLI tool that displays directory structure of a specified path in tree-like format, similar to the `tree` Unix command.
- Allows customized output by ignoring certain files and directories (see `.rrignore` file!).
- Supports colorized output (file-type-based coloring, tree depth gradients, etc.).

## Pics
<div style="display: flex; align-items: center; justify-content: center;">
  <img src="https://github.com/user-attachments/assets/bde39693-4a22-415c-aac0-42eb9470d173" alt="Image 1" style="height: 300px; margin-right: 10px;">
  <img src="https://github.com/user-attachments/assets/026fb592-005d-4453-a4e7-91b3c8c6d056" alt="Image 3" style="height: 300px; margin-right: 10px;">
  <img src="https://github.com/user-attachments/assets/43d3a049-0175-4cbd-bd6e-bb4a3080938b" alt="Image 4" style="height: 300px; margin-right: 10px;">
  <img src="https://github.com/user-attachments/assets/03f90277-b603-49bc-ba77-be650f0ffcf0" alt="Image 2" style="height: 300px; margin-right: 10px;">
</div>

## Demo
Here is a short example video showing rusty-roots in action:

https://github.com/user-attachments/assets/9689ad2f-937b-4b58-a9a9-217c85ccd2e3

## Installation
### Prerequisites
1. Ensure Rust is installed. Visit: https://www.rust-lang.org/tools/install.
### Steps
1. Install rusty-roots:
```
cargo install rusty-roots
```
2. Run rusty-roots in some directory (with or without extra args, as specified below):
```
rusty-roots
```

## Options
```
Usage: rusty-roots [OPTIONS]

Options:
  -p, --path <PATH>  Path to get tree from
  -i, --ignore       Ignore files and directories as specified in {$path}/.rrignore
  -f, --fast-print   Print directory on the fly without pre-calculation
  -h, --help         Print help
  -V, --version      Print version
```

*tip: If the program seems to be hanging because the called directory is extremely large, try running with `-f`. Pre-calculation for the branch color gradient is not done when this flag is called, so the program will be able to print the tree on-the-fly.* 
