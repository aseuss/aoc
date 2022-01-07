use aoc_utils::files::read_lines; 
use std::collections::HashMap;

fn main() {
    let path = "input"; 
    let mut lantern_fishes = Vec::new();
    if let Ok(lines) = read_lines(path) {
        if let Some(line) = lines.filter_map(Result::ok).next() {
            lantern_fishes = line.split(',')
                .map(|c| u8::from_str_radix(c, 10)
                .unwrap()).collect::<Vec<u8>>();
        }

    }
    //println!("{:?}", lantern_fish);
    let mut more_lantern_fishes = lantern_fishes.to_vec();
    part_one(&mut lantern_fishes, 80);
    println!("\n-----------------------------\n");
    part_two(&mut more_lantern_fishes, 256);
}

fn part_one(lantern_fishes : &mut Vec<u8>, number_of_days : usize) {
    println!("Part one\n");
    // stupid brute force simulations approach: waste mem, cpu, time...
    let number_of_fishes = calculate_number_of_fish(lantern_fishes, number_of_days);
    println!("Number of lantern fishes after {} days: {}", number_of_days, number_of_fishes);
}

fn part_two(lantern_fishes : &mut Vec<u8>, number_of_days : i64) {
    println!("Part two\n");

    for i in 0..lantern_fishes.len() {
        lantern_fishes[i] = lantern_fishes[i] + 1; // adapt for algorithm
    }
    let mut lookup = HashMap::new();
    // one more than stated in task to accomodate for algorithm
    let numb_of_days_adapted = number_of_days + 1;
    let mut number_of_fishes : i64 = 0;

    for fish in lantern_fishes {
        match lookup.get(fish) {
            Some(fish_count) => number_of_fishes = number_of_fishes + fish_count,
            None => {
                println!("miss, need to calculate");
                let fish_count = num_of_fish(numb_of_days_adapted,
                                             (*fish).into()) + 1;
                number_of_fishes = number_of_fishes + fish_count;
                lookup.insert(fish, fish_count);
            }
        }
    }

    println!("Number of lantern fishes after {} days: {}", number_of_days, number_of_fishes);
}

const REPRODUCTION_RATE : i64 = 7;
const INITIAL_REPRODUCTION_RATE : i64 = 9;

fn num_of_fish(available_days : i64, first_reproduction : i64) -> i64 {
    let mut available_days = available_days - first_reproduction;
    if available_days <= 0 {
        // can't reproduce in the given time
        return 0;
    }
    // first reproduction on day 0
    let mut number_of_fish = available_days / REPRODUCTION_RATE;
    if number_of_fish > 0 && available_days % REPRODUCTION_RATE > 0 {
        // if the last available day is the day of reproduction,
        // take this into account
        number_of_fish = number_of_fish + 1;
    }
    //println!("mod {}", (available_days % REPRODUCTION_RATE));
    //println!("available days {}, fish {}", available_days, number_of_fish);
    if number_of_fish > 0 {
        let mut number_of_new_fish = number_of_fish;
        while number_of_new_fish > 0 {
            number_of_fish = number_of_fish + num_of_fish(available_days, INITIAL_REPRODUCTION_RATE);
            available_days = available_days - REPRODUCTION_RATE;
            number_of_new_fish = number_of_new_fish - 1;
        }
    } else {
        // account for the current fish
        number_of_fish = number_of_fish + 1;
    }
    number_of_fish
}

fn calculate_number_of_fish(lantern_fishes : &mut Vec<u8>, number_of_days : usize) -> usize {
    for _day in 0..number_of_days {
        let mut new_fish_count : u32 = 0;
        for i in 0..lantern_fishes.len() {
            let timer = lantern_fishes[i];
            if timer == 0 {
                lantern_fishes[i] = 6;
                new_fish_count = new_fish_count + 1;
            } else {
                lantern_fishes[i] = timer - 1;
            }
        }
        //println!("add {} new fishes", new_fish_count);
        for _fish in 0..new_fish_count {
            lantern_fishes.push(8);
        }
    }
    lantern_fishes.len()
}
