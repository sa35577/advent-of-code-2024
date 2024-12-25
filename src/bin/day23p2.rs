use std::{collections::{HashMap, HashSet}, fs, vec};

fn main() {
    let f = fs::read_to_string("day23.txt").unwrap();
    let data: Vec<Vec<&str>> = f.split("\n").map(|s| s.split("-").collect::<Vec<&str>>()).collect();
    let mut network: HashMap<(char, char), HashSet<(char, char)>> = HashMap::new();
    let mut longest = 0;
    let mut ans: Vec<(char, char)> = vec![];
    let mut all_keys: Vec<(char, char)> = vec![];
    let mut seen: HashSet<(char, char)> = HashSet::new();

    for i in 0..data.len() {
        let c1: Vec<char> = data[i][0].chars().collect();
        let c1 = (c1[0], c1[1]);
        let c2: Vec<char> = data[i][1].chars().collect();
        let c2 = (c2[0], c2[1]);
        network.entry(c1).or_insert(HashSet::new()).insert(c2);
        network.entry(c2).or_insert(HashSet::new()).insert(c1);
        if !seen.contains(&c1) {
            seen.insert(c1);
            all_keys.push(c1);
        }
        if !seen.contains(&c2) {
            seen.insert(c2);
            all_keys.push(c2);
        }
        
    }
    println!("{} {}",seen.len(),all_keys.len());
    for i in 0..all_keys.len()-1 {
        println!("{i}");
        let mut all_sequences: Vec<Vec<(char, char)>> = vec![vec![all_keys[i]]];
        for j in i+1..all_keys.len() {
            let mut add_sequences: Vec<Vec<(char, char)>> = vec![];
            let st = network.get(&all_keys[j]).unwrap();
            for v in &all_sequences {
                let mut good = true;
                for entry in v {
                    if !st.contains(entry) {
                        good = false;
                        break;
                    }
                }
                if good {
                    let mut w = v.clone();
                    w.push(all_keys[j]);
                    add_sequences.push(w);
                    // println!("asdfasdfasdf");
                }
            }
            for seq in add_sequences {
                if seq.len() > longest {
                    longest = seq.len();
                    ans = seq.clone();
                }
                all_sequences.push(seq);
            }
        }
        // println!("{:?}",all_sequences);
    }
    ans.sort();
    for i in 0..ans.len() {
        print!("{}{}",ans[i].0,ans[i].1);
        if i != ans.len()-1 {
            print!(",");
        }
    }
    println!();
}