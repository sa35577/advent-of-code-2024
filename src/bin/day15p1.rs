use std::{collections::HashMap, fs};

fn main() {
    let f = fs::read_to_string("day15.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let mut grid: Vec<Vec<char>> = vec![];
    for i in 0..50 {
        grid.push(lines[i].chars().collect());
    }
    
    let mut pos: (i32, i32) = (0,0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                pos = (i as i32,j as i32);
            }
        }
    }

    let mut hm: HashMap<char, (i32, i32)> = HashMap::new();
    hm.insert('^', (-1,0));
    hm.insert('v', (1,0));
    hm.insert('<', (0,-1));
    hm.insert('>', (0,1));

    for jason_oxygenmask in 51..lines.len() {
        for c in lines[jason_oxygenmask].chars().collect::<Vec<char>>() {
            let mut num_boxes = 0;
            let dir = hm.get(&c).unwrap();
            while grid[(pos.0+dir.0*(num_boxes+1)) as usize][(pos.1+dir.1*(num_boxes+1)) as usize] == 'O' {
                num_boxes += 1;
            }
            if grid[(pos.0+dir.0*(num_boxes+1)) as usize][(pos.1+dir.1*(num_boxes+1)) as usize] == '#' {
                //wall, do nothing
                num_boxes = num_boxes;
            }
            else {
                //shift
                grid[pos.0 as usize][pos.1 as usize] = '.';
                grid[(pos.0+dir.0*(num_boxes+1)) as usize][(pos.1+dir.1*(num_boxes+1)) as usize] = 'O';
                pos.0 += dir.0;
                pos.1 += dir.1;
                grid[pos.0 as usize][pos.1 as usize] = '@';
            }
            // println!("{:?}",pos);
        }
    }
    let mut result = 0;
    for i in 0..grid.len() {
        // println!("{:?}",grid[i]);
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                result += 100*i+j;
            }
        }
    }

    println!("{result}")


}