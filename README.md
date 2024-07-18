


_originaly made for [eza](https://github.com/eza-community/eza) `--total-size`, but cant make it build cross-plarform, so it just work standalone on mac m1 as of now_


`du` vs `dx`:
![output](https://github.com/abelcha/dx-cli/assets/6186996/9f5f01de-dae6-4e02-a706-15c24c3fffa3)


# Intro

How can finder display dir sizes instantly, but in userland it takes forever to walk through the file tree?

Its ovbious macos caches dir sizes somewhere but [Apple dont expose any api or document this](https://developer.apple.com/documentation/foundation)


It's only accessible through AppleScript's `getinfo` methods, which I've made the first version with. However, it's slow and requires spawning a new process for each call. so this is what i came up with:

## Features

- **libfffs**: An attempt to reverse-engineer the underlying system call.
- **dx-cli**: A wrapper around libfffs to provide a `du -sh`-type interface and Rust FFI bindings.

## Strategies

The system provides three strategies that will fallback the following order by default:

1. **AEV (AppleEvents)**: A long-forgotten IPC protocol using C with Pascal strings. It works well but sometimes fails on concurrent calls.
2. **Dstore**: `.ds_store` are binary dumps of Finder's internal database. Finder regenerates it even if the AEV call fails, so the combination of the two is surprisingly reliable. 

_Few parser attempts have been made since [the OG chad that first reverse-engineered it](https://0day.work/parsing-the-ds_store-file-format/), but on some more recent osx version it fails attempting to parse the entire B-tree_

_I've made a simpler implementation that only focuses on 1st level size and dates metadata and works on every `.ds_store` found in GitHub Code Search._
_its available in the lib as  `dstore_parser`_

3. **Live**: Falls back to a classic recursive walkthrough of the file tree, which is safe and reliable but slow.

4. **Osa**: (legacy) Original applescript (osascript) implementation, still here for perfs benchmarking


## Usage

```bash
dx 0.3.1

USAGE:
    dx [FLAGS] [OPTIONS] [--] [PATH]...

FLAGS:
    -b, --bytes      Bytes
        --dironly    directory only
    -h, --help       Prints help information
    -l, --list       List
    -p, --perf       Performance
        --sort       Sort by size
        --trace      Traces
    -V, --version    Prints version information
    -v, --verbose

OPTIONS:
        --color <color>              [default: auto]  [possible values: auto, always, never]
    -s, --strategy <strategy>...     [possible values: aev, dstore, live, osa]

ARGS:
    <PATH>...    Paths [default: ./]
```

# with options kitchen sink:

![Screenshot 2024-07-18 at 05 31 20](https://github.com/user-attachments/assets/cebc0846-7385-43d5-843c-1ec5200579de)
