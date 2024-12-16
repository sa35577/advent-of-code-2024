use std::fs;

fn main() {
    let f = fs::read_to_string("day6.txt").unwrap();
    let contents: Vec<&str> = f.split("\n").collect();
    let contents: Vec<Vec<char>> = contents.iter().map(|line| line.chars().collect()).collect();
    let mut dir = 1;
    let mut y = 0;
    let mut x = 0;
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; contents[0].len()]; contents.len()];
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
    
    while true {
        matrix[y][x] = 1;
        if dir == 0 { //up
            if y == 0 {
                break;
            }
            else if contents[y-1][x] == '#' {
                dir += 1;
            }
            else {
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
            else {
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
            else {
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
            else {
                x -= 1;
            }
        }
    }

    let mut score = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            score += matrix[i][j];
        }
    }
    println!("{}",score);
}