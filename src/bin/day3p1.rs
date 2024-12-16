use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("day3.txt").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // println!("{}",contents);
    let mut result: i32 = 0;

    for (dd, [n1,n2]) in re.captures_iter(&contents).map(|s| s.extract()) {
        // println!("{n1} {n2}");
        println!("{dd}");
        let nn1: i32 = n1.parse().unwrap();
        let nn2: i32 = n2.parse().unwrap();
        result = result + nn1 * nn2;
    }
    println!("{result}");

}