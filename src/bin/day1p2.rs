use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("day1.txt").unwrap();
    let parts = contents.split("\n");
    let mut result = 0;

    let mut vec1 = Vec::new();
    let mut hashmap = HashMap::new();

    for part in parts {
        let nums: Vec<&str> = part.split("   ").collect();
        vec1.push(nums[0].parse::<i32>().unwrap());
        // vec2.push(nums[1].parse::<i32>().unwrap());
        let count = hashmap.entry(nums[1].parse::<i32>().unwrap()).or_insert(0);
        *count += 1;
    }
    for val in vec1 {
        // println!("{val}");
        if hashmap.contains_key(&val) {
            result += val * hashmap[&val];
            // println!("{result}");
        }
        // else {
            // println!("not found");
        // }
        // println!("----");
    }
    println!("{result}");
}