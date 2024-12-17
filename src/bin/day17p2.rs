use std::{collections::HashMap, fs};
fn main() {
    let f = fs::read_to_string("day17.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let program: Vec<i32> = lines[4].split("Program: ").collect::<Vec<&str>>()[1].split(",").map(|s| s.parse().unwrap()).collect();
    println!("{:?}",program);

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

    let mut hm: HashMap<i32, u128> = HashMap::new();

    let mut a: u128 = 0;

    // [(a%8 xor 5) xor floor(a / 2^(a%8 xor 2))]
    for i in 0..8 {
        let v1 = (i%8) ^ 5;
        let v2 = i / 2_u32.pow((i%8) ^ 2);
        let v = v1 ^ v2;
        if i != 2 {
            hm.insert( v as i32, i as u128);
        }
        println!("{i} {v}");

        
    }

    for i in 0..program.len()-1 {
        let idx = program.len()-1-i;
        let target_val: u128 = *hm.get(&program[idx]).unwrap();
        a = a * 8_u128 + target_val;
        println!("{}",a);
    }
    println!("{}",a*8_u128);

    // for aa in 0..1167334280 {
    //     let mut a = aa;
    //     let mut b = 0;
    //     let mut c = 0;
    //     let mut ip: usize = 0;
    //     let mut valid: bool = true;

    //     let mut res: Vec<i32> = vec![];

    //     while ip < program.len() {
    //         let ins = program[ip];
    //         if ins == 0 {
    //             let r = combo(program[ip+1], a, b, c);
    //             if r == -1 {
    //                 valid = false;
    //                 break;
    //             }
    //             if r >= 0 {
    //                 a = a / 2_i32.pow(r as u32);
    //             }
    //             else {
    //                 a = a * 2_i32.pow((-r) as u32);
    //             }
                
    //             ip += 2;
    //         }
    //         else if ins == 1 {
    //             b = b ^ program[ip+1];
    //             ip += 2;
    //         }
    //         else if ins == 2 {
    //             let res = combo(program[ip+1], a, b, c);
    //             if res == -1 {
    //                 valid = false;
    //                 break;
    //             }
    //             b = res % 8;
    //             ip += 2;
    //         }
    //         else if ins == 3 {
    //             if a == 0 {
    //                 ip += 2;
    //             }
    //             else {
    //                 ip = program[ip+1] as usize;
    //             }
    //         }
    //         else if ins == 4 {
    //             b = b ^ c;
    //             ip += 2;
    //         }
    //         else if ins == 5 {
    //             let r = combo(program[ip+1], a, b, c);
    //             if r == -1 {
    //                 valid = false;
    //                 break;
    //             }
    //             res.push(r % 8);
    //             if res.len() > program.len() {
    //                 valid=false;
    //                 break;
    //             }
    //             ip += 2;
    //         }
    //         else if ins == 6 {
    //             let r = combo(program[ip+1], a, b, c);
    //             if r == -1 {
    //                 valid=false;
    //                 break;
    //             }
    //             if r >= 0 {
    //                 b = a / 2_i32.pow(r as u32);
    //             }
    //             else {
    //                 b = a * 2_i32.pow((-r) as u32);
    //             }
    //             ip += 2;
    //         }
    //         else if ins == 7 {
    //             let r = combo(program[ip+1], a, b, c);
    //             if r == -1 {
    //                 valid=false;
    //                 break;
    //             }
    //             if r >= 0 {
    //                 c = a / 2_i32.pow(r as u32);
    //             }
    //             else {
    //                 c = a * 2_i32.pow((-r) as u32);
    //             }
    //             ip += 2;
    //         }

    //         if res.len() > 0 && res[res.len()-1] != program[res.len()-1] {
    //             valid = false;
    //             break;
    //         }

    //     }
    //     if valid && res.len() == program.len() {
    //         println!("{aa} valid!!");
    //     }
        
    //     // for i in 0..res.len() {
    //     //     print!("{},",res[i]);
    //     // }
    //     // println!("----");
    // }

    



}