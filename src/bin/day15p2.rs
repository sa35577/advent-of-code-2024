use std::{collections::{HashMap, HashSet}, fs, vec};

fn main() {
    let f = fs::read_to_string("day15.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let mut grid: Vec<Vec<char>> = vec![];
    for i in 0..50 {
        let tmp: Vec<char> = lines[i].chars().collect();
        let mut res: Vec<char> = vec![];
        for c in tmp {
            if c == '#' {
                res.push('#');
                res.push('#');
            }
            else if c == 'O' {
                res.push('[');
                res.push(']');
            }
            else if c == '.' {
                res.push('.');
                res.push('.');
            }
            else {
                res.push('@');
                res.push('.');
            }
        }
        grid.push(res);
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

    // for i in &grid {
    //     println!("{:?}",i);
    // }
    // println!("------------------------");

    for jason_oxygenmask in 51..lines.len() {
        for c in lines[jason_oxygenmask].chars().collect::<Vec<char>>() {
            let mut num_boxes = 0;
            let dir = hm.get(&c).unwrap();
            if c == '<' || c == '>' {
                //num_boxes is like num_cols, so probably like 2 x number of actual boxes
                while grid[(pos.0+dir.0*(num_boxes+1)) as usize][(pos.1+dir.1*(num_boxes+1)) as usize] == '[' || grid[(pos.0+dir.0*(num_boxes+1)) as usize][(pos.1+dir.1*(num_boxes+1)) as usize] == ']' {
                    num_boxes += 1;
                }
                if grid[(pos.0+dir.0*(num_boxes+1)) as usize][(pos.1+dir.1*(num_boxes+1)) as usize] == '#' {
                    //wall, do nothing
                    num_boxes = num_boxes;
                }
                else {
                    //shift
                    grid[pos.0 as usize][pos.1 as usize] = '.';
                    // grid[(pos.0+dir.0*(num_boxes+1)) as usize][(pos.1+dir.1*(num_boxes+1)) as usize] = ']';
                    for i in 0..num_boxes {
                        let ii = num_boxes-i;
                        grid[pos.0 as usize][(pos.1+dir.1*(ii+1)) as usize] = grid[pos.0 as usize][(pos.1+dir.1*(ii)) as usize]
                    }
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                    grid[pos.0 as usize][pos.1 as usize] = '@';
                }
            }
            else {
                //num_boxes here is treated more like num rows 
                let mut good = true;
                let mut checks: HashSet<i32> = HashSet::new();
                let mut all_boxes: Vec<HashSet<i32>> = vec![];
                checks.insert(pos.1);
                while true {
                    let mut new_checks: HashSet<i32> = HashSet::new();
                    for col in &checks {
                        let above_char = grid[(pos.0+dir.0*(num_boxes+1)) as usize][*col as usize];
                        if above_char == '#' {
                            good = false;
                            break;
                        }
                        else if above_char == '[' {
                            new_checks.insert(*col);
                            new_checks.insert(*col+1);
                        }
                        else if above_char == ']' {
                            new_checks.insert(*col);
                            new_checks.insert(*col-1);
                        }

                    }
                    if !good || new_checks.len() == 0 {
                        break;
                    }
                    num_boxes += 1;
                    all_boxes.push(new_checks.clone());
                    checks = new_checks;
                }

                if good {
                    //move all boxes lol
                    grid[pos.0 as usize][pos.1 as usize] = '.';
                    for i in 0..all_boxes.len() {
                        let ii = all_boxes.len()-1-i;
                        for col in &all_boxes[ii] {
                            grid[(pos.0+dir.0*(ii as i32+2)) as usize][*col as usize] = grid[(pos.0+dir.0*(ii as i32+1)) as usize][*col as usize];
                            grid[(pos.0+dir.0*(ii as i32+1)) as usize][*col as usize] = '.';
                        }

                    }

                    pos.0 += dir.0;
                    pos.1 += dir.1;
                    grid[pos.0 as usize][pos.1 as usize] = '@';
                    // println!("{:?} {}",all_boxes,good);
                }
                else {
                    //do nothing
                    num_boxes = num_boxes;
                }
            }

            // for i in &grid {
            //     println!("{:?}",i);
            // }
            // println!("------------------------");
            
            // println!("{:?}",pos);
        }
    }
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '[' {
                result += 100*i+j;
            }
        }
    }

    println!("{result}")


}