use std::{fs, vec};
use std::collections::BinaryHeap;

fn main() {
    let mut grid: Vec<Vec<i32>> = vec![vec![100000; 71]; 71];
    let f = fs::read_to_string("day18.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();
    let mut allCoords: Vec<(usize, usize)> = vec![];

    for i in 0..lines.len() {
        let coords: Vec<usize> = lines[i].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        allCoords.push((coords[0], coords[1]));
    }

    let mut l = 1;
    let mut r = allCoords.len();

    while l+1 < r {
        let mid = (l+r)/2;
        grid = vec![vec![100000; 71]; 71];
        for i in 0..mid as usize{
            grid[allCoords[i].0][allCoords[i].1] = -1;
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

        if grid[70][70] == 100000 {
            r = mid;
        }
        else {
            l = mid+1;
        }
    }

    grid = vec![vec![100000; 71]; 71];
    for i in 0..l as usize{
        grid[allCoords[i].0][allCoords[i].1] = -1;
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
    if grid[70][70] == 100000 {
        println!("{} {:?}",l,allCoords[l-1]);
    }
    else {
        println!("{} {:?}",r,allCoords[r-1]);
    }


}