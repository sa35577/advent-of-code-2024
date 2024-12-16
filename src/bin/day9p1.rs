use std::fs;

fn main() {
    let s = fs::read_to_string("day9.txt").unwrap();
    let s: Vec<char> = s.chars().collect();
    let mut id = 0;
    let mut v: Vec<i32> = vec![];
    let mut cnt = 0;

    for i in 0..s.len() {
        let val: i32 = s[i].to_digit(10 as u32).unwrap() as i32;
        if i % 2 == 0 {
            for _ in 0..val {
                v.push(id);
            }
            cnt += val;
            id += 1;
        }
        else {
            for _ in 0..val {
                v.push(-1);
            }
        }
    }

    let mut rptr = v.len()-1;
    let mut tot: u64 = 0;

    for i in 0..cnt {
        let ii: usize = i as usize;
        if v[ii] == -1 {
            while v[rptr] == -1 {
                rptr -= 1;
            }
            v[ii] = v[rptr];
            v[rptr] = -1;
        }
        tot += (v[ii] as u64) * (i as u64);
    }
    print!("{tot}");


}