use std::fs;
use std::collections::HashSet;

fn main() {
    let f = fs::read_to_string("day5.txt").unwrap();
    let contents: Vec<&str> = f.split("\n").collect();

    let mut hs = HashSet::new();
    let mut parse_rules = true;
    let mut final_result = 0;

    for i in 0..contents.len() {
        if contents[i].len() == 0 {
            parse_rules = false;
        }
        else if parse_rules {
            let vals: Vec<i32> = contents[i].split("|").filter_map(|s| s.parse::<i32>().ok()).collect();
            hs.insert([vals[1], vals[0]]);
        }
        else {
            let vals: Vec<i32> = contents[i].split(",").filter_map(|s| s.parse::<i32>().ok()).collect();
            let mut valid = true;
            for j in 1..vals.len() {
                for k in 0..j {
                    if hs.contains(&[vals[k], vals[j]]) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            if valid {
                println!("{}",vals.len());
                let idx = (vals.len()-1)/2;
                final_result += vals[idx];
            }
        }
    }
    println!("{}",final_result);

}