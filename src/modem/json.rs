use plist::{Value};
use seek_bufread::BufReader;
use serde::ser::Serialize;
use serde_json as json;
use serde_json::ser::CompactFormatter as CompactFormatterJSON;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{Read, Write};
use std::path::Path;


pub fn transcode_from_xml_reader<R: Read, W: Write>(input: BufReader<R>, output: BufWriter<W>) {
    let value = Value::from_reader_xml(input).expect("failed to read plist");
    value
        .serialize(&mut json::Serializer::with_formatter(
            output,
            CompactFormatterJSON,
        ))
        .unwrap();
}
pub fn transcode_xml_to_json_file(source_path: String, destination_path: String) {
    let stdin = OpenOptions::new()
        .read(true)
        .open(Path::new(&source_path))
        .unwrap();
    let input = BufReader::new(stdin);

    let stdout = OpenOptions::new()
        .write(true)
        .open(Path::new(&destination_path))
        .unwrap();

    let output = BufWriter::new(stdout);

    transcode_from_xml_reader(input, output)
}

// ...TK
// use plist::{Value, XmlWriteOptions, Dictionary};
// fn json_to_plist_xml<R: Read, W: Write>(input: BufReader<R>, output: BufWriter<W>) {
//     let json_value = serde_json::from_reader(input).unwrap();
//     match *json_value {

//     }
//     let plist_value = Dictionary
//     let options = XmlWriteOptions::default().indent(32, 2);
//     value
//         .to_writer_xml_with_options(BufWriter::new(&mut output), &options)
//         .unwrap();

//     value
//         .serialize(&mut json::Serializer::with_formatter(
//             output,
//             CompactFormatterJSON,
//         ))
//         .unwrap();
// }
