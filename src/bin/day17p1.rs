use std::fs;
fn main() {
    let f = fs::read_to_string("day17.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let program: Vec<i32> = lines[4].split("Program: ").collect::<Vec<&str>>()[1].split(",").map(|s| s.parse().unwrap()).collect();
    println!("{:?}",program);

    let mut a = 27334280;
    let mut b = 0;
    let mut c = 0;
    let mut ip: usize = 0;

    let combo = |x: i32, a: i32, b: i32, c: i32| -> i32 {
        if x <= 3 {
            return x;
        }
        if x == 4 {
            return a;
        }
        if x == 5 {
            return b;
        }
        if x == 6 {
            return c;
        }
        println!("ajkdsf;askdlfj {x}");
        return -1;
    };

    let mut res: Vec<i32> = vec![];

    while ip < program.len() {
        let ins = program[ip];
        if ins == 0 {
            let res = combo(program[ip+1], a, b, c);
            if res >= 0 {
                a = a / 2_i32.pow(res as u32);
            }
            else {
                a = a * 2_i32.pow((-res) as u32);
            }
            
            ip += 2;
        }
        else if ins == 1 {
            b = b ^ program[ip+1];
            ip += 2;
        }
        else if ins == 2 {
            b = combo(program[ip+1], a, b, c) % 8;
            ip += 2;
        }
        else if ins == 3 {
            if a == 0 {
                ip += 2;
            }
            else {
                ip = program[ip+1] as usize;
            }
        }
        else if ins == 4 {
            b = b ^ c;
            ip += 2;
        }
        else if ins == 5 {
            res.push(combo(program[ip+1], a, b, c) % 8);
            ip += 2;
        }
        else if ins == 6 {
            let res = combo(program[ip+1], a, b, c);
            if res >= 0 {
                b = a / 2_i32.pow(res as u32);
            }
            else {
                b = a * 2_i32.pow((-res) as u32);
            }
            ip += 2;
        }
        else if ins == 7 {
            let res = combo(program[ip+1], a, b, c);
            if res >= 0 {
                c = a / 2_i32.pow(res as u32);
            }
            else {
                c = a * 2_i32.pow((-res) as u32);
            }
            ip += 2;
        }

    }
    
    for i in 0..res.len() {
        print!("{},",res[i]);
    }



}