use std::fs;


fn mix(initial_secret: u128, cur_val: u128) -> u128 {
    return initial_secret ^ cur_val;
}

fn prune(initial_secret: u128) -> u128 {
    return initial_secret % 16777216;
}

fn main() {
    let f = fs::read_to_string("day22.txt").unwrap();
    let nums: Vec<u128> = f.split("\n").map(|i| i.parse::<u128>().unwrap()).collect();
    let mut tot = 0;

    for i in 0..nums.len() {
        let mut val = nums[i];
        for _i in 0..2000 {
            val = mix(val, val*64);
            val = prune(val);
            val = mix(val, val/32);
            val = prune(val);
            val = mix(val,val*2048);
            val = prune(val);
        }
        tot += val;

    }
    println!("{}",tot);

}