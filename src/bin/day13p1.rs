use std::fs;

fn main() {
    let f = fs::read_to_string("day13.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();
    let mut line_idx: usize = 0;
    let mut tot = 0;
    while line_idx < lines.len() {
        let a_data = lines[line_idx].split("Button A: ").collect::<Vec<&str>>()[1];
        let a_coords = a_data.split(", ").collect::<Vec<&str>>().iter().map(|s| s[2..].parse::<i32>().unwrap()).collect::<Vec<i32>>();
        
        let b_data = lines[line_idx+1].split("Button B: ").collect::<Vec<&str>>()[1];
        let b_coords = b_data.split(", ").collect::<Vec<&str>>().iter().map(|s| s[2..].parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let prize_data = lines[line_idx+2].split("Prize: ").collect::<Vec<&str>>()[1];
        let prize_coords = prize_data.split(", ").collect::<Vec<&str>>().iter().map(|s| s[2..].parse::<i32>().unwrap()).collect::<Vec<i32>>();

        // try to press the B button the most times
        let mut result = 3000000;
        for b in 0..101 {
            let rem0 = prize_coords[0] - b*b_coords[0];
            let rem1 = prize_coords[1] - b*b_coords[1];
            if rem0 >= 0 && rem1 >= 0 {
                if rem0 % a_coords[0] == 0 && rem1 % a_coords[1] == 0 && rem0/a_coords[0] == rem1/a_coords[1] {
                    let a = rem0/a_coords[0];
                    if a <= 100 && result > 3*a+b {
                        result = 3*a+b;
                    }
                }
            }
        }
        if result != 3000000 {
            tot += result;
        }
        line_idx += 4;
    }
    println!("{}",tot);
}