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

    /*
    every sequence can be encoded as (prev_char, cur_char) -> freq mappings for every combination of prev and cur char.
    initial sequence is (A,s[0]),(s[0],s[1]),...,(s[m-1],s[m]) -> group in this encoding
    say the route from prev_char to cur_char is r[0],...,r[n].
    for the next sequence, you add freq to: (A,r[0]), (r[0], r[1]), ..., (r[m-1],r[m]), (r[m],A)
     */


    
    let mut localtot: u128 = 0;
    for input in lines {
        
        let mut sy: usize = 3;
        let mut sx: usize = 2;
        let mut robot1: Vec<usize> = vec![];
        let mut freq: Vec<Vec<u128>> = vec![vec![0; 5]; 5];

        for cc in &input {
            let c = *cc;
            let mut x = 10;
            if c != 'A' {
                x = c.to_digit(10).unwrap() as usize;
            }
            let ey = positions[x].0;
            let ex = positions[x].1;
            // let mut doLeftFirst = true;
            // if sy == 3 && ex == 0 {
            //     doLeftFirst = false;
            // }
            // let mut doDownFirst = true;
            // if sx == 0 && ey == 3 {
            //     doDownFirst = false;
            // }
            // if doLeftFirst && ex < sx {
            //     for i in 0..sx-ex {
            //         robot1.push(3);
            //     }
            // }
            // if doDownFirst && ey > sy {
            //     for i in 0..ey-sy {
            //         robot1.push(2);
            //     }
            // }
            // // right
            // if ex > sx {
            //     for i in 0..ex-sx {
            //         robot1.push(1);
            //     }
            // }
            // // up
            // if ey < sy {
            //     for i in 0..sy-ey {
            //         robot1.push(0);
            //     }
            // }

            // // left
            // if !doLeftFirst && ex < sx {
            //     for i in 0..sx-ex {
            //         robot1.push(3);
            //     }
            // }

            // // down
            // if !doDownFirst && ey > sy {
            //     for i in 0..ey-sy {
            //         robot1.push(2);
            //     }
            // }
            //---
            if sy == 3 && ex == 0 {
                //updown + leftright
                // up
                if ey < sy {
                    for i in 0..sy-ey {
                        robot1.push(0);
                    }
                }
                // down
                if ey > sy {
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
                //left
                if ex < sx {
                    for i in 0..sx-ex {
                        robot1.push(3);
                    }
                }
                
            }
            else if sx == 0 && ey == 3 {
                //leftright + updown
                // right
                if ex > sx {
                    for i in 0..ex-sx {
                        robot1.push(1);
                    }
                }
                //left
                if ex < sx {
                    for i in 0..sx-ex {
                        robot1.push(3);
                    }
                }
                // up
                if ey < sy {
                    for i in 0..sy-ey {
                        robot1.push(0);
                    }
                }
                // down
                if ey > sy {
                    for i in 0..ey-sy {
                        robot1.push(2);
                    }
                }
            }
            else {
                if ex >= sx {
                    //right
                    if ey >= sy {
                        //down
                        for i in 0..ey-sy {
                            robot1.push(2);
                        }
                    }
                    else {
                        //up
                        for i in 0..sy-ey {
                            robot1.push(0);
                        }
                    }
                    for i in 0..ex-sx {
                        robot1.push(1);  
                    }
                }
                else {
                    //left
                    for i in 0..sx-ex {
                        robot1.push(3);
                    }
                    if ey >= sy {
                        //down
                        for i in 0..ey-sy {
                            robot1.push(2);
                        }
                    }
                    else {
                        //up
                        for i in 0..sy-ey {
                            robot1.push(0);
                        }
                    }
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
        println!("{}",robot1.len());
        for i in 1..robot1.len() {
            freq[robot1[i-1]][robot1[i]] += 1;
        }
        freq[4][robot1[0]] += 1;
        // println!("{:?}",freq);
        localtot = 0;
        for i in 0..5 {
            for j in 0..5 {
                localtot += freq[i][j];
            }
        }
        println!("{}",localtot);

        for _iteration_value in 0..25 {
            let mut newfreq: Vec<Vec<u128>> = vec![vec![0; 5]; 5];
            for i in 0..5 {
                for j in 0..5 {
                    let sy = robotPositions[i].0;
                    let sx = robotPositions[i].1;
                    let ey = robotPositions[j].0;
                    let ex = robotPositions[j].1;
                    let mut robot2: Vec<usize> = vec![]; // the steps to get from (sy,sx) -> (ey,ex)
                    robot2.push(4);
                    //exclusive rule
                    if sx == 0 {
                        // right
                        if ex > sx {
                            for i in 0..ex-sx {
                                robot2.push(1);
                            }
                        }
                        if ex < sx {
                            for i in 0..sx-ex {
                                robot2.push(3);
                            }
                        }
                        // up
                        if ey < sy {
                            for i in 0..sy-ey {
                                robot2.push(0);
                            }
                        }
                        // down
                        if ey > sy {
                            for i in 0..ey-sy {
                                robot2.push(2);
                            }
                        }
                    }
                    //exclusive rule
                    else if ex == 0 {
                        // up
                        if ey < sy {
                            for i in 0..sy-ey {
                                robot2.push(0);
                            }
                        }
                        // down
                        if ey > sy {
                            for i in 0..ey-sy {
                                robot2.push(2);
                            }
                        }
                        // right
                        if ex > sx {
                            for i in 0..ex-sx {
                                robot2.push(1);
                            }
                        }
                        // left
                        if ex < sx {
                            for i in 0..sx-ex {
                                robot2.push(3);
                            }
                        }
                    }
                    else {
                        // right
                        if ex >= sx {
                            //right
                            if ey >= sy {
                                //down
                                for i in 0..ey-sy {
                                    robot2.push(2);
                                }
                            }
                            else {
                                //up
                                for i in 0..sy-ey {
                                    robot2.push(0);
                                }
                            }
                            for i in 0..ex-sx {
                                robot2.push(1);  
                            }
                        }
                        else {
                            //left
                            for i in 0..sx-ex {
                                robot2.push(3);
                            }
                            if ey >= sy {
                                //down
                                for i in 0..ey-sy {
                                    robot2.push(2);
                                }
                            }
                            else {
                                //up
                                for i in 0..sy-ey {
                                    robot2.push(0);
                                }
                            }
                        }
                    }
                    
                    robot2.push(4);
                    for k in 1..robot2.len() {
                        newfreq[robot2[k-1]][robot2[k]] += freq[i][j];
                    }
                }
            }
            freq = newfreq;
            localtot = 0;
            for i in 0..5 {
                for j in 0..5 {
                    localtot += freq[i][j];
                }
            }
            println!("{}",localtot);

        }


        let mut parseVal: u128 = 0;
        for cc in &input {
            let c = *cc;
            if c != 'A' {
                parseVal = parseVal*10 + (c.to_digit(10).unwrap() as u128);
            }
        }
        println!("{}",parseVal);
        tot += localtot*parseVal;
        println!("jasdfjkl;aksdjf;alsdjfa;lskdfjas;lkjdfa;sdlfkas;dlfkas;ldfjkas;dlkfjasdl;jfksadf");

    }
    println!("{}",tot);

    

// too high: 416181745525478
// too high:1068802524908690
// too low:  162057294272602

// too low:  196449080607436
// propose:  195664513288128

}
