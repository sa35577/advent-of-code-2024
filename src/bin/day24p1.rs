use std::{collections::{HashMap, HashSet}, fs, result};

use queues::{IsQueue, Queue};

fn main() {
    let mut evals: HashMap<&str,i32> = HashMap::new();
    let f = fs::read_to_string("day24.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();
    let mut parse_rules = true;
    let mut network: Vec<(&str, &str, &str, &str)> = vec![]; //(first val, operator, second val, result)
    let mut q: Queue<usize> = Queue::new();
    let mut finished: HashSet<usize> = HashSet::new();

    for line_idx in 0..lines.len() {
        if lines[line_idx].len() == 0 {
            parse_rules = false;
        }
        else if parse_rules {
            let data: Vec<&str> = lines[line_idx].split(": ").collect();
            evals.insert(data[0], data[1].parse::<i32>().unwrap());
        }
        else {
            let sides: Vec<&str> = lines[line_idx].split(" -> ").collect();
            let data: Vec<&str> = sides[0].split(" ").collect();
            network.push((data[0],data[1],data[2],sides[1]));
            if evals.contains_key(data[0]) && evals.contains_key(data[2]) {
                q.add(network.len()-1).unwrap();
                finished.insert(network.len()-1);
            }
        }
    }
    let mut cnt = 0;
    println!("{:?}",network[88]);
    println!("{:?}",network[179]);
    while q.size() > 0 {
        let idx = q.remove().unwrap();
        let data = network[idx];
        let mut result = 0;
        cnt += 1;
        // println!("{cnt} {idx}");
        if data.1 == "AND" {
            result = evals.get(data.0).unwrap() & evals.get(data.2).unwrap();
        }
        else if data.1 == "OR" {
            result = evals.get(data.0).unwrap() | evals.get(data.2).unwrap();
        }
        else if data.1 == "XOR" {
            result = evals.get(data.0).unwrap() ^ evals.get(data.2).unwrap();
        }
        evals.insert(data.3, result);
        // finished.insert(idx);
        for i in 0..network.len() {
            if !finished.contains(&i) && evals.contains_key(network[i].0) && evals.contains_key(network[i].2) {
                q.add(i).unwrap();
                finished.insert(i);
            }
        }
    }
    let mut result_data: Vec<(&str, i32)> = vec![];
    for (s, v) in evals.into_iter() {
        let chars: Vec<char> = s.chars().collect();
        if chars[0] == 'z' {
            result_data.push((s,v));
        }

    }
    result_data.sort();
    let mut final_val: u128 = 0;
    for i in 0..result_data.len() {
        let idx = result_data.len()-1-i;
        final_val = final_val * 2 + (result_data[idx].1 as u128);
    }
    println!("{}",final_val);

}