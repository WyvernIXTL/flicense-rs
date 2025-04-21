<div align="center">

# `flicense`
**CLI for printing license information of rust cargo projects to the terminal.**

[![Crates.io Version](https://badgen.net/crates/v/flicense)](https://crates.io/crates/flicense)
[![GitHub License](https://badgen.net/github/license/WyvernIXTL/flicense-rs)](https://github.com/WyvernIXTL/flicense-rs/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/WyvernIXTL/flicense-rs/status.svg)](https://deps.rs/repo/github/WyvernIXTL/flicense-rs)

</div>

```
CLI for printing license information of rust cargo projects to the terminal.

Usage: flicense.exe [OPTIONS] [MANIFEST_DIR_PATH]

Arguments:
  [MANIFEST_DIR_PATH]  Optional path to manifest dir (where Cargo.toml and Cargo.lock are). Defaults to current dir

Options:
  -y, --yaml               Output as yaml
  -j, --json               Output as json
  -s, --short              Outputs only a short overview
  -o, --omit-license-text  Omits outputting license text
  -l, --license            Outputs license information regarding this software and it's dependencies
  -h, --help               Print help
  -V, --version            Print version
```

## Installation

### [Scoop](https://scoop.sh/) (Windows)

```sh
scoop bucket add stupid-bucket https://github.com/WyvernIXTL/stupid-bucket
scoop install stupid-bucket/flicense
```

### [Cargo Binstall](https://github.com/cargo-bins/cargo-binstall) (Windows/Linux/MacOS)

```sh
cargo binstall -y flicense
```

### From Source

```sh
cargo install flicense
```


## Usage

### Prerequisite

1. Have [Cargo](https://github.com/rust-lang/cargo) installed.
2. Ensure the dependencies of the project for which you want to fetch licenses are downloaded (e.g., using `cargo fetch`).


## Examples:

### Without Any Flag

```
flicense.exe ..\license-fetcher\`
================================================================================

Package:     license-fetcher 0.6.2
Description: Fetch licenses of dependencies at build time and embed them into your program.
Authors:     - Adam McKellar <dev@mckellar.eu>
Repository:  https://github.com/WyvernIXTL/license-fetcher
SPDX Ident:  BSL-1.0

--------------------------------------------------------------------------------
Copyright Adam McKellar 2024

Boost Software License - Version 1.0 - August 17th, 2003

...
```

### YAML Without License Text

```
flicense.exe .\license-fetcher\ -o -y`
- name: license-fetcher
  version: '0.6.2'
  authors:
  - Adam McKellar <dev@mckellar.eu>
  description: Fetch licenses of dependencies at build time and embed them into your program.
  homepage: null
  repository: https://github.com/WyvernIXTL/license-fetcher
  license_identifier: BSL-1.0
  license_text: null

...
```

### Short License Overview
```
flicense.exe .\license-fetcher\ -s
MIT OR Zlib OR Apache-2.0: miniz_oxide
MIT: bincode, bincode_derive, virtue
0BSD OR MIT OR Apache-2.0: adler2
BSL-1.0: license-fetcher
```


