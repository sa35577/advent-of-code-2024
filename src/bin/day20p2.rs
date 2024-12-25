use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, fs};

fn main() {
    let f = fs::read_to_string("day20.txt").unwrap();
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

    let mut pq: BinaryHeap<Reverse<(u32, usize, usize)>> = BinaryHeap::new();
    let mut minDist: HashMap<(usize, usize), u32> = HashMap::new();
    
    // println!("{} {}",ey,ex);
    pq.push(Reverse((0,ey,ex)));
    while pq.len() > 0 {
        let Reverse(data) = pq.pop().unwrap();
        if minDist.contains_key(&(data.1,data.2)) {
            continue;
        }
        // println!("({} {}) {}",data.1,data.2,data.0);
        minDist.insert((data.1,data.2),data.0);
        if data.1 > 1 && grid[data.1-1][data.2] != '#' && !minDist.contains_key(&(data.1-1,data.2)) {
            pq.push(Reverse((data.0+1,data.1-1,data.2)));
        }
        if data.1+1 < grid.len() && grid[data.1+1][data.2] != '#' && !minDist.contains_key(&(data.1+1,data.2)) {
            pq.push(Reverse((data.0+1,data.1+1,data.2)));
        }
        if data.2 > 1 && grid[data.1][data.2-1] != '#' && !minDist.contains_key(&(data.1,data.2-1)) {
            pq.push(Reverse((data.0+1,data.1,data.2-1)));
        }
        if data.2 + 1 < grid[0].len() && grid[data.1][data.2+1] != '#' && !minDist.contains_key(&(data.1,data.2+1)) {
            pq.push(Reverse((data.0+1,data.1,data.2+1)));
        }
    }

    let mut good_borders: HashSet<(usize,usize,usize,usize)> = HashSet::new();
    // for i in 1..grid.len()-1 {
    //     for j in 0..grid[0].len() {
    //         if grid[i-1][j] != '#' && grid[i+1][j] != '#' && grid[i][j] == '#' {
    //             let a1 = minDist.get(&(i-1,j)).unwrap();
    //             let a2 = minDist.get(&(i+1,j)).unwrap();
    //             if a1.abs_diff(*a2) >= 102 {
    //                 good_borders.insert((i,j));
    //             }
    //         }
    //     }
    // }
    // for i in 0..grid.len() {
    //     for j in 1..grid[0].len()-1 {
    //         if grid[i][j-1] != '#' && grid[i][j+1] != '#' && grid[i][j] == '#' {
    //             let a1 = minDist.get(&(i,j-1)).unwrap();
    //             let a2 = minDist.get(&(i,j+1)).unwrap();
    //             if a1.abs_diff(*a2) >= 102 {
    //                 good_borders.insert((i,j));
    //             }
    //         }
    //     }
    // }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                continue;
            }
            let a1 = minDist.get(&(i,j)).unwrap();
            for deltax  in 0..21 {
                for deltay in 0..21-deltax {
                    if i >= deltay && j >= deltax && grid[i-deltay][j-deltax] != '#' {
                        let a2 = minDist.get(&(i-deltay,j-deltax)).unwrap();
                        if *a2 >= 100 + *a1 + (deltay as u32) + (deltax as u32) {
                            good_borders.insert((i,j,i-deltay,j-deltax));
                        }
                    }
                    if i >= deltay && j + deltax < grid[0].len() && grid[i-deltay][j+deltax] != '#' {
                        let a2 = minDist.get(&(i-deltay,j+deltax)).unwrap();
                        if *a2 >= 100 + *a1 + (deltay as u32) + (deltax as u32) {
                            good_borders.insert((i,j,i-deltay,j+deltax));
                        }
                    }
                    if i + deltay < grid.len() && j >= deltax && grid[i+deltay][j-deltax] != '#' {
                        let a2 = minDist.get(&(i+deltay,j-deltax)).unwrap();
                        if *a2 >= 100 + *a1 + (deltay as u32) + (deltax as u32) {
                            good_borders.insert((i,j,i+deltay,j-deltax));
                        }
                    }
                    if i + deltay < grid.len() && j + deltax < grid[0].len() && grid[i+deltay][j+deltax] != '#' {
                        let a2 = minDist.get(&(i+deltay,j+deltax)).unwrap();
                        if *a2 >= 100 + *a1 + (deltay as u32) + (deltax as u32) {
                            good_borders.insert((i,j,i+deltay,j+deltax));
                        }
                    }
                    
                }
            }
        }
    }
    
    println!("{}",good_borders.len());

}