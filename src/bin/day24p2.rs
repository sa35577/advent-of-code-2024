/* 
general rule:
we have two outputs for each i,

y00 XOR x00 -> z00
y00 AND x00 -> spq //carry on

y01 XOR x01 -> gpk
y01 AND x01 -> khh

spq XOR gpk -> z01 //z_i = (carry on of i-1 ^ y_i ^ x_i)
gpk AND spq -> bvk // (exactly 1 of x_i and y_i) AND (carry on of i-1)
bvk OR khh -> nmw // carry on of i = [(exactly 1 of x_i and y_i) AND (carry on of i-1)] XOR (both x_i and y_i) 


*/

use std::{collections::HashMap, fs, vec};


fn get_data(s: &str) -> (char, usize) {
    // assumes intermediate symbols dont begin with x or y or z
    let data: Vec<char> = s.chars().collect();
    if data[0] != 'x' && data[0] != 'y' && data[0] != 'z' {
        return (data[0],3000);
    }
    let dig1 = data[1].to_digit(10).unwrap();
    let dig2 = data[2].to_digit(10).unwrap();
    return (data[0], (10 * dig1 + dig2) as usize)
}
fn main() {
    // let mut evals: HashMap<&str,i32> = HashMap::new();
    let f = fs::read_to_string("day24.txt").unwrap();
    let lines: Vec<&str> = f.split("\n").collect();
    let mut parse_rules = true;
    let mut network: Vec<(&str, &str, &str, &str)> = vec![]; //(first val, operator, second val, result)
    // let mut q: Queue<usize> = Queue::new();
    // let mut finished: HashSet<usize> = HashSet::new();
    // let mut all_z: Vec<&str> = vec![];
    // let mut all_z_set: HashSet<&str> = HashSet::new();;
    let mut and_results: HashMap<usize,&str> = HashMap::new();
    let mut xor_results: HashMap<usize,&str> = HashMap::new();
    let mut ans: Vec<&str> = vec![];

    for line_idx in 0..lines.len() {
        if lines[line_idx].len() == 0 {
            parse_rules = false;
        }
        else if parse_rules {
            let data: Vec<&str> = lines[line_idx].split(": ").collect();
        }
        else {
            let sides: Vec<&str> = lines[line_idx].split(" -> ").collect();
            let data: Vec<&str> = sides[0].split(" ").collect();
            network.push((data[0],data[1],data[2],sides[1]));

            let arg1_data = get_data(data[0]);
            if arg1_data.1 != 3000 {
                if data[1] == "AND" {
                    and_results.insert(arg1_data.1, sides[1]);
                }
                else if data[1] == "XOR" {
                    xor_results.insert(arg1_data.1, sides[1]);
                }
            }

            let rhs_data = get_data(sides[1]);
            if rhs_data.1 != 3000 {
                //its a z for sure
                if data[1] != "XOR" && rhs_data.1 != 45 {
                    println!("{} {}", rhs_data.1, sides[1]);
                    ans.push(sides[1]);
                }
            }
        }
    }
    println!("^z discrepancies for not on XOR^");

    for i in 0..45 {
        let (mut or_cnt, mut xor_cnt, mut and_cnt) = (0,0,0);
        let target_str = *xor_results.get(&i).unwrap();
        let mut good = true;
        for j in 0..network.len() {
            if network[j].0 == target_str || network[j].2 == target_str {
                if network[j].1 == "AND" { and_cnt += 1; }
                if network[j].1 == "XOR" { 
                    xor_cnt += 1;
                    let parse_data = get_data(network[j].3);
                    if parse_data.0 != 'z' || parse_data.1 != i {
                        // if parse_data.0 != 'z' {
                        //     println!("{} {} ---",target_str,parse_data.0);
                        // }
                        good = false;
                    }
                }
                if network[j].1 == "OR" { or_cnt += 1; }
            }
        }
        if good && (and_cnt != 1 || or_cnt != 0 || xor_cnt != 1) {
            // println!("2 {} {} {}",and_cnt,or_cnt,xor_cnt);
            good = false;
        }
        if !good && i != 0 {
            println!("{} {}",i,target_str);
            ans.push(target_str);
        } 
    }
    println!("^xor discrepancies found^");


    for i in 0..45 {
        let (mut or_cnt, mut xor_cnt, mut and_cnt) = (0,0,0);
        let target_str = *and_results.get(&i).unwrap();
        let mut good = true;
        for j in 0..network.len() {
            if network[j].0 == target_str || network[j].2 == target_str {
                if network[j].1 == "AND" { and_cnt += 1; }
                if network[j].1 == "XOR" { xor_cnt += 1; }
                if network[j].1 == "OR" { or_cnt += 1; }
            }
        }
        if good && (and_cnt != 0 || or_cnt != 1 || xor_cnt != 0) {
            // println!("{} {} {} {}",i,and_cnt,or_cnt,xor_cnt);
            good = false;
        }
        if !good && i != 0 {
            println!("{} {}",i,target_str);
            ans.push(target_str);
        } 
    }
    println!("^and discrepancies found^");
    ans.sort();
    for i in 0..ans.len() {
        print!("{},",ans[i]);
    }
    //remove duplicate


}
//bbn,jbr,khg,pps,tvb,z12,z21,z33 wrong
//bbn,jbr,khg,pps,stq,tvb,z12,z33 last one is not rsc 
//bbn,jbr,khg,pps,tvb,z12,z33 pps swap with vdc, jbr swap with gst, add rsc?
//bbn,gst,khg,rsc,tvb,vdc,z12,z33 wrong

//answer: part2: 
//gst,khg,nhn,tvb,vdc,z12,z21,z33
//my output: bbn,jbr,khg,pps,tvb,z12,z12,z21,z33
//take out dup z12, swap pps (12) with what should be z12: vdc
//swap jbr (33) with what should be z33: rsc
//swap bbn (21) with what should be z21: nhn
//gst,khg,nhn,tvb,vdc,z12,z21,z33