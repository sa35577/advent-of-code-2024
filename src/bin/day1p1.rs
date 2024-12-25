use std::fs;

fn main() {
    let contents = fs::read_to_string("day1.txt").unwrap();
    let parts = contents.split("\n");
    let mut result = 0;

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for part in parts {
        let nums: Vec<&str> = part.split("   ").collect();
        vec1.push(nums[0].parse::<i32>().unwrap());
        vec2.push(nums[1].parse::<i32>().unwrap());
    }
    vec1.sort();
    vec2.sort();

    for i in 0..vec1.len() {
        result += (vec1[i] - vec2[i]).abs();
    }
    println!("{result}");
}