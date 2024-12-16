use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("day3.txt").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result: i32 = 0;

    let do_str = "do()";
    let dont_str = "don't()";
    let mut indexes: Vec<(usize, i32)> = vec![];
    let mut iptr = 0;
    let mut enabled = true;

    for i in 0..contents.len() {
        if i+do_str.len() <= contents.len() && contents[i..i+do_str.len()] == *do_str {
            println!("{i}");
            indexes.push((i,0));
        }
        if i+dont_str.len() <= contents.len() && contents[i..i+dont_str.len()] == *dont_str {
            println!("{i}");
            indexes.push((i,1));
        }
    }

    println!("{:?}",indexes);
    for mat in re.captures_iter(&contents) {
        let full_match = mat.get(0).unwrap(); // The full match
        let start_index = full_match.start(); // Starting index of the match

        let n1 = mat[1].parse::<i32>().unwrap();
        let n2 = mat[2].parse::<i32>().unwrap();

        while iptr < indexes.len() && indexes[iptr].0 < start_index {
            if indexes[iptr].1 == 1 {
                enabled = false;
            }
            else {
                enabled = true;
            }
            iptr += 1;
        }
        
        if enabled {
            result = result + n1 * n2;
        }
        
    }
    println!("{result}");

}