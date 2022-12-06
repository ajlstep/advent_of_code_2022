use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    first_part("input.txt");
    second_part("input.txt");
}

fn get_buffer(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("file not found!");
    BufReader::new(file)
}

fn first_part(file_name: &str) {
    for line in get_buffer(file_name).lines() {
        let ln = line.expect("get line err");
    }
}

fn second_part(file_name: &str) {
    for line in get_buffer(file_name).lines() {
        let ln = line.expect("get line err");
    }
}





