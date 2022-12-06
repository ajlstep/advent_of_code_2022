use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // let s = vec!["X X", "X Y", "X Z", "Y X", "Y Y", "Y Z", "Z X", "Z Y", "Z Z"];
    // for el in s {
    //     let m = get_arr(el);
    //     let a = get_i32_to_string(m.1);
    //     let b = get_i32_to_string(m.0);
    //     let rez = (a - b + 1 + 3) % 3 * 3 + (a + 1);
    //     println!("{}", rez); 
    // }

    first_part("input1.txt");
    second_part("input1.txt");
}

fn first_part(file_name: &str) {
    let mut sum = 0;
    for line in get_buffer(file_name).lines() {
        let ln = rep(line.expect("get line err"));
        let m = get_arr(&ln[..]);
        let rez = ((get_i32_to_string(m.1) - get_i32_to_string(m.0) + 1 + 3) % 3 * 3) + (get_i32_to_string(m.1) + 1); //stealing
        sum += rez;
    }
    println!("first part: {}", sum);
}

fn second_part(file_name: &str) {
    let mut sum = 0;
    for line in get_buffer(file_name).lines() {
        let ln = rep(line.expect("get line err"));
        let m = get_arr(&ln[..]);
        sum += get_i32_to_string(m.1) * 3;
        sum += (get_i32_to_string(m.0) + (get_i32_to_string(m.1 ) - 1) + 3) % 3;
        // let rez = ((get_i32_to_string(m.1) - get_i32_to_string(m.0) + 1 + 3) % 3 * 3) + (get_i32_to_string(m.1) + 1); //stealing
        sum += 1;
    }
    println!("second part : {}", sum);
}

fn rep(arg: String) -> String {
    // let ret: &mut String = &mut "".to_string();
    let ret = &mut arg.replace("A", "X");
    let ret = &mut ret.replace("B", "Y");
    let ret = &mut ret.replace("C", "Z");
    ret.to_string()
}

fn get_buffer(file: &str) -> BufReader<File> {
    let file = File::open(file).expect("file not found!");
    BufReader::new(file)
}

fn get_i32_to_string(arg: &str) -> i32 {
    match arg {
       "X" => 0,
       "Y" => 1,
       "Z" => 2,
       _ => 0,
    }
}

fn get_arr(arg: &str) -> (&str, &str) {
    let m: Vec<&str> = arg.split(" ").collect();
    (m[0], m[1])
}

#[cfg(test)]
mod tests {
    // use crate::get_arr;
    use crate::get_i32_to_string;
    use crate::get_arr;
    
    #[test]
    fn cnb123() {
        // let s1 = "X X";
        // let s2 = "X Y";
        // let s3 = "X Z";
        // let s4 = "Y X";
        // let s5 = "Y Y";
        // let s5 = "Y Z";
        // let s5 = "Z X";
        // let s5 = "Z Y";
        // let s5 = "Z Z";

        let s = vec!["X X", "X Y", "X Z", "Y X", "Y Y", "Y Z", "Z X", "Z Y", "Z Z"];
        for el in s {
            let m = get_arr(el);
            let rez = (get_i32_to_string(m.1) - get_i32_to_string(m.0) + 1 + 3) % 3 + 3;
            println!("{}", rez); 
        }
    }
}