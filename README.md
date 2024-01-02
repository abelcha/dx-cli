# Accelerated Folder Sizing with Finder Cache in Rust and Objective C

## Overview
dx leverages the macOS Finder cache to rapidly compute folder sizes. Built in Rust, this utility integrates Objective C AppleScript via Foreign Function Interface (FFI), offering a significant performance boost. Experience folder sizing at speeds up to 20x faster than traditional methods like `du -sh`.

## Features
- **Efficient Sizing**: Utilizes macOS Finder cache for ultra-fast folder size computation.
- **Rust and Objective C Integration**: Seamlessly integrates Rust with Objective C AppleScript using FFI.
- **Performance**: Outperforms `du -sh` by up to 20x in speed.
- **User-Friendly Defaults**: Provides more intuitive defaults compared to traditional methods.

## Installation
```sh
cargo install dx-cli
```

## Usage
sane defaults:
- `dx dir` is equivalent to `du -sh dir`
- `dx dir -l` functions like `du -h dir`

## Contributing
Contributions to dx are welcome! Whether it's bug reports, feature suggestions, or code contributions, your input is valuable. See [CONTRIBUTING.md](link-to-your-contributing-guidelines) for guidelines on how to contribute.

## License
(Include information about your project's license here.)
