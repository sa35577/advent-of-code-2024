use std::fs;

fn main() {
    let s = fs::read_to_string("day9.txt").unwrap();
    let s: Vec<char> = s.chars().collect();
    let mut id = 0;
    let mut vals: Vec<(i32, i32, i32)> = vec![]; // id, startindex, length
    let mut frees: Vec<(i32, i32, i32)> = vec![]; //-1, startindex, length
    let mut cnt = 0;
    let mut idx = 0;

    for i in 0..s.len() {
        let val: i32 = s[i].to_digit(10 as u32).unwrap() as i32;
        if i % 2 == 0 {
            vals.push((id, idx, val));
            cnt += val;
            idx += val;
            id += 1;
        }
        else {
            frees.push((-1,idx,val));
            idx += val;
        }
    }

    let mut tot: u64 = 0;
    let mut rptr = vals.len()-1;
    while true {
        for i in 0..frees.len() {
            if frees[i].1 > vals[rptr].1 {
                break;
            }
            if frees[i].2 >= vals[rptr].2 {
                vals[rptr].1 = frees[i].1; //move vals over
                frees[i].1 += vals[rptr].2; //change starting index of free block
                frees[i].2 -= vals[rptr].2;
                break;
            }
        }
        if rptr == 0 {
            break;
        }
        rptr -= 1;
    }

    for i in 0..vals.len() {
        let s: u64 = vals[i].1 as u64;
        let e: u64 = (vals[i].1 + vals[i].2 - 1) as u64;
        println!("{s} {e}");
        let res: u64 = (vals[i].0 as u64) * ((s + e) * (vals[i].2 as u64)) / (2 as u64);
        tot += res;

    }
    println!("{tot}");
}