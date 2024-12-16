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
    let mut hs: HashSet<(usize, usize)> = HashSet::new();
    let mut gp = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !hs.contains(&(i,j)) {
                let mut q: Queue<(usize, usize)> = Queue::new();
                q.add((i,j)).unwrap();
                gp += 1;
                hs.insert((i,j));

                while q.size() > 0 {
                    let val: (usize, usize) = q.remove().unwrap();
                    hm.entry(gp).or_insert(vec![]).push(val);
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


            }
        }
    }
    let mut result: u32 = 0;

    for coords in hm.values() {
        let area: u32 = coords.len() as u32;
        let mut perimeter: u32 = 0;
        for val in coords {
            if val.0 >= 1 && grid[val.0-1][val.1] == grid[val.0][val.1] {
                perimeter = perimeter;
            }
            else {
                perimeter += 1;
            }
            if val.0+1 < grid[0].len() && grid[val.0+1][val.1] == grid[val.0][val.1] {
               perimeter = perimeter;
            }
            else {
                perimeter += 1;
            }
            if val.1 >= 1 && grid[val.0][val.1-1] == grid[val.0][val.1] {
                perimeter = perimeter;
            }
            else {
                perimeter += 1;
            }
            if val.1+1 < grid[0].len() && grid[val.0][val.1+1] == grid[val.0][val.1] {
                perimeter = perimeter;
            }
            else {
                perimeter += 1;
            }
        }
        result += area * perimeter;

    }
    println!("{}",result);


}