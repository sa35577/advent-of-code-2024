use std::fs;

fn dfs(vals: &Vec<u128>, idx: usize, target: u128) -> bool {
    if idx == 0 {
        return vals[idx] == target;
    }
    if target % vals[idx] == 0 && dfs(vals, idx-1, target/vals[idx]) {
        return true;
    }
    if target < vals[idx] {
        return false;
    }
    return dfs(vals, idx-1, target-vals[idx]);
}

fn main() {
    let f = fs::read_to_string("day7.txt").unwrap();
    let contents: Vec<&str> = f.split("\n").collect();
    let mut result: u128 = 0;

    for line in contents {
        let content: Vec<&str> = line.split(": ").collect();
        let test_val: u128 = content[0].parse().unwrap();
        let vals: Vec<u128> = content[1].split(" ").filter_map(|s| s.parse().ok()).collect();
        println!("{:?}",vals);
        if dfs(&vals, &vals.len()-1, test_val) {
            result += test_val;
        }
    }
    println!("{}",result);
}