use aoc_utils::files::read_lines; 
use std::convert::TryInto;

fn main() {
    let path = "input"; 
    let mut crab_sub_positions = Vec::new();
    if let Ok(lines) = read_lines(path) {
        if let Some(line) = lines.filter_map(Result::ok).next() {
            crab_sub_positions = line.split(',')
                .map(|c| i32::from_str_radix(c, 10)
                .unwrap()).collect::<Vec<i32>>();
        }

    }
    crab_sub_positions.sort();
    print_crab_distribution(&crab_sub_positions);

    println!("\npart one");
    calculate_minimum_fuel_consumption(&crab_sub_positions, calculate_cost);

    println!("\n-------------------------\n");
    println!("part two");
    calculate_minimum_fuel_consumption(&crab_sub_positions, calculate_cost_crab_way);
    println!();
}

fn calculate_minimum_fuel_consumption(crab_sub_positions : &[i32], cost_function : fn(&[i32], i32) -> i32) {
    let mut lowest_cost = i32::MAX;
    for i in 0..crab_sub_positions.len().try_into().unwrap() {
        let cost = cost_function(&crab_sub_positions, i);
        if cost < lowest_cost {
            lowest_cost = cost;
        } else if lowest_cost < cost {
            // we found our lowest cost
            println!("lowest fuel consumption at [{}] {}", i-1, lowest_cost);
            break;
        }
    }
}

fn print_crab_distribution(crab_sub_positions : &Vec<i32>) {
    let mut last_pos = -1;
    for crab_sub_position in crab_sub_positions {
        if last_pos != *crab_sub_position {
            println!();
            print!("pos: {:4} |", crab_sub_position);
            last_pos = *crab_sub_position;
        } else {
            print!("|");
        }
    }
    println!();
}

fn calculate_cost(data : &[i32], index : i32) -> i32 {
    data.iter().map(|pos| move_sub(*pos, index)).sum()
}

fn calculate_cost_crab_way(data : &[i32], index : i32) -> i32 {
    data.iter().map(|pos| move_sub_crab_way(*pos, index)).sum()
}

fn move_sub(current_pos : i32, target_pos : i32) -> i32 {
    if current_pos <= target_pos {
        target_pos - current_pos
    } else {
        current_pos - target_pos
    }
}

fn move_sub_crab_way(current_pos : i32, target_pos : i32) -> i32 {
    let mut distance = move_sub(current_pos, target_pos);
    let mut cost = 0;
    while distance > 0 {
        cost = cost + distance;
        distance = distance - 1;
    }
    cost
}
