use std::fs;

fn main() {
    let f = fs::read_to_string("day14.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();
    println!("{}",lines.len());

    let xlen = 101;
    let ylen = 103;

    let mut pos: Vec<(i32, i32)> = vec![];
    let mut vel: Vec<(i32, i32)> = vec![];

    for line in lines {
        let data = line.split(" ").collect::<Vec<&str>>().iter().map(|s| s.split("=").collect::<Vec<&str>>()[1]).collect::<Vec<&str>>();
        let poso: Vec<i32> = data[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        let velo: Vec<i32> = data[1].split(",").map(|n| n.parse::<i32>().unwrap()).collect();

        pos.push((poso[0], poso[1]));
        vel.push((velo[0], velo[1]));
    }


    for iteration in 0..10403 {
        let (mut q1,mut q2,mut q3,mut q4) = (0,0,0,0);
        for i in 0..pos.len() {
            pos[i].0 = ((pos[i].0 + vel[i].0) % xlen + xlen) % xlen;
            pos[i].1 = ((pos[i].1 + vel[i].1) % ylen + ylen) % ylen;

            if pos[i].0 > 50 && pos[i].1 < 51 {
                q1 += 1;
            }
            else if pos[i].0 > 50 && pos[i].1 > 51 {
                q4 += 1;
            }
            else if pos[i].0 < 50 && pos[i].1 < 51 {
                q2 += 1;
            }
            else if pos[i].0 < 50 && pos[i].1 > 51 {
                q3 += 1;
            }
        }
        if iteration == 7568 {
            // println!("{}",iteration);
            let mut map: Vec<Vec<char>> = vec![vec![' '; 101]; 103];
            for i in 0..pos.len() {
                if pos[i].0 < 50 {
                    map[pos[i].1 as usize][pos[i].0 as usize] = '/';
                }
                else {
                    map[pos[i].1 as usize][pos[i].0 as usize] = '\\';
                }
            }
            let mut good = false;
            for i in 0..map.len() {
                let mut cnt = 0;
                for c in &map[i] {
                    if *c != ' ' {
                        cnt += 1;
                    }
                }
                if cnt >= 13 {
                    good = true;
                    break;
                }
            }
            if good {
                println!("{iteration}");
                for i in 0..map.len() {
                    for c in &map[i] {
                        print!("{}", c);
                    }
                    println!();
                }
                for i in 0..5 { println!("") }
            }
            //7568 had a good print out. answer is thus one more.
            println!("{} {} {} {}",q1,q2,q3,q4);
        }
    }
}