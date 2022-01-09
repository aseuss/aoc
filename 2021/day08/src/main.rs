use aoc_utils::files::read_lines; 
use std::convert::TryInto;

fn main() {
    let path = "input"; 
    let mut digit_lines = Vec::new();
    let mut complete_lines = Vec::new();
    if let Ok(lines) = read_lines(path) {
        let lines = lines.filter_map(Result::ok);
        for line in lines {
                digit_lines.push(line.split_whitespace().skip(11)
                                 .map(|s| {
                                     let mut shit = s.chars().collect::<Vec<char>>();
                                      shit.sort();shit.into_iter().collect::<String>()}).collect::<Vec<String>>());
                complete_lines.push(line.split_whitespace().filter(|s| *s != "|")
                                 .map(convert_string_to_binary).collect::<Vec<u32>>());
        }
    }
    part_one(&digit_lines);
    println!("\n------------------\n");
    part_two(&mut complete_lines);
}

fn convert_string_to_binary(string : &str) -> u32 {
    let mut converted_char = 0;
    let mut chars = string.chars().collect::<Vec<char>>();
    println!("chars {:?}", chars);
    chars.sort();
    let strings = chars.into_iter().collect::<String>();
    println!("strings {:?}", strings);
    for ch in strings.chars() {
        print!("char: {} ", ch);
        match ch {
            'a' => converted_char = converted_char | 0b01000000,
            'b' => converted_char = converted_char | 0b00100000,
            'c' => converted_char = converted_char | 0b00010000,
            'd' => converted_char = converted_char | 0b00001000,
            'e' => converted_char = converted_char | 0b00000100,
            'f' => converted_char = converted_char | 0b00000010,
            'g' => converted_char = converted_char | 0b00000001,
            _ => (),
        }
    }
    println!("\n{:#08b}", converted_char);
    println!();
    converted_char
}

fn convert_binary_to_string(number : u32) -> String {
    let mut value = Vec::new();
    if number & 0b01000000 > 0 {
        value.push('a');
    }
    if number & 0b00100000 > 0 {
        value.push('b');
    }
    if number & 0b00010000 > 0 {
        value.push('c');
    }
    if number & 0b00001000 > 0 {
        value.push('d');
    }
    if number & 0b00000100 > 0 {
        value.push('e');
    }
    if number & 0b00000010 > 0 {
        value.push('f');
    }
    if number & 0b00000001 > 0 {
        value.push('g');
    }
    value.into_iter().collect::<String>()
}

fn part_one(digit_lines : &Vec<Vec<String>>) {
    println!("\nPart one\n");
    let mut count = 0;
    for digit_line in digit_lines {
        println!("{:?}", digit_line);
        count = count + digit_line.into_iter().filter(|d|
            match d.len() {
                2 => true,
                3 => true,
                4 => true,
                7 => true,
                _ => false,
            }).count();
    }
    println!("\nnumber of 1, 4, 7 and 8 counted: {}", count);
}

/*
 *       abcdefg
 * 0  0b01110111
 * 1  0b00010010
 * 2  0b01011101
 * 3  0b01011011
 * 4  0b00111010
 * 5  0b01101011
 * 6  0b01101111
 * 7  0b01010010
 * 8  0b01111111
 * 9  0b01111011
 */
fn part_two(lines : &mut Vec<Vec<u32>>) {
    println!("\nPart two\n");

    let mut overall_sum = 0;
   
    //* index is misued as map entry, values 0..9.
    for pattern in lines {
        let mut map = vec![0; 10];
        let output_values = pattern.split_off(10);
        pattern.sort();

        let it = pattern.iter();
        print!("pattern: [ ");
        for i in it {
            print!("{}, ", convert_binary_to_string(*i));
        }
        println!("]");
        let it = output_values.iter();
        print!("output values: [ ");
        for i in it {
            print!("{}, ", convert_binary_to_string(*i));
        }
        println!("]");

        // convert known segments
        let it = pattern.iter();
        for item in it {
            match count_bits(*item) {
                2 => map[1] = *item,
                3 => map[7] = *item,
                4 => map[4] = *item,
                7 => map[8] = *item,
                _ => (),
            }
        }
        // remove known segments from data set
        pattern.retain(|i| *i != map[1] && *i != map[7] && *i != map[4] && *i != map[8]);

        // next figure out segments for 9
        let it = pattern.iter();
        for i in it {
            if count_bits(*i) == 6 && count_bits(map[4] | *i) == 6 {
                map[9] = *i
            }
        }
        pattern.retain(|i| *i != map[9]);

        // next figure out segments for 0
        let it = pattern.iter();
        for i in it {
            if count_bits(*i) == 6 && count_bits(map[1] | *i) == 6 {
                map[0] = *i
            }
        }
        pattern.retain(|i| *i != map[0]);

        // now we can figure out 3, 5 and 6
        let it = pattern.iter();
        for i in it {
            if count_bits(*i) == 6 {
                map[6] = *i
            }
            if count_bits(*i) == 5 && count_bits(map[1] | *i) == 5 {
                map[3] = *i
            }
            if count_bits(*i) == 5 && (map[1] | *i) == map[9] {
                map[5] = *i
            }
        }
        pattern.retain(|i| *i != map[6] && *i != map[3] && *i != map[5]);

        // left over has to be 2!
        map[2] = pattern[0];
        pattern.retain(|i| *i != map[2]);

        // let's print the mapping
        let mut it = map.iter();
        let mut index = 0;
        while let Some(value) = it.next() {
            print!("[{}]: {}, ", index, convert_binary_to_string(*value));
            index = index + 1;
        }
        println!();

        // time for counting
        let it = output_values.iter();
        let mut factor = 1000;
        let mut sum = 0;
        // convert single digits into number for summing up
        for output in it {
            let index : i32 = map.iter().position(|o| o == output).unwrap().try_into().unwrap();
            sum = sum + index * factor;
            if factor > 0 {
                factor = factor / 10;
            }
        }
        overall_sum = overall_sum + sum;
        println!("output value: {}", sum);

        println!();
    }
    println!("overall sum: {}", overall_sum);
}

fn count_bits(mut number : u32) -> u32 {
    let mut bit_count = 0;
    let bit_mask = 0b1;
    for _i in 0..7 {
        bit_count = bit_count + (number & bit_mask);
        number =  number >> 1;
    }
    bit_count
}
