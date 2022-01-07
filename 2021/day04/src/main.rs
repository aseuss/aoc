use aoc_utils::files::read_lines; 

fn main() {
    let path = "input"; 
    part_one(path); 
    part_two(path); 
}

fn part_one(path: &str) {
    println!("\nPart one\n");
    if let Ok(lines) = read_lines(path) {
        let mut lines = lines.filter_map(Result::ok);
        let drawn_numbers = lines.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        for i in &drawn_numbers {
            println!("drawing number: {}", i);
        }

        let mut current : Vec<i32> = Vec::new();
        let mut cards : Vec<Vec<i32>> = Vec::new();
        for line in lines {
            if line.is_empty() {
                // the input file needs to end with an empty line for this
                // to work
                if current.len() > 0 {
                    cards.push(current);
                    current = Vec::new();
                }
                continue;
            }
           
            let mut numbers = line.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
            current.append(&mut numbers);
        }
        for card in &cards {
            for item in card {
                print!("{:#02} ", item);
            }
            println!();
        }
        let mut bingo_found = false;
        for drawn_number in drawn_numbers {
            println!("drawing number {}", drawn_number);
            for mut card in &mut cards {
                let position = mark_number(&mut card, drawn_number);
                bingo_found = check_bingo_horizontal(card)
                    || check_bingo_vertical(card);
 
                // just some print stuff...
                let mut count = 0;
                for item in &*card {
                    if count % 5 == 0 {
                        print!(" | ");
                    }
                    count = count + 1;
                    if *item == -1 {
                        print!(" X " );
                    } else {
                        print!("{:#02} ", item);
                    }
                }
                print!("| ");
                if position.is_some() {
                    print!(" found at {}", position.unwrap());
                }

                // did we get a bingo?
                if bingo_found {
                    println!(" BINGO!!");
                    let sum : i32 = card.iter().filter(|&value| value > &0).sum();
                    let winning_number = drawn_number;
                    println!("winning number: {:#2}", winning_number);
                    println!("sum winning card: {}", sum);
                    println!("result: {}", winning_number * sum);
                    break;
                }
                println!();
            }
            if bingo_found {
                break;
            }
        }
    }

}

fn check_bingo_horizontal(card : &Vec<i32>) -> bool {
    for row_index in 0..5 {
        for column_index in 0..5 {
            let index = row_index * 5 + column_index;
            if card[index] >= 0 {
                break;
            }
            // if we did not bail out until here, we have a bingo!
            if index == 4 {
                return true;
            }
        }
    }
    false
}

fn check_bingo_vertical(card : &Vec<i32>) -> bool {
    for column_index in 0..5 {
        for row_index in 0..5 {
            let index = row_index * 5 + column_index;
            if card[index] >= 0 {
                break;
            }
            // if we did not bail out until here, we have a bingo!
            if row_index == 4 {
                return true;
            }
        }
    }
    false
}

fn mark_number(card : &mut Vec<i32>, number : i32) -> Option<usize> {
    let position = card.iter().position(|&value| value == number);
    if position.is_some() {
        let position = position.unwrap();
        //std::mem::replace(&mut card[position], -1);
        card[position] = -1;
        return Option::Some(position);
    }
    Option::None
}

fn part_two(path: &str) {
    println!("\nPart two\n");
    if let Ok(lines) = read_lines(path) {
        let mut lines = lines.filter_map(Result::ok);
        let drawn_numbers = lines.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

        let mut current : Vec<i32> = Vec::new();
        let mut cards : Vec<Vec<i32>> = Vec::new();
        for line in lines {
            if line.is_empty() {
                // the input file needs to end with an empty line for this
                // to work
                if current.len() > 0 {
                    cards.push(current);
                    current = Vec::new();
                }
                continue;
            }
           
            let mut numbers = line.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
            current.append(&mut numbers);
        }
        for drawn_number in drawn_numbers {
            println!("drawing number {}", drawn_number);
            let mut bingo_found = false;
            let mut card_index = 0;
            let cards_len = cards.len();
            let mut card_iter = cards.iter_mut();
            while let Some(mut card) = card_iter.next() {
                let position = mark_number(&mut card, drawn_number);
                bingo_found = check_bingo_horizontal(card)
                    || check_bingo_vertical(card);
 
                // just some print stuff...
                let mut count = 0;
                for item in &*card {
                    if count % 5 == 0 {
                        print!(" | ");
                    }
                    count = count + 1;
                    if *item == -1 {
                        print!(" X " );
                    } else {
                        print!("{:#02} ", item);
                    }
                }
                print!("| ");
                if position.is_some() {
                    print!(" found at {}", position.unwrap());
                }

                // did we get a bingo?
                if bingo_found {
                    print!(" BINGO!! on card {}", card_index);
                    if cards_len > 1 {
                        // Clear card to indicate it can be removed
                        // outside the loop. If there is only one left
                        // we need to keep it calculate the score
                        card.clear();
                    } else {
                        println!();
                        let sum : i32 = card.iter().filter(|&value| value > &0).sum();
                        let winning_number = drawn_number;
                        println!("winning number: {:#2}", winning_number);
                        println!("sum winning card: {}", sum);
                        println!("result: {}", winning_number * sum);
                        break;
                    }
                }
                println!();
                card_index = card_index + 1;
            }
            if cards.len() > 1 {
                // remove all the cards that got a bingo in the last draw
                cards.retain(|card| !card.is_empty());
            } else if bingo_found && cards.len() == 1 {
                // assumption is there will only be one last card in the
                // solution space. If not, algorithm probably won't give
                // the correct answer.
                break;
            }
        }
    }

}
