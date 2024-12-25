use std::{collections::{HashMap, HashSet}, fs, vec};


fn mix(initial_secret: u128, cur_val: u128) -> u128 {
    return initial_secret ^ cur_val;
}

fn prune(initial_secret: u128) -> u128 {
    return initial_secret % 16777216;
}

fn main() {
    let f = fs::read_to_string("day22.txt").unwrap();
    let nums: Vec<u128> = f.split("\n").map(|i| i.parse::<u128>().unwrap()).collect();
    // let mut tot = 0;
    let mut globalBest: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();
    let mut mx: i32 = 0;

    for i in 0..nums.len() {
        let mut val = nums[i];
        let mut diffs: Vec<i32> = vec![];
        let mut prices: Vec<u128> = vec![];
        for _i in 0..2000 {
            let v: i32 = (val%10) as i32;
            val = mix(val, val*64);
            val = prune(val);
            val = mix(val, val/32);
            val = prune(val);
            val = mix(val,val*2048);
            val = prune(val);
            prices.push(val);
            diffs.push(((val%10) as i32) - v);
        }
        // tot += val;
        let mut seenSet: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for j in 3..2000 {
            let tup = (diffs[j-3],diffs[j-2],diffs[j-1],diffs[j]);
            // if tup == (-2,1,-1,3) {
            //     println!("seen at {j} for iteration {i}");
            // }
            if !seenSet.contains(&tup) {
                seenSet.insert(tup);
                *globalBest.entry(tup).or_insert(0) += (prices[j] % 10) as i32;
                let v = *globalBest.get(&tup).unwrap();
                if mx < v {
                    mx = v;
                }
            }
        }
        println!("Done iteration {i}");
    }
    println!("{}",mx);

}