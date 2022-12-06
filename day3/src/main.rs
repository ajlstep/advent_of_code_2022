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
    let mut sum = 0;
    for line in get_buffer(file_name).lines() {
        let ln = line.expect("get line err");
        let vec_equals = find_equals(split_str(&ln[..]));
        sum += count_sum(vec_equals);
    }
    println!("first part: {}", sum);
}

fn second_part(file_name: &str) {
    let mut sum = 0;
    let mut vc: Vec<String> = Vec::new();
    for line in get_buffer(file_name).lines() {
        let ln = line.expect("get line err");
        vc.push(ln);
    }
    let vc2 = vc.clone();
    let mut s = 0;
    while s < vc.len() {
        let vecs:Vec<&str> = vec![vc2[s].as_ref(), vc2[s+1].as_ref(), vc2[s+2].as_ref()];
        sum += count_sum(find_equals(vecs));
        s+=3;
    }
    println!("first part: {}", sum);
}

fn split_str(arg: &str) -> Vec<&str> {
    vec![&arg[0.. (arg.len()) / 2], &arg[(arg.len()) / 2..]]
}

fn find_equals(arg: Vec<&str>) -> Vec<char> {
    let mut ret: Vec<char> = vec![];
    if arg.len() < 2 {
        println!("err");
        return ret;
    }
    let vc0 = arg[0];
    let vecv = arg[1..].to_vec().clone();
    let vc0_chars: Vec<char> = vc0.chars().collect();
    for ch in vc0_chars {
        let mut add = true;
        for elaux in vecv.clone() {
            if !elaux.contains(ch) {
                add = false;
                // break;
            }
        }
        if add {
            if !ret.contains(&ch) {
                ret.push(ch.clone());    
            }
        }
    }
    // println!("{:?}", ret.clone());
    ret.to_vec()
}

fn count_sum(vec: Vec<char>) -> i32 {
    let mut s = 0;
    for el in vec {
        s += char_to_int(el);
    }
    s
}

fn char_to_int(ch: char) -> i32 {
    let i = ch as i32;
    let s= if i > 95 {
        i-96
    } else {
        i-38
    };
    s
}

#[cfg(test)]
mod tests {
    use crate::split_str;
    use crate::find_equals;
    use crate::char_to_int;
   
    #[test]
    fn test1() {
            let str1 = "asHMdfppqweaNMY"; 
            assert_eq!(find_equals(split_str(str1)), vec!['a', 'M', 'p']);
            assert_eq!(find_equals(split_str("vJrwpWtwJgWrhcsFMMfFFhFp")), vec!['p']);
            assert_eq!(find_equals(split_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")), vec!['L']);
            assert_eq!(find_equals(split_str("PmmdzqPrVvPwwTWBwg")), vec!['P']);
            assert_eq!(find_equals(split_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")), vec!['v']);
            assert_eq!(find_equals(split_str("ttgJtRGJQctTZtZT")), vec!['t']);
            assert_eq!(find_equals(split_str("CrZsJsPPZsGzwwsLwLmpwMDw")), vec!['s']);
            assert_eq!(char_to_int('a'), 1);
            assert_eq!(char_to_int('z'), 26);
            assert_eq!(char_to_int('A'), 27);
            assert_eq!(char_to_int('Z'), 52);
    }
}
