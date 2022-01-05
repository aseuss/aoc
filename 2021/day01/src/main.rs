use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = "./input";
    part_one(path);
    part_two(path);
}

fn part_one(path : &str) {
    let mut count = 0;
    let mut d_old = u32::MAX; // no reading can be larger than u32::MAX, used to ignore first line
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(depth) = line {
                let d = depth.parse::<u32>().unwrap();
                if d > d_old {
                    count = count + 1;
                }
                d_old = d;
            }
        }
    }
    println!("part one: {} increments", count);
}


// A: 0 1 2
// B: 1 2 3
// C: 2 3 4
// D: 0 1 2
fn part_two(path: &str) {
    let mut line_number = 0;
    let mut a_counter = 0;
    let mut b_counter = 0;
    let mut c_counter = 0;
    let mut increment_count = 0;
    let mut last = u32::MAX; // no reading can be larger than u32::MAX, used to ignore first line
    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(depth) = line {
                let d = depth.parse::<u32>().unwrap();

                if a_count == 0 {
                    a_counter = d;
                } else {
                    a_counter = a_counter + d;
                }
                if a_count == 2 {
                    if a_counter > last {
                        increment_count = increment_count + 1;
                        println!("last [+]: {}", last);
                    } else if a_counter == last {
                        println!("last [=]: {}", last);
                    } else {
                        println!("last [-]: {}", last);
                    }
                    last = a_counter;
                    a_count = 0;
                } else {
                    a_count = a_count + 1;
                }

                if (line_number > 0) {
                    if b_count == 0 {
                        b_counter = d;
                    } else {
                        b_counter = b_counter + d;
                    }
                    if b_count == 2 {
                        if b_counter > last {
                            increment_count = increment_count + 1;
                            println!("last [+]: {}", last);
                        } else if b_counter == last {
                            println!("last [=]: {}", last);
                        } else {
                            println!("last [-]: {}", last);
                        }
                        last = b_counter;
                        b_count = 0;
                    } else {
                        b_count = b_count + 1;
                    }
                }

                if (line_number > 1) {
                    if c_count == 0 {
                        c_counter = d;
                    } else {
                        c_counter = c_counter + d;
                    }
                    if c_count == 2 {
                        if c_counter > last {
                            increment_count = increment_count + 1;
                            println!("last [+]: {}", last);
                        } else if c_counter == last {
                            println!("last [=]: {}", last);
                        } else {
                            println!("last [-]: {}", last);
                        }
                        last = c_counter;
                        c_count = 0;
                    } else {
                        c_count = c_count + 1;
                    }

                }
            }
            line_number = line_number + 1;
        }
    }
    println!("part two: {} increments", increment_count);
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}
