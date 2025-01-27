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
  -y, --yaml               Output as yaml
  -j, --json               Output as json
  -s, --short              Outputs only a short overview
  -o, --omit-license-text  Omits outputing license text
  -l, --license            Outputs license information regarding this software and it's dependencies
  -h, --help               Print help
  -V, --version            Print version
```

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


