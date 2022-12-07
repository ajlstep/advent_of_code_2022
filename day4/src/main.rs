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
    let mut sum: i32 = 0;
    for line in get_buffer(file_name).lines() {
        let ln = line.expect("get line err");
        // sum += increment_of_containts(ln);
        sum += increment_of_containts_prs(ln);
    }
    println!("first part: {}", sum);
}

fn second_part(file_name: &str) {
    let mut sum: i32 = 0;
    for line in get_buffer(file_name).lines() {
        let ln = line.expect("get line err");      
        sum += increment_of_include_prs(ln);
    }
    println!("second part: {}", sum);
}

#[allow(dead_code)]
fn increment_of_containts(arg: String) -> i32 {
    let arr: Vec<&str> = arg.split(&[',', '-'][..]).collect();
    if arr[0] <= arr[2] && arr[1] >= arr[3] || arr[2] <= arr[0] && arr[3] >= arr[1] {
        1
    } else {
        0
    }
}

fn increment_of_containts_prs(arg: String) -> i32 {
    let arr: Vec<&str> = arg.split(&[',', '-'][..]).collect();
    let l0: i32 = arr[0].parse::<i32>().expect("parse to i32 err");
    let l1: i32 = arr[1].parse::<i32>().expect("parse to i32 err");
    let l2: i32 = arr[2].parse::<i32>().expect("parse to i32 err");
    let l3: i32 = arr[3].parse::<i32>().expect("parse to i32 err");
    if l0 <= l2 && l1 >= l3 || l2 <= l0 && l3 >= l1 {
        1
    } else {
        0
    }
}

fn increment_of_include_prs(arg: String) -> i32 {
    let arr: Vec<&str> = arg.split(&[',', '-'][..]).collect();
    let l0: i32 = arr[0].parse::<i32>().expect("parse to i32 err");
    let l1: i32 = arr[1].parse::<i32>().expect("parse to i32 err");
    let l2: i32 = arr[2].parse::<i32>().expect("parse to i32 err");
    let l3: i32 = arr[3].parse::<i32>().expect("parse to i32 err");
    if (l0 == l2 || l0 == l3 || l1 == l2 || l1 == l3) || ((l0 < l2 && l1 > l2) || (l2 < l0 && l3> l0)) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::increment_of_containts;
    use crate::increment_of_containts_prs;
  
    #[test]
    fn test1() {
            assert_eq!(increment_of_containts("2-5,2-5".to_string()), 1);
            assert_eq!(increment_of_containts("1-5,2-5".to_string()), 1);
            assert_eq!(increment_of_containts("3-5,2-5".to_string()), 1);
            assert_eq!(increment_of_containts("2-6,2-5".to_string()), 1);
            assert_eq!(increment_of_containts("2-6,2-7".to_string()), 1);
            assert_eq!(increment_of_containts("2-6,1-7".to_string()), 1);
            let s = increment_of_containts("2-6,3-7".to_string());
            assert_eq!(s, 0);
            assert_eq!(increment_of_containts("3-6,4-7".to_string()), 0);
            assert_eq!(increment_of_containts("4-7,3-6".to_string()), 0);
            assert_eq!(increment_of_containts("1-2,3-3".to_string()), 0);
            assert_eq!(increment_of_containts("19-19,20-99".to_string()), 0);
            assert_eq!(increment_of_containts("1-20,50-50".to_string()), 0);
    }
    #[test]
    fn test2() {
        assert_eq!(increment_of_containts_prs("2-5,2-5".to_string()), 1);
        assert_eq!(increment_of_containts_prs("1-5,2-5".to_string()), 1);
        assert_eq!(increment_of_containts_prs("3-5,2-5".to_string()), 1);
        assert_eq!(increment_of_containts_prs("2-6,2-5".to_string()), 1);
        assert_eq!(increment_of_containts_prs("2-6,2-7".to_string()), 1);
        assert_eq!(increment_of_containts_prs("2-6,1-7".to_string()), 1);
        let s = increment_of_containts_prs("2-6,3-7".to_string());
        assert_eq!(s, 0);
        assert_eq!(increment_of_containts_prs("3-6,4-7".to_string()), 0);
        assert_eq!(increment_of_containts_prs("4-7,3-6".to_string()), 0);
        assert_eq!(increment_of_containts_prs("1-2,3-3".to_string()), 0);
        assert_eq!(increment_of_containts_prs("19-19,20-99".to_string()), 0);
        assert_eq!(increment_of_containts_prs("1-20,50-50".to_string()), 0);
    }
}
// 99-99,18-99
// 2-86,1-86
// 9-21,10-22
// 1-24,7-23
