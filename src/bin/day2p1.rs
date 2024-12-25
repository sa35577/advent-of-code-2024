use std::fs;

fn main() {
    println!("Hello there");
    let contents = fs::read_to_string("day2.txt").unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut result = 0;

    for line in lines {
        let nums: Vec<i32> = line.split(" ").filter_map(|s| s.parse::<i32>().ok()).collect();
        let (mut all_inc, mut all_dec, mut all_valid) = (true, true, true);
        for i in 1..nums.len() {
            if nums[i] >= nums[i-1] {
                all_dec = false;
            }
            else if nums[i] <= nums[i-1] {
                all_inc = false;
            }
            if (nums[i] - nums[i-1]).abs() < 1 || (nums[i] - nums[i-1]).abs() > 3 {
                all_valid = false;
            }
        }
        if all_valid && (all_dec || all_inc) {
            result += 1;
        }
    }
    println!("{result}");
}