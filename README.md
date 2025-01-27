<div align="center">

# `flicense`
**CLI for printing license information of rust cargo projects to the terminal.**

[![Crates.io Version](https://img.shields.io/crates/v/flicense)](https://crates.io/crates/flicense)
[![GitHub License](https://img.shields.io/github/license/WyvernIXTL/license-fetcher)](https://github.com/WyvernIXTL/license-fetcher/blob/main/LICENSE)
[![docs.rs](https://img.shields.io/docsrs/flicense)](https://docs.rs/flicense)
[![dependency status](https://deps.rs/repo/github/WyvernIXTL/flicense/status.svg)](https://deps.rs/repo/github/WyvernIXTL/flicense)

</div>

```
CLI for printing license information of rust cargo projects to the terminal.

Usage: flicense.exe [OPTIONS] [MANIFEST_DIR_PATH]

Arguments:
  [MANIFEST_DIR_PATH]  Optional path to manifest dir (where Cargo.toml and Cargo.lock are). Defaults to current dir

Options:
  -y, --yaml     Output as yaml
  -j, --json     Output as json
  -s, --short    Outputs only a short overview
  -l, --license  Outputs license information regarding this software and it's dependencies
  -h, --help     Print help
  -V, --version  Print version
```


