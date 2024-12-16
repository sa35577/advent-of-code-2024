use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let f = fs::read_to_string("day8.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();
    let lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let ysize: i32 = lines.len() as i32;
    let xsize: i32 = lines[0].len() as i32;

    let mut hm: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut hs: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            let c: char = lines[i][j];
            if c != '.' && c != '#' {
                hm.entry(c).or_insert_with(Vec::new).push((i as i32, j as i32));
            }
        }
    }

    // println!("{:?}",hs);

    for (_, v) in &hm {
        for i in 0..v.len() {
            for j in 0..v.len() {
                if i != j {
                    let diffy = v[j].0 - v[i].0;
                    let diffx = v[j].1 - v[i].1;
                    let newy = v[i].0 + 2*diffy;
                    let newx = v[i].1 + 2*diffx;
                    if newy >= 0 && newy < ysize && newx >= 0 && newx < xsize {
                        hs.insert((newy,newx));
                    } 
                }
            }
        }
    }
    println!("{}",hs.len());


    

}