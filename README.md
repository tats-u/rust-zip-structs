# Rust crates for ZIP structures (zip_structs)

[![CI (master)](<https://github.com/tats-u/rust-zip-structs/workflows/CI%20(master)/badge.svg>)](https://github.com/tats-u/rust-zip-structs/actions?query=workflow%3A%22CI+%28master%29%22)
[![CI (Release)](<https://github.com/tats-u/rust-zip-structs/workflows/CI%20(Release)/badge.svg>)](https://github.com/tats-u/rust-zip-structs/actions?query=workflow%3A%22CI+%28Release%29%22)
[![zip_structs at crates.io](https://img.shields.io/crates/v/zip_structs.svg)](https://crates.io/crates/zip_structs)
[![zip_structs at docs.rs](https://docs.rs/zip_structs/badge.svg)](https://docs.rs/zip_structs/)

This crates handles structures in ZIP files.

- End of central directory (EOCD) structure
- Central directory structure
- Local file header structure
  - Data descriptor structure

## Installation

Add the following line to `[dependencies]` in your `Cargo.toml`.

```toml
zip_structs = "^0.2"
```

Breaking changes to the API are planned for the future.  Therefore, it is strongly recommended to use this versioning format.

## Basic usage of parsing from the EOCD sturcture of a ZIP archive

```rust
use std::io::BufReader;
use std::fs::File;
use oem_cp::decode_string_complete_table;
use oem_cp::code_table::DECODING_TABLE_CP437;

use zip_structs::zip_central_directory::ZipCDEntry;
use zip_structs::zip_eocd::ZipEOCD;
use zip_structs::zip_local_file_header::ZipLocalFileHeader;

let mut zip_file = BufReader(File::open("path/to/archive.zip")?);

let eocd = ZipEOCD::from_reader(&mut zip_file)?;
let cd_list = ZipCDEntry::all_from_eocd(&mut zip_file, &eocd)?;

// Show file names in the ZIP archive
for cd in &cd_list {
    println!(
        "{}",
        if cd.is_encoded_in_utf8() {
            String::from_utf8_lossy(&cd.file_name_raw);
        } else {
            decode_string_complete_table(&cd.file_name_raw, DECODING_TABLE_CP437)
        }
    );
    let local_file_header = ZipLocalFileHeader::from_central_directory(&mut zip_file, &cd)?;
    do_something(&local_file_header);
}
```

## Competing libraries

There are some libraries providing more abstract and higher-level APIs.

- `zip`
- `rc-zip`

These libraries do not handle general purpose bit flags in central directories and local file headers.  `rc-zip` cannot handle file names encoded in other than UTF-8.

- `vfs-zip`

This library provides virtual file system, not structures in ZIP archives.

## Support of ZIP64

This library has not supported ZIP64 yet.

## License

MIT

[See LICENSE](./LICENSE)
