use std::fs;
use std::collections::HashSet;

fn main() {
    let f = fs::read_to_string("day6.txt").unwrap();
    let mut contents: Vec<&str> = f.split("\n").collect();
    let mut contents: Vec<Vec<char>> = contents.iter().map(|line| line.chars().collect()).collect();
    let mut dir = 1;
    let mut y = 0;
    let mut x = 0;
    // let mut matrix: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 4]; contents[0].len()]; contents.len()];
    let mut steps: Vec<(usize, usize, usize)> = vec![];
    let mut result = 0;
    

    for i in 0..contents.len() {
        for j in 0..contents[0].len() {
            if contents[i][j] == '^' {
                y = i;
                x = j;
                dir = 0;
                break;
            }
        }
        if dir == 0 { 
            break;
        }
    }
    let yy = y;
    let xx = x;

    for i in 0..contents.len() {
        for j in 0..contents[0].len() {
            if contents[i][j] != '#' && contents[i][j] != '^' {
                steps.push((i,j,0));
            }
        }
    }
    println!("{:?}",steps.len());
    let mut cnt = 0;

    for step in steps {
        cnt += 1;
        // println!("{:?}",step.0);
        // let stepDirection = step.2;
        dir=0;
        y = yy;
        x = xx;
        contents[step.0][step.1] = '#';
        // println!("{} {} {}",step.0,step.1,step.2);

        //execute
        let mut hs= HashSet::new();

        while true {
            // matrix[y][x] = 1;
            if dir == 0 { //up
                if y == 0 {
                    break;
                }
                else if contents[y-1][x] == '#' {
                    dir += 1;
                }
                else if hs.contains(&(y,x,dir)) {
                    result += 1;
                    // println!("{} {} {}",step.0,step.1,step.2);
                    break;
                }
                else {
                    hs.insert((y,x,dir));
                    y -= 1;
                }
            }
            else if dir == 1 { //right
                if x+1 == contents[0].len() {
                    break;
                }
                else if contents[y][x+1] == '#' {
                    dir += 1;
                }
                else if hs.contains(&(y,x,dir)) {
                    result += 1;
                    // println!("{} {} {}",step.0,step.1,step.2);
                    break;
                }
                else {
                    hs.insert((y,x,dir));
                    x += 1;
                }
            }
            else if dir == 2 {
                if y+1 == contents.len() {
                    break;
                }
                else if contents[y+1][x] == '#' {
                    dir += 1;
                }
                else if hs.contains(&(y,x,dir)) {
                    result += 1;
                    // println!("{} {} {}",step.0,step.1,step.2);
                    break;
                }
                else {
                    hs.insert((y,x,dir));
                    y += 1;
                }
            }
            else if dir == 3 {
                if x == 0 {
                    break;
                }
                else if contents[y][x-1] == '#' {
                    dir = 0;
                }
                else if hs.contains(&(y,x,dir)) {
                    result += 1;
                    // println!("{} {} {}",step.0,step.1,step.2);
                    break;
                }
                else {
                    hs.insert((y,x,dir));
                    x -= 1;
                }
            }
        }

        contents[step.0][step.1] = '.';
        if cnt % 400 == 0 {
            println!("{cnt}");
        }
    }
    println!("{}",result);
}