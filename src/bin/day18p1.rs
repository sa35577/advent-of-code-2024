use std::fs;
use std::collections::BinaryHeap;

fn main() {
    let mut grid: Vec<Vec<i32>> = vec![vec![100000; 71]; 71];
    let f = fs::read_to_string("day18.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    for i in 0..1024 {
        let coords: Vec<usize> = lines[i].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        grid[coords[0]][coords[1]] = -1;
    }

    let mut pq: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
    pq.push((0,0,0));
    grid[0][0] = 0;
    while pq.len() > 0 {
        let data = pq.pop().unwrap();
        let dist = -data.0;
        let y = data.1;
        let x = data.2;
        // println!("{} {} {}",dist,y,x);
        if y >= 1 && grid[y-1][x] != -1 && grid[y-1][x] > dist+1 {
            grid[y-1][x] = dist+1;
            pq.push((-(dist+1), y-1, x));
        }
        if y+1 < grid.len() && grid[y+1][x] != -1 && grid[y+1][x] > dist+1 {
            grid[y+1][x] = dist+1;
            pq.push((-(dist+1), y+1, x));
        }
        if x >= 1 && grid[y][x-1] != -1 && grid[y][x-1] > dist+1 {
            grid[y][x-1] = dist+1;
            pq.push((-(dist+1), y, x-1));
        }
        if x+1 < grid[0].len() && grid[y][x+1] != -1 && grid[y][x+1] > dist+1 {
            grid[y][x+1] = dist+1;
            pq.push((-(dist+1), y, x+1));
        }
    }
    println!("{}",grid[70][70]);
    // println!("{}",grid[6][6]);
    // for i in 0..7 {
    //     println!("{:?}",grid[i]);
    // }



}