use std::fs;

fn main() {
    let f = fs::read_to_string("day14.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let xlen = 101;
    let ylen = 103;

    let (mut q1,mut q2,mut q3,mut q4) = (0,0,0,0);

    for line in lines {
        let data = line.split(" ").collect::<Vec<&str>>().iter().map(|s| s.split("=").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>();
        let mut pos: Vec<i32> = data[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        let vel: Vec<i32> = data[1].split(",").map(|n| n.parse::<i32>().unwrap()).collect();

        pos[0] = ((pos[0] + 100 * vel[0]) % xlen + xlen) % xlen;
        pos[1] = ((pos[1] + 100 * vel[1]) % ylen + ylen) % ylen;


        if pos[0] > 50 && pos[1] < 51 {
            q1 += 1;
        }
        else if pos[0] > 50 && pos[1] > 51 {
            q4 += 1;
        }
        else if pos[0] < 50 && pos[1] < 51 {
            q2 += 1;
        }
        else if pos[0] < 50 && pos[1] > 51 {
            q3 += 1;
        }
        
    }
    println!("{} {} {} {}",q1,q2,q3,q4);
    println!("{}",q1*q2*q3*q4);
}