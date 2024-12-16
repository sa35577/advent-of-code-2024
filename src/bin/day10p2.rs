use std::fs;
use queues::{queue, IsQueue, Queue};
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let f = fs::read_to_string("day10.txt").unwrap();
    let contents: Vec<&str> = f.split("\n").collect();
    const RADIX: u32 = 10;

    let contents: Vec<Vec<char>> = contents.iter().map(|s| s.chars().collect()).collect();

    let contents: Vec<Vec<u32>> = contents.iter().map(|v| v.iter().map(|c| c.to_digit(RADIX).unwrap()).collect()).collect();

    let mut sources: Vec<(usize, usize)> = vec![];

    for i in 0..contents.len() {
        for j in 0..contents[0].len() {
            if contents[i][j] == 0 {
                sources.push((i,j));
            }
        }
    }
    let mut cnt = 0;
    for source in sources {
        let mut hs: HashSet<(usize, usize)> = HashSet::new();
        let mut q: Queue<(u32, usize, usize)> = queue![];
        let mut hm: HashMap<(usize, usize), i32> = HashMap::new();

        q.add((0,source.0,source.1)).unwrap();
        hm.insert((source.0, source.1), 1);
        while q.size() > 0 {
            let data = q.remove().unwrap();
            // println!("{:?}",data);
            let cur_score = *hm.get(&(data.1, data.2)).unwrap();
            if data.0 == 9 {
                cnt += cur_score;
            }
            if data.1 > 0 as usize && contents[data.1-1][data.2] == 1 + data.0 {
                if hs.contains(&(data.1-1,data.2)) {
                    *hm.entry((data.1-1,data.2)).or_insert(0) += cur_score;
                }
                else {
                    q.add((1+data.0, data.1-1, data.2)).unwrap();
                    hs.insert((data.1-1,data.2));
                    *hm.entry((data.1-1,data.2)).or_insert(0) += cur_score;
                }
                
            }
            if data.2 > 0 as usize && contents[data.1][data.2-1] == 1 + data.0 {
                if !hs.contains(&(data.1,data.2-1)) {
                    q.add((1+data.0, data.1, data.2-1)).unwrap();
                    hs.insert((data.1,data.2-1));
                }
                *hm.entry((data.1,data.2-1)).or_insert(0) += cur_score;                
            }
            if data.1+1 < contents.len() && contents[data.1+1][data.2] == 1 + data.0 {
                if !hs.contains(&(data.1+1,data.2)) {
                    q.add((1+data.0, data.1+1, data.2)).unwrap();
                    hs.insert((data.1+1,data.2));
                }
                *hm.entry((data.1+1,data.2)).or_insert(0) += cur_score;                
            }
            if data.2+1 < contents[0].len() && contents[data.1][data.2+1] == 1 + data.0 {
                if !hs.contains(&(data.1,data.2+1)) {
                    q.add((1+data.0, data.1, data.2+1)).unwrap();
                    hs.insert((data.1,data.2+1));
                }
                *hm.entry((data.1,data.2+1)).or_insert(0) += cur_score;
            }
        }
        
    }
    
    println!("{cnt}");

    // println!("{:?}",sources);



}