use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() -> std::io::Result<()> {
    let mut vc = vec![0,0,0];
    vc.remove(0);
    println!("{:?}", vc[0]);
    println!("{:?}", vc);

    first_part();
    second_part();
    Ok(())
}

fn first_part() {
    let mut match_big = 0;
    let mut now = 0;
    for line in get_buffer("input1.txt").lines() {
        let ln = line.expect("get line err");
        if ln == "".to_string() {
            if now > match_big {
                match_big = now;
            } 
            now = 0;
            continue;
    }
        let numb = ln.parse::<i32>().expect("parse to i32 err");
        now += numb;
    }
    println!("bigest of 1:  {}", match_big);
}

fn second_part() {
    let mut vc = vec![0,0,0];
    // let mut match_big = 0;
    let mut now = 0;
    for line in get_buffer("input2.txt").lines() {
        let ln = line.expect("get line err");
        if ln == "".to_string() {
            let mut is_biggest = false;
            for n in vc.iter() {
                if now > *n {
                    is_biggest = true;
                    break;
                }
            }
            if is_biggest {
                vc.remove(0);
                vc.push(now);
                vc.sort();
            }
            // let tot = vc.iter().sum();
            now = 0;
            continue;
    }
        let numb = ln.parse::<i32>().expect("parse to i32 err");
        now += numb;
    }
    let s: i32 = vc.iter().sum();
    println!("bigest of 3:  {}",  s);
}

fn get_buffer(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("file not found!");
    BufReader::new(file)
}