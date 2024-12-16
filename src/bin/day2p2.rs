use std::fs;

fn helper(nums: Vec<i32>) -> i32 {
    // return -1 if good
    // return discrepancy index if not good
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
        if !all_valid || !(all_inc || all_dec) {
            return i as i32;
        }
    }
    return -1;
    
}

fn main() {
    println!("Hello there");
    let contents = fs::read_to_string("day2.txt").unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut result = 0;

    for line in lines {
        let nums: Vec<i32> = line.split(" ").filter_map(|s| s.parse::<i32>().ok()).collect();

        let first_try = helper(nums.clone());
        if first_try == -1 {
            result += 1;
        }
        else {
            let i = first_try as usize;
            let f1 = [&nums[..i], &nums[i+1..]].concat();
            let f2 = [&nums[..i-1], &nums[i..]].concat();
            if helper(f1) == -1 || helper(f2) == -1 {
                result += 1;
            }
            else if i > 1 { // discrepancy can be any of the last 3, for example removing first x in sequence (x, val, x) or (1, 3, 2)
                let f3 = [&nums[..i-2], &nums[i-1..]].concat();
                if helper(f3) == -1 {
                    result += 1;
                    println!("{:?}",nums);
                }
            }
        }
    }
    println!("{result}");
}