use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let f = fs::read_to_string("day23.txt").unwrap();
    let data: Vec<Vec<&str>> = f.split("\n").map(|s| s.split("-").collect::<Vec<&str>>()).collect();
    let mut network: HashMap<(char, char), HashSet<(char, char)>> = HashMap::new();
    let mut tot = 0;
    for i in 0..data.len() {
        let c1: Vec<char> = data[i][0].chars().collect();
        let c1 = (c1[0], c1[1]);
        let c2: Vec<char> = data[i][1].chars().collect();
        let c2 = (c2[0], c2[1]);
        network.entry(c1).or_insert(HashSet::new()).insert(c2);
        network.entry(c2).or_insert(HashSet::new()).insert(c1);
        let n1 = network.get(&c1).unwrap();
        let n2 = network.get(&c2).unwrap();
        for c3 in n1 {
            if n2.contains(c3) && (c1.0 == 't' || c2.0 == 't' || c3.0 == 't') {
                tot += 1;
            }
        }
    }
    println!("{}",tot);
}