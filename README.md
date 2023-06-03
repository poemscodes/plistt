# Plistt &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [Rust 2021][Rust 1.67]

[Build Status]: https://img.shields.io/github/actions/workflow/status/poemscodes/plistt/ci.yml?branch=doctor
[actions]: https://github.com/poemscodes/plistt/actions?query=branch%3Adoctor
[Latest Version]: https://img.shields.io/crates/v/plistt.svg
[crates.io]: https://crates.io/crates/plistt
[Rust 1.67]: https://blog.rust-lang.org/2023/02/09/Rust-1.67.1.html

**Plistt a rust library and command-line tool to convert xml-encoded plist data into json.**

---

## Plistt in action

<details>
<summary>
Click to show Cargo.toml.
<a href="https://play.rust-lang.org/?edition=2018&gist=72755f28f99afc95e01d63174b28c1f5" target="_blank">Run this code in the playground.</a>
</summary>

```toml
[dependencies]

plistt = { version = "0.1.0" }
```

</details>
<p></p>

```rust
use plistt::json;
use plistt::{BufReader, BufWriter};
use std::fs::OpenOptions;
use std::io::{Cursor};
use std::process::Command;
use std::path::Path;

fn main() {
    let ioreg = Command::new("ioreg")
        .arg("-c")
        .arg("IOUSB")
        .arg("-a")
        .output()
        .unwrap();
    let input = BufReader::new(Cursor::new(ioreg.stdout));

    let stdout = OpenOptions::new()
        .write(true)
        .open(Path::new("/dev/stdout"))
        .unwrap();

    let output = BufWriter::new(stdout);

    json::transcode_from_xml_reader(input, output)
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Plistt by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
