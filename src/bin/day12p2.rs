use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use queues::IsQueue;
use queues::Queue;

fn main() {
    let f = fs::read_to_string("day12.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    
    let mut hm: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
    let mut groupMap: HashMap<(usize, usize), usize> = HashMap::new();
    let mut hs: HashSet<(usize, usize)> = HashSet::new();
    let mut gp: usize = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !hs.contains(&(i,j)) {
                let mut q: Queue<(usize, usize)> = Queue::new();
                q.add((i,j)).unwrap();
                hs.insert((i,j));

                while q.size() > 0 {
                    let val: (usize, usize) = q.remove().unwrap();
                    hm.entry(gp as i32).or_insert(vec![]).push(val);
                    groupMap.insert(val, gp);
                    if val.0 >= 1 && grid[val.0-1][val.1] == grid[val.0][val.1] && !hs.contains(&(val.0-1,val.1)) {
                        hs.insert((val.0-1,val.1));
                        q.add((val.0-1,val.1)).unwrap();
                    }
                    if val.0+1 < grid[0].len() && grid[val.0+1][val.1] == grid[val.0][val.1] && !hs.contains(&(val.0+1,val.1)) {
                        hs.insert((val.0+1,val.1));
                        q.add((val.0+1,val.1)).unwrap();
                    }
                    if val.1 >= 1 && grid[val.0][val.1-1] == grid[val.0][val.1] && !hs.contains(&(val.0,val.1-1)) {
                        hs.insert((val.0,val.1-1));
                        q.add((val.0,val.1-1)).unwrap();
                    }
                    if val.1+1 < grid[0].len() && grid[val.0][val.1+1] == grid[val.0][val.1] && !hs.contains(&(val.0,val.1+1)) {
                        hs.insert((val.0,val.1+1));
                        q.add((val.0,val.1+1)).unwrap();
                    }

                }
                gp += 1;


            }
        }
    }
    let mut result: u32 = 0;
    let mut perimeter: Vec<u32> = vec![0; gp];

    for row in 0..grid.len() {
        let mut ptr: usize = 0;
        while ptr < grid[0].len() {
            if row > 0 && grid[row][ptr] == grid[row-1][ptr] {
                ptr += 1;
            }
            else {
                let group = groupMap[&(row,ptr)];
                perimeter[group] += 1;
                ptr += 1;
                while ptr < grid[0].len() && groupMap[&(row,ptr)] == group && (row == 0 || groupMap[&(row-1,ptr)] != group) {
                    ptr += 1;
                }
            }
        }
        ptr = 0;
        while ptr < grid[0].len() {
            if row+1 < grid.len() && grid[row][ptr] == grid[row+1][ptr] {
                ptr += 1;
            }
            else {
                let group = groupMap[&(row,ptr)];
                perimeter[group] += 1;
                ptr += 1;
                while ptr < grid[0].len() && groupMap[&(row,ptr)] == group && (row+1 == grid.len() || groupMap[&(row+1,ptr)] != group) {
                    ptr += 1;
                }
            }
        }

    }

    for col in 0..grid[0].len() {
        let mut ptr: usize = 0;
        while ptr < grid.len() {
            if col > 0 && grid[ptr][col] == grid[ptr][col-1] {
                ptr += 1;
            }
            else {
                let group = groupMap[&(ptr,col)];
                perimeter[group] += 1;
                ptr += 1;
                while ptr < grid.len() && groupMap[&(ptr,col)] == group && (col == 0 || groupMap[&(ptr,col-1)] != group) {
                    ptr += 1;
                }
            }
        }
        ptr = 0;
        while ptr < grid.len() {
            if col+1 < grid[0].len() && grid[ptr][col] == grid[ptr][col+1] {
                ptr += 1;
            }
            else {
                let group = groupMap[&(ptr,col)];
                perimeter[group] += 1;
                ptr += 1;
                while ptr < grid.len() && groupMap[&(ptr,col)] == group && (col+1 == grid[0].len() || groupMap[&(ptr,col+1)] != group) {
                    ptr += 1;
                }
            }
        }
    }

    for i in 0..gp {
        result += perimeter[i] * (hm.get(&(i as i32)).unwrap().len() as u32);
    }

    println!("{}",result);


}