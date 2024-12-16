use std::fs;

fn main() {
    let contents = fs::read_to_string("day4.txt").unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut result = 0;
    let lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    for i in 1..lines.len()-1 {
        for j in 1..lines[0].len()-1 {
            if lines[i][j] == 'A' {
                if lines[i+1][j+1] == 'M' && lines[i-1][j-1] == 'S' {
                    if lines[i+1][j-1] == 'M' && lines[i-1][j+1] == 'S' {
                        println!("{i} {j}");
                        result += 1;
                    }
                    else if lines[i+1][j-1] == 'S' && lines[i-1][j+1] == 'M' {
                        result += 1;
                        println!("{i} {j}");
                    }
                }
                else if lines[i+1][j+1] == 'S' && lines[i-1][j-1] == 'M' {
                    if lines[i+1][j-1] == 'M' && lines[i-1][j+1] == 'S' {
                        println!("{i} {j}");
                        result += 1;
                    }
                    else if lines[i+1][j-1] == 'S' && lines[i-1][j+1] == 'M' {
                        result += 1;
                        println!("{i} {j}");
                    }
                }
            }
        }
    }
    
    println!("{result}");
}