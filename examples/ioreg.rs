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
