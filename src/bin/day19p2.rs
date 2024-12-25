use std::fs;

fn main() {
    let f = fs::read_to_string("day19.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let options: Vec<Vec<char>> = lines[0].split(", ").map(|s| s.chars().collect()).collect();
    // println!("{:?}",options);
    let mut tot: u64 = 0;

    for idx in 2..lines.len() {
        let input: Vec<char> = lines[idx].chars().collect();
        let mut dp: Vec<u64> = vec![0; input.len()+1];
        dp[0] = 1;
        for i in 0..input.len() {
            if dp[i] == 0 {
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
                    dp[i+opt.len()] += dp[i];
                }
            }
        }
        tot += dp[input.len()];
        // if dp[input.len()] {
        //     tot += 1;
        // }
        // println!("{}",dp[input.len()]);
    }
    println!("{}",tot);




}