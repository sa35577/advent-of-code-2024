use std::fs;

fn main() {
    let f = fs::read_to_string("day19.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let options: Vec<Vec<char>> = lines[0].split(", ").map(|s| s.chars().collect()).collect();
    // println!("{:?}",options);
    let mut tot = 0;

    for idx in 2..lines.len() {
        let input: Vec<char> = lines[idx].chars().collect();
        let mut dp: Vec<bool> = vec![false; input.len()+1];
        dp[0] = true;
        for i in 0..input.len() {
            if !dp[i] {
                continue;
            }
            for opt in &options {
                let mut good = true;
                for j in 0..opt.len() {
                    if i+j >= input.len() || opt[j] != input[i+j] {
                        good = false;
                        break;
                    }
                }
                if good {
                    dp[i+opt.len()] = true;
                }
            }
        }
        if dp[input.len()] {
            tot += 1;
        }
    }
    println!("{}",tot);




}