use std::fs;

fn main() {
    let f = fs::read_to_string("day11.txt").unwrap();
    let mut nums: Vec<u128> = f.split(" ").filter_map(|s| s.parse().ok()).collect();

    for _ in 0..25 {
        let mut newnums: Vec<u128> = vec![];
        for num in nums {
            if num == 0 {
                newnums.push(1);
            }
            else {
                let log_ceil = num.ilog10()+1;
                if log_ceil % 2 == 0 {
                    let splitter: u128 = 10_u128.pow(log_ceil/2);
                    newnums.push(num/splitter);
                    newnums.push(num%splitter);
                }
                else {
                    newnums.push(num * 2024);
                }
            }
        }
        // println!("{:?}",newnums);
        nums = newnums;
    }
    println!("{}",nums.len());
}
