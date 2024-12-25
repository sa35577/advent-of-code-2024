use std::{collections::HashMap, fs};
fn main() {
    let f = fs::read_to_string("day17.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();

    let program: Vec<u128> = lines[4].split("Program: ").collect::<Vec<&str>>()[1].split(",").map(|s| s.parse().unwrap()).collect();
    println!("{:?}",program);

    let combo = |x: u128, a: u128, b: u128, c: u128| -> u128 {
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
        return 66666666666666;
    };

    // let mut hm: HashMap<u128, u128> = HashMap::new();

    // let mut aa: u128 = 0;
    let mut possibles: Vec<u128> = vec![];
    possibles.push(0);
    for i in 0..program.len() {
        let idx = program.len()-1-i;
        let mut newPossibles: Vec<u128> = vec![];

        for aa in possibles {
            for add_val in 0..8 {
                let mut a: u128 = aa*8+add_val;
                let mut b: u128 = 0;
                let mut c = 0;
                let mut ip: usize = 0;
                let mut valid: bool = true;
    
                while ip < program.len() {
                    let ins = program[ip];
                    if ins == 0 {
                        let r = combo(program[ip+1], a, b, c);
                        if r == 66666666666666 {
                            valid = false;
                            break;
                        }
                        if r >= 0 {
                            a = a / 2_u128.pow(r as u32);
                        }
                        
                        ip += 2;
                    }
                    else if ins == 1 {
                        b = b ^ program[ip+1];
                        ip += 2;
                    }
                    else if ins == 2 {
                        let res = combo(program[ip+1], a, b, c);
                        if res == 66666666666666 {
                            valid = false;
                            break;
                        }
                        b = res % 8;
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
                        let r = combo(program[ip+1], a, b, c);
                        if r == 66666666666666 {
                            valid = false;
                            break;
                        }
                        if r%8 != program[idx] {
                            valid = false;
                            break;
                        }
                        else {
                            break;
                        }
                        ip += 2;
                    }
                    else if ins == 6 {
                        let r = combo(program[ip+1], a, b, c);
                        if r == 66666666666666 {
                            valid=false;
                            break;
                        }
                        if r >= 0 {
                            b = a / 2_u128.pow(r as u32);
                        }
                        ip += 2;
                    }
                    else if ins == 7 {
                        let r = combo(program[ip+1], a, b, c);
                        if r == 66666666666666 {
                            valid=false;
                            break;
                        }
                        if r >= 0 {
                            c = a / 2_u128.pow(r as u32);
                        }
                        ip += 2;
                    }
        
                }
                if valid {
                    // println!("Found {add_val} for step {i}");
                    newPossibles.push(aa*8+add_val);
                    // println!("{aa}");
                }
            }
        }
        possibles = newPossibles;
        println!("{:?}",possibles);


        
        
        // for j in 0..8 {
        //     if hm.contains_key(&j) && *hm.get(&j).unwrap() == program[idx] {
        //         target_val = j;
        //         break;
        //     }
        // }
        // a = a * 8_u128 + target_val;
        // println!("{}",a);
    }
    // println!("{}",a*8_u128);
    let mut ans = u128::MAX;
    for p in possibles {
        if ans > p {
            ans = p;
        }
    }
    println!("{ans}");

    for aa in 0..1 {
        let mut a: u128 = ans;
        let mut b: u128 = 0;
        let mut c = 0;
        let mut ip: usize = 0;
        let mut valid: bool = true;

        let mut res: Vec<u128> = vec![];

        while ip < program.len() {
            let ins = program[ip];
            if ins == 0 {
                let r = combo(program[ip+1], a, b, c);
                if r == 66666666666666 {
                    valid = false;
                    // break;
                }
                if r >= 0 {
                    a = a / 2_u128.pow(r as u32);
                }
                // else {
                //     a = a * 2_u128.pow((-r) as u32);
                // }
                
                ip += 2;
            }
            else if ins == 1 {
                b = b ^ program[ip+1];
                ip += 2;
            }
            else if ins == 2 {
                let res = combo(program[ip+1], a, b, c);
                if res == 66666666666666 {
                    valid = false;
                    // break;
                }
                b = res % 8;
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
                let r = combo(program[ip+1], a, b, c);
                if r == 66666666666666 {
                    valid = false;
                    // break;
                }
                res.push(r % 8);
                if res.len() > program.len() {
                    valid=false;
                    // break;
                }
                ip += 2;
            }
            else if ins == 6 {
                let r = combo(program[ip+1], a, b, c);
                if r == 66666666666666 {
                    valid=false;
                    // break;
                }
                if r >= 0 {
                    b = a / 2_u128.pow(r as u32);
                }
                // else {
                //     b = a * 2_u128.pow((-r) as u32);
                // }
                ip += 2;
            }
            else if ins == 7 {
                let r = combo(program[ip+1], a, b, c);
                if r == 66666666666666 {
                    valid=false;
                    // break;
                }
                if r >= 0 {
                    c = a / 2_u128.pow(r as u32);
                }
                // else {
                //     c = a * 2_u128.pow((-r) as u32);
                // }
                ip += 2;
            }

            // if res.len() > 0 && res[res.len()-1] != program[res.len()-1] {
            //     valid = false;
            //     break;
            // }

        }
        // if valid && res.len() == program.len() {
        //     println!("{aa} valid!!");
        // }
        
        for i in 0..res.len() {
            print!("{},",res[i]);
        }
        // println!("----");
    }
    



}