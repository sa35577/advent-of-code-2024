use std::{fs, vec};

fn main() {
    /*
    controlling a directional keypad and getting its length is really easy.
    score for each key: 2 * (A=0, up=1, right=1, down=2, left=3)
    controlling a number keypad is also really easy.
     */

    let f = fs::read_to_string("day21.txt").unwrap();
    let lines: Vec<Vec<char>> = f.split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect();
    let mut positions: Vec<(usize, usize)> = vec![];
    positions.push((3,1));
    positions.push((2,0));
    positions.push((2,1));
    positions.push((2,2));
    positions.push((1,0));
    positions.push((1,1));
    positions.push((1,2));
    positions.push((0,0));
    positions.push((0,1));
    positions.push((0,2));
    positions.push((3,2));

    //up=0,right=1,down=2,left=3,A=4
    let mut robotPositions: Vec<(usize, usize)> = vec![];
    robotPositions.push((0,1));
    robotPositions.push((1,2));
    robotPositions.push((1,1));
    robotPositions.push((1,0));
    robotPositions.push((0,2));

    //go right first, then go down, then go up, then go left
    let mut tot = 0;
    

    for input in lines {
        
        let mut sy: usize = 3;
        let mut sx: usize = 2;
        let mut robot1: Vec<usize> = vec![];

        for cc in &input {
            let c = *cc;
            let mut x = 10;
            if c != 'A' {
                x = c.to_digit(10).unwrap() as usize;
            }
            let ey = positions[x].0;
            let ex = positions[x].1;
            let mut doLeftFirst = true;
            if sy == 3 && ex == 0 {
                doLeftFirst = false;
            }
            let mut doDownFirst = true;
            if sx == 0 && ey == 3 {
                doDownFirst = false;
            }
            if doLeftFirst && ex < sx {
                for i in 0..sx-ex {
                    robot1.push(3);
                }
            }
            if doDownFirst && ey > sy {
                for i in 0..ey-sy {
                    robot1.push(2);
                }
            }
            // right
            if ex > sx {
                for i in 0..ex-sx {
                    robot1.push(1);
                }
            }
            // up
            if ey < sy {
                for i in 0..sy-ey {
                    robot1.push(0);
                }
            }

            // left
            if !doLeftFirst && ex < sx {
                for i in 0..sx-ex {
                    robot1.push(3);
                }
            }

            // down
            if !doDownFirst && ey > sy {
                for i in 0..ey-sy {
                    robot1.push(2);
                }
            }
            robot1.push(4);
            sy = ey;
            sx = ex;
        }
        for cc in &robot1 {
            let c = *cc;
            if c == 0 {
                print!("^");
            }
            if c == 1 {
                print!(">");
            }
            if c == 2 {
                print!("v");
            }
            if c == 3 {
                print!("<");
            }
            if c == 4 {
                print!("A");
            }
        }
        println!();
        sy = 0;
        sx = 2;
        let mut robot2: Vec<usize> = vec![];
        for cc in &robot1 {
            let c = *cc;
            let ey = robotPositions[c].0;
            let ex = robotPositions[c].1;

            // right
            if ex > sx {
                for i in 0..ex-sx {
                    robot2.push(1);
                }
            }

            // down
            if ey > sy {
                for i in 0..ey-sy {
                    robot2.push(2);
                }
            }

            // up
            if ey < sy {
                for i in 0..sy-ey {
                    robot2.push(0);
                }
            }

            // left
            if ex < sx {
                for i in 0..sx-ex {
                    robot2.push(3);
                }
            }

            
            robot2.push(4);
            sy = ey;
            sx = ex;
        }
        // println!("{:?}",robot2);
        for cc in &robot2 {
            let c = *cc;
            if c == 0 {
                print!("^");
            }
            if c == 1 {
                print!(">");
            }
            if c == 2 {
                print!("v");
            }
            if c == 3 {
                print!("<");
            }
            if c == 4 {
                print!("A");
            }
        }
        println!();
        sy = 0;
        sx = 2;
        let mut robot3: Vec<usize> = vec![];
        for cc in &robot2 {
            let c = *cc;
            let ey = robotPositions[c].0;
            let ex = robotPositions[c].1;

            // right
            if ex > sx {
                for i in 0..ex-sx {
                    robot3.push(1);
                }
            }

            // down
            if ey > sy {
                for i in 0..ey-sy {
                    robot3.push(2);
                }
            }

            // up
            if ey < sy {
                for i in 0..sy-ey {
                    robot3.push(0);
                }
            }

            // left
            if ex < sx {
                for i in 0..sx-ex {
                    robot3.push(3);
                }
            }

            
            robot3.push(4);
            sy = ey;
            sx = ex;
        }
        println!("{}",robot3.len());
        for cc in &robot3 {
            let c = *cc;
            if c == 0 {
                print!("^");
            }
            if c == 1 {
                print!(">");
            }
            if c == 2 {
                print!("v");
            }
            if c == 3 {
                print!("<");
            }
            if c == 4 {
                print!("A");
            }
        }
        println!();
        let mut parseVal = 0;
        for cc in &input {
            let c = *cc;
            if c != 'A' {
                parseVal = parseVal*10 + c.to_digit(10).unwrap();
            }
        }
        println!("{}",parseVal);
        tot += (robot3.len() as u32)*parseVal;

    }
    println!("{}",tot);

    




}


// v<<A>>^AvA^A v<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
// <v<A>>^AvA^A <vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A



