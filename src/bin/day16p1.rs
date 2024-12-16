use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, fs};

fn main() {
    let f = fs::read_to_string("day16.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let mut grid: Vec<Vec<char>> = vec![];

    let (mut sy, mut sx) = (0,0);
    let (mut ey, mut ex) = (0,0);

    for i in 0..lines.len() {
        let tmp: Vec<char> = lines[i].chars().collect();
        for j in 0..tmp.len() {
            if tmp[j] == 'S' {
                sy = i;
                sx = j;
            }
            else if tmp[j] == 'E' {
                ey = i;
                ex = j;
            }
        }
        grid.push(tmp);
    }

    let mut pq: BinaryHeap<Reverse<(u32, char, usize, usize)>> = BinaryHeap::new();
    let mut minDist: HashMap<(char, usize, usize), u32> = HashMap::new();
    pq.push(Reverse((0,'E',sy,sx)));
    while pq.len() > 0 {
        let Reverse(data) = pq.pop().unwrap();
        // println!("{:?}",data);
        if data.2 == ey && data.3 == ex {
            println!("{}",data.0);
            return;
        }
        if minDist.contains_key(&(data.1,data.2,data.3)) {
            continue;
        }
        minDist.insert((data.1,data.2,data.3), data.0);
        if data.1 == 'E' {
            if !minDist.contains_key(&('W',data.2,data.3)) {
                pq.push(Reverse((data.0+2000, 'W', data.2, data.3)));
            }
            if !minDist.contains_key(&('S',data.2,data.3)) {
                pq.push(Reverse((data.0+1000, 'S', data.2, data.3)));
            }
            if !minDist.contains_key(&('N',data.2,data.3)) {
                pq.push(Reverse((data.0+1000, 'N', data.2, data.3)));
            }
            if data.3 + 1 < grid[0].len() && grid[data.2][data.3+1] != '#' && !minDist.contains_key(&('E', data.2, data.3+1)) {
                pq.push(Reverse((data.0+1,'E', data.2, data.3+1)));
            }
        }
        else if data.1 == 'W' {
            if !minDist.contains_key(&('E',data.2,data.3)) {
                pq.push(Reverse((data.0+2000, 'E', data.2, data.3)));
            }
            if !minDist.contains_key(&('S',data.2,data.3)) {
                pq.push(Reverse((data.0+1000, 'S', data.2, data.3)));
            }
            if !minDist.contains_key(&('N',data.2,data.3)) {
                pq.push(Reverse((data.0+1000, 'N', data.2, data.3)));
            }
            if data.3 >= 1 && grid[data.2][data.3-1] != '#' && !minDist.contains_key(&('W', data.2, data.3-1)) {
                pq.push(Reverse((data.0+1,'W', data.2, data.3-1)));
            }
        }
        else if data.1 == 'S' {
            if !minDist.contains_key(&('N',data.2,data.3)) {
                pq.push(Reverse((data.0+2000, 'N', data.2, data.3)));
            }
            if !minDist.contains_key(&('E',data.2,data.3)) {
                pq.push(Reverse((data.0+1000, 'E', data.2, data.3)));
            }
            if !minDist.contains_key(&('W',data.2,data.3)) {
                pq.push(Reverse((data.0+1000, 'W', data.2, data.3)));
            }
            if data.2+1 < grid.len() && grid[data.2+1][data.3] != '#' && !minDist.contains_key(&('S', data.2+1, data.3)) {
                pq.push(Reverse((data.0+1,'S', data.2+1, data.3)));
            }
        }
        else if data.1 == 'N' {
            if !minDist.contains_key(&('S',data.2,data.3)) {
                pq.push(Reverse((data.0+2000, 'S', data.2, data.3)));
            }
            if !minDist.contains_key(&('E',data.2,data.3)) {
                pq.push(Reverse((data.0+1000, 'E', data.2, data.3)));
            }
            if !minDist.contains_key(&('W',data.2,data.3)) {
                pq.push(Reverse((data.0+1000, 'W', data.2, data.3)));
            }
            if data.2 >= 1 && grid[data.2-1][data.3] != '#' && !minDist.contains_key(&('N', data.2-1, data.3)) {
                pq.push(Reverse((data.0+1,'N', data.2-1, data.3)));
            }
        }
        
    }




}