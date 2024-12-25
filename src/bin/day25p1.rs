use std::fs;

fn main() {
    let f = fs::read_to_string("day25.txt").unwrap();
    let lines: Vec<Vec<char>> = f.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect();
    let mut line_ptr = 0;
    let mut keys: Vec<Vec<usize>> = vec![];
    let mut locks: Vec<Vec<usize>> = vec![];

    while line_ptr < lines.len() {
        if lines[line_ptr][0] == '#' {
            //locks
            let mut lock: Vec<usize> = vec![];
            for i in 0..lines[0].len() {
                let mut height = 0;
                while lines[line_ptr+height+1][i] == '#' {
                    height += 1;
                }
                lock.push(height);
            }
            locks.push(lock);
        }
        else {
            let mut key: Vec<usize> = vec![];
            let mut line_end = line_ptr + 6;
            for i in 0..lines[0].len() {
                let mut height = 0;
                while lines[line_end-height-1][i] == '#' {
                    height += 1;
                }
                key.push(height);
            }
            keys.push(key);
        }
        line_ptr += 8;
    }
    // println!("{:?}",locks);
    // println!("{:?}",keys);
    let mut cnt = 0;
    for i in 0..locks.len() {
        for j in 0..keys.len() {
            let mut good = true;
            for k in 0..locks[i].len() {
                if locks[i][k] + keys[j][k] > 5 {
                    good = false;
                    break;
                }
            }
            if good {
                cnt += 1;
            }
        }
    }
    println!("{}",cnt);
}