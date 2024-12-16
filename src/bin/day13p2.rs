use std::{fs, u128};

fn main() {
    let f = fs::read_to_string("day13.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();
    let mut line_idx: usize = 0;
    let mut tot: u128 = 0;
    while line_idx < lines.len() {
        let a_data = lines[line_idx].split("Button A: ").collect::<Vec<&str>>()[1];
        let a_coords = a_data.split(", ").collect::<Vec<&str>>().iter().map(|s| s[2..].parse::<u128>().unwrap()).collect::<Vec<u128>>();
        
        let b_data = lines[line_idx+1].split("Button B: ").collect::<Vec<&str>>()[1];
        let b_coords = b_data.split(", ").collect::<Vec<&str>>().iter().map(|s| s[2..].parse::<u128>().unwrap()).collect::<Vec<u128>>();

        let prize_data = lines[line_idx+2].split("Prize: ").collect::<Vec<&str>>()[1];
        let mut prize_coords = prize_data.split(", ").collect::<Vec<&str>>().iter().map(|s| s[2..].parse::<u128>().unwrap()).collect::<Vec<u128>>();
        prize_coords[0] += 10000000000000;
        prize_coords[1] += 10000000000000;

        // println!("{:?}",prize_coords);

        /*
        ax + by = e
        cx + dy = f

        (bc - ad)y = ce - af
        y = (ce - af) / (bc - ad) = (af - ce) / (ad - bc)

        (ad - bc)x = (de - bf)
        x = (de - bf) / (ad - bc)
         */

        // try to press the B button the most times


        if a_coords[0] * b_coords[1] == a_coords[1] * b_coords[0] {
            println!("equal"); // this never prints out
        }
        else if a_coords[0] * b_coords[1] > a_coords[1] * b_coords[0] {
            let denom = a_coords[0] * b_coords[1] - a_coords[1] * b_coords[0];
            if prize_coords[0] * b_coords[1] >= prize_coords[1] * b_coords[0] { //x
                if prize_coords[1] * a_coords[0] >= prize_coords[0] * a_coords[1] { //y
                    let diffx = prize_coords[0] * b_coords[1] - prize_coords[1] * b_coords[0];
                    let diffy = prize_coords[1] * a_coords[0] - prize_coords[0] * a_coords[1];
                    if diffx % denom == 0 && diffy % denom == 0 {
                        let x = diffx / denom;
                        let y = diffy / denom;
                        tot += 3*x + y;
                    }
                }
            }
        }
        else if a_coords[0] * b_coords[1] < a_coords[1] * b_coords[0] {
            let denom = a_coords[1] * b_coords[0] - a_coords[0] * b_coords[1];
            if prize_coords[0] * b_coords[1] <= prize_coords[1] * b_coords[0] { //x
                if prize_coords[1] * a_coords[0] <= prize_coords[0] * a_coords[1] { //y
                    let diffx =  prize_coords[1] * b_coords[0] - prize_coords[0] * b_coords[1];
                    let diffy = prize_coords[0] * a_coords[1] - prize_coords[1] * a_coords[0];
                    if diffx % denom == 0 && diffy % denom == 0 {
                        let x = diffx / denom;
                        let y = diffy / denom;
                        tot += 3*x + y;

                    }
                }
            }
        }

        line_idx += 4;
        
    }
    println!("{}",tot);
}