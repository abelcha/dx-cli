# fast directory sizing on OSX, using reverse engineered finder cache api

## Overview

dx leverages the obscure macOS C Finder cache to rapidly get folder sizes. Built in Rust, this utility integrates Objective C via Foreign Function Interface (FFI). Experience folder sizing at speeds up to 20x faster than traditional methods like `du -sh`.

![output](https://github.com/abelcha/dx-cli/assets/6186996/9f5f01de-dae6-4e02-a706-15c24c3fffa3)


## Features
- **Efficient Sizing**: Utilizes macOS Finder cache for ultra-fast folder size computation.
- **Rust and Objective C Integration**: Seamlessly integrates Rust with Objective C using FFI.
- **Multiple Methods**: if you dont want any cache you can still run dx with --live and get a recursive sizing
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
