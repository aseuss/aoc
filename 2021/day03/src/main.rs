use aoc_utils::files::read_lines; 
 
fn main() { 
    let path = "input"; 
    part_one(path); 
    part_two(path); 
}
const BIT_COUNT : usize = 16;

fn part_one(path: &str) {
    println!("Part one\n");

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut line_count = 0;
    let mut bits: [u32; BIT_COUNT] = [0; BIT_COUNT];
    let mask = 0x1;

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(report_line) = line {
                line_count = line_count + 1;
                let diag_report = u32::from_str_radix(&report_line, 2).unwrap();
                print!("[");
                for i in 0..BIT_COUNT {
                    let shifted_value = diag_report >> i;
                    if shifted_value & mask == 1 {
                        bits[i] = bits[i] + 1;
                    }
                    print!("{} ", bits[i]);
                }
                println!("]");
            }
        }
    }
    println!("line count: {}", line_count);
    let line_count_half = line_count / 2;
    for i in 0..BIT_COUNT {
        if bits[i] == 0 {
            break;
        }
        if bits[i] >= line_count_half {
            gamma_rate = gamma_rate | (0x1 << i);
        } else {
            epsilon_rate = epsilon_rate | (0x1 << i);
        }
    }

    println!("gamma rate: {} epsilon rate: {}", gamma_rate, epsilon_rate);
    println!("Power consumption: {}", gamma_rate * epsilon_rate);
}

fn part_two(path: &str) {
    println!("\nPart two\n");
    if let Ok(lines) = read_lines(path) {
        let mut numbers = lines.filter_map(Result::ok).map(|y| u32::from_str_radix(&y, 2).unwrap()).collect::<Vec<u32>>();
        numbers.sort_unstable();
        let number = numbers.last().unwrap();
        let bit_mask = find_msb_bit_mask(number);
        let numbers_clone = numbers.to_vec();

        println!("Find oxygen genarator rating\n");
        let oxygen_rating = do_stuff(numbers, bit_mask, pick_ones_over_zeroes);
        println!("\nFind CO2 scrubber rating\n");
        let co2_rating = do_stuff(numbers_clone, bit_mask, pick_zeroes_over_ones);
        let life_support_rating = oxygen_rating * co2_rating;

        println!("\nOxygen generator rating: {}", oxygen_rating);
        println!("CO2 scrubber rating:     {}", co2_rating);
        println!("Life support rating:     {}", life_support_rating);

    }
}

fn find_msb_bit_mask(number : &u32) -> u32 {
    let mut bit_mask = u32::MAX ^ (u32::MAX >> 1);
    while bit_mask > 0 {
        if number & bit_mask == 0 {
            bit_mask = bit_mask >> 1;
        } else {
            break;
        }
    }
    bit_mask << 1
}

fn do_stuff(mut numbers : Vec<u32>, bit_mask: u32, evaluator: fn(Vec<u32>,Vec<u32>) -> Vec<u32>) -> u32 {
    println!("numbers len {}", numbers.len());
    if numbers.len() == 1 {
        // time to stop...
        println!("found rating, time to stop");
        return *numbers.first().unwrap();
    }
    let bit_reduction_mask = bit_mask - 1;
    let bit_mask = bit_mask >> 1;
    println!("reduction_mask {:#016b}", bit_reduction_mask);
    numbers.sort_unstable_by(|l, r| {
        let a = l & bit_reduction_mask;
        let b = r & bit_reduction_mask;
        a.cmp(&b)
    });
    let n = numbers.partition_point(|&x| x & bit_reduction_mask < bit_mask);
    for i in &numbers {
        println!(" {:#5} {:#016b}", i, i);
    }
    println!("partition at {} value {} count: {}", n, numbers[n], numbers.len());
    
    let ones = numbers.split_off(n);
    let zeroes = numbers;
    let keeper = evaluator(zeroes, ones);
    
    do_stuff(keeper, bit_mask, evaluator)
}

fn pick_ones_over_zeroes(zeroes : Vec<u32>, ones : Vec<u32>) -> Vec<u32> {
    if zeroes.len() > ones.len() {
        println!(" more zeroes -> continue with zeroes");
        zeroes
    } else if zeroes.len() < ones.len() {
        println!(" more ones -> continue with ones");
        ones
    } else {
        println!(" equal number of zeroes and ones -> continue with ones");
        ones
    }
}

fn pick_zeroes_over_ones(zeroes : Vec<u32>, ones : Vec<u32>) -> Vec<u32> {
    if zeroes.len() > ones.len() {
        println!(" more zeroes -> continue with ones");
        ones
    } else if zeroes.len() < ones.len() {
        println!(" more ones -> continue with zeroes");
        zeroes
    } else {
        println!(" equal number of zeroes and ones -> continue with zeroes");
        zeroes
    }
}
