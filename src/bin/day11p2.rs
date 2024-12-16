use std::fs;
use std::collections::HashMap;

fn main() {
    let f = fs::read_to_string("day11.txt").unwrap();
    let nums: Vec<u128> = f.split(" ").filter_map(|s| s.parse().ok()).collect();
    let mut hm: HashMap<u128, (u128, u128)> = HashMap::new(); //next state
    let mut freq: HashMap<u128, u128> = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    for v in 1..76 {
        let mut newfreq: HashMap<u128, u128> = HashMap::new();
        for (val, frequency) in freq.into_iter() {
            if val == 0 {
                *newfreq.entry(1).or_insert(0) += frequency;
            }
            else if hm.contains_key(&val) {
                let res = hm.get(&val).unwrap();
                *newfreq.entry(res.0).or_insert(0) += frequency;
                *newfreq.entry(res.1).or_insert(0) += frequency;
            }
            else {
                let log_ceil = val.ilog10()+1;
                if log_ceil % 2 == 0 {
                    let splitter: u128 = 10_u128.pow(log_ceil/2);
                    let res = (val/splitter, val%splitter);
                    hm.insert(val, res);
                    *newfreq.entry(res.0).or_insert(0) += frequency;
                    *newfreq.entry(res.1).or_insert(0) += frequency;
                }
                else {
                    *newfreq.entry(val*2024).or_insert(0) += frequency;
                }
            }
        }
        freq = newfreq;
        println!("{} {}", freq.values().sum::<u128>(), v)
    }
    println!("{}",freq.values().sum::<u128>());
}