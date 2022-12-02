use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    first_part("input1.txt");
}

fn first_part(file_name: &str) {
    let vc: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    for line in get_buffer(file_name).lines() {
        let ln = rep(line.expect("get line err"));

    }
}

fn rep(arg: String) -> String {
    // let ret: &mut String = &mut "".to_string();
    let ret = &mut arg.replace("X", "A");
    let ret = &mut ret.replace("Y", "B");
    let ret = &mut ret.replace("Z", "C");
    ret.to_string()
}


fn get_buffer(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("file not found!");
    BufReader::new(file)
}