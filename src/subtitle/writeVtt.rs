use std::fs::File;
use std::io::Write;

pub fn write_vtt(path: &str, blocks: &Vec<String>) {
    let mut file = File::create(path).expect("Cannot create file");
    writeln!(file, "WEBVTT\n").unwrap();

    for block in blocks {
        writeln!(file, "{}\n", block).unwrap();
    }
}
