use std::fs;
use std::collections::HashSet;
use queues::*;

fn main() {
    let f = fs::read_to_string("day5.txt").unwrap();
    let contents: Vec<&str> = f.split("\n").collect();

    let mut hs = HashSet::new();
    let mut parse_rules = true;
    let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); 100];
    let mut final_result = 0;
    

    for kk in 0..contents.len() {
        if contents[kk].len() == 0 {
            parse_rules = false;
        }
        else if parse_rules {
            let vals: Vec<i32> = contents[kk].split("|").filter_map(|s| s.parse::<i32>().ok()).collect();
            hs.insert([vals[1], vals[0]]);
            let v = vals[0] as usize;
            let u = vals[1] as usize;
            graph[v].insert(u);
        }
        else {
            let vals: Vec<i32> = contents[kk].split(",").filter_map(|s| s.parse::<i32>().ok()).collect();
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
                // println!("{}",vals.len());
                // let idx = (vals.len()-1)/2;
                // final_result += vals[idx];
                // println!("asdfasdfasdf {:?}",vals);
                final_result += 0;
            }
            else {
                let mut order: Vec<i32> = vec!();
                let mut in_degree: Vec<i32> = vec!();
                let mut q = queue![];
                let mut nums = HashSet::new();
                for num in &vals {
                    nums.insert(num);
                }
                for _i in 0..100 {
                    in_degree.push(0);
                }
                for i in 0..100 {
                    let ii: i32 = i as i32;
                    for j in &graph[i] {
                        let jj = *j as i32;
                        if nums.contains(&ii) && nums.contains(&jj) {
                            in_degree[*j] += 1;
                        }
                    }
                }

                for i in 0..100 {
                    let ii: i32 = i as i32;
                    if in_degree[i] == 0 && nums.contains(&ii) {
                        let _ = q.add(i);
                    }
                }

                while q.size() > 0 {
                    let v = q.remove().unwrap();
                    order.push(v as i32);
                    for nxt in &graph[v] {
                        let nn: i32 = *nxt as i32;
                        if nums.contains(&nn) {
                            in_degree[*nxt] -= 1;
                            if in_degree[*nxt] == 0 {
                                let _ = q.add(*nxt);
                            }
                            // println!("{}",*nxt);
                        }
                    }
                    // println!("{v}");
                }

                // println!("{}",order.len());
                println!("{:?}",order);
                let idx = (order.len()-1)/2;
                final_result += order[idx];

            }
        }
    }
    println!("{}",final_result);

}