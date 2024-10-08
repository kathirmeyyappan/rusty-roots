# rusty-roots
-  CLI tool that displays directory structure of a specified path in tree-like format, similar to the `tree` Unix command.
- Allows customized output by ignoring certain files and directories (see `.rrignore` file!).
- Supports colorized output (file-type-based coloring, tree depth gradients, etc.).

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
