use aoc_utils::files::read_lines;

fn main() {
    let path = "input";
    part_one(path);
    part_two(path);
}

fn part_one(path: &str) {
    println!("Part one\n");
    let mut position = 0;
    let mut depth = 0;

    println!("pos {} depth {}", position, depth);

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(command) = line {
                let mut bla = command.split_whitespace();
                match bla.next() {
                    Some("forward") => position = position + bla.next().unwrap().parse::<u32>().unwrap(),
                    Some("up") => depth = depth - bla.next().unwrap().parse::<u32>().unwrap(),
                    Some("down") => depth = depth + bla.next().unwrap().parse::<u32>().unwrap(),
                    other => println!("invalid command: {}", other.unwrap()),
                }
                println!("pos {} depth {}", position, depth);
            }
        }
    }
    println!("Result: {}", position * depth);
}

fn part_two(path: &str) {
    println!("\nPart two\n");
    let mut position = 0;
    let mut depth = 0;
    let mut aim : i32 = 0;

    println!("pos {} depth {}", position, depth);

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(command_line) = line {
                let mut command_tokens = command_line.split_whitespace();
                let command = command_tokens.next().unwrap();
                let value = command_tokens.next().unwrap().parse::<i32>().unwrap();
                match command {
                    "forward" => {
                        position = position + value;
                        depth = depth + aim * value;
                    },
                    "up" => aim = aim - value,
                    "down" => aim = aim + value,
                    other => println!("invalid command: {}", other),
                }
                println!("pos {} depth {}", position, depth);
            }
        }
    }
    println!("Result: {}", position * depth);

}
