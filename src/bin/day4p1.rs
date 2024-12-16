use std::fs;

fn main() {
    let contents = fs::read_to_string("day4.txt").unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    let xmas = "XMAS";
    let xmas_rev = "SAMX";
    let mut result = 0;
    for line in &lines {
        // println!("{line}");
        for i in 0..line.len()-3 {
            if line[i..i+4] == *xmas || line[i..i+4] == *xmas_rev {
                result += 1;
            }
        }
    }

    let lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    
    for i in 0..lines.len()-3 {
        for j in 0..lines[0].len() {
            if lines[i][j] == 'X' && lines[i+1][j] == 'M' && lines[i+2][j] == 'A' && lines[i+3][j] == 'S' {
                result += 1;
            }
            if lines[i][j] == 'S' && lines[i+1][j] == 'A' && lines[i+2][j] == 'M' && lines[i+3][j] == 'X' {
                result += 1;
            }
            if j+3 < lines[0].len() && lines[i][j] == 'X' && lines[i+1][j+1] == 'M' && lines[i+2][j+2] == 'A' && lines[i+3][j+3] == 'S' {
                result += 1;
            }
            if j+3 < lines[0].len() && lines[i][j] == 'S' && lines[i+1][j+1] == 'A' && lines[i+2][j+2] == 'M' && lines[i+3][j+3] == 'X' {
                result += 1;
            }
            if j >= 3 && lines[i][j] == 'X' && lines[i+1][j-1] == 'M' && lines[i+2][j-2] == 'A' && lines[i+3][j-3] == 'S' {
                result += 1;
            }
            if j >= 3 && lines[i][j] == 'S' && lines[i+1][j-1] == 'A' && lines[i+2][j-2] == 'M' && lines[i+3][j-3] == 'X' {
                result += 1;
            }
        }
    }
    println!("{result}");
}