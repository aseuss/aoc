use aoc_utils::files::read_lines; 
use std::convert::TryInto;

fn main() {
    let path = "input"; 

    if let Ok(lines) = read_lines(path) {
        let lines = lines.filter_map(Result::ok).map(|s| s.replace(" -> ", ","));
        let mut vent_lines = Vec::new();
        let mut largest_x = 0;
        let mut largest_y = 0;

        for line in lines {
            let bla = line.split(',').map(|c| i32::from_str_radix(c, 10).unwrap()).collect::<Vec<i32>>();
            let from = Point {
                x : bla[0],
                y : bla[1],
            };
            if from.x > largest_x {
                largest_x = from.x;
            }
            if from.y > largest_y {
                largest_y = from.y;
            }
            let to = Point {
                x : bla[2],
                y : bla[3],
            };
            if to.x > largest_x {
                largest_x = to.x;
            }
            if to.y > largest_y {
                largest_y = to.y;
            }
            let vent_line = Line {
                from : from,
                to : to,
            };
            vent_lines.push(vent_line);
        }

        // indexing starts at 0, so 1 needs to be added to largest
        // x and y values
        largest_x = largest_x + 1;
        largest_y = largest_y + 1;

        let map_width = largest_x.try_into().unwrap();

        println!("dimensions [0.0] -> [{},{}]", largest_x, largest_y);
        let mut map = vec![0; (largest_x*largest_y).try_into().unwrap()];
    
        part_one(&mut map, map_width, &vent_lines); 
        print_map(&map, map_width);
        let count = map.iter().filter(|&x| *x > 1).count();
        println!("Result: {}", count);

        println!("\n-----------------------------\n");

        let mut map = vec![0; (largest_x*largest_y).try_into().unwrap()];
        part_two(&mut map, map_width, &vent_lines);
        print_map(&map, map_width);
        let count = map.iter().filter(|&x| *x > 1).count();
        println!("Result: {}", count);
    }
    println!();
}

#[derive(Debug)]
struct Point {
    x : i32,
    y : i32,
}

#[derive(Debug)]
struct Line {
    from : Point,
    to: Point,
}

fn part_one(map : &mut Vec<i32>, map_width : usize, vent_lines : &Vec<Line>) {
    println!("Part one\n");

    for vent_line in vent_lines {
        println!("{:?}", vent_line);
        add_to_map_no_diagonals(map, map_width, &vent_line);
        println!();
    }
}

fn part_two(map : &mut Vec<i32>, map_width : usize, vent_lines : &Vec<Line>) {
    println!("Part two\n");

    for vent_line in vent_lines {
        println!("{:?}", vent_line);
        add_to_map(map, map_width, &vent_line);
        println!();
    }
}

fn print_map(map : &Vec<i32>, width : usize) {
    for i in 0..map.len() {
        if map[i] == 0 {
            print!(".");
        } else {
            print!("{}", map[i]);
        }
        if (i + 1) % width == 0 {
            println!();
        }
    }
    println!();
}

fn add_to_map_no_diagonals(map : &mut Vec<i32>, width : usize, vent_line : &Line) {
    add_horizontally(map, width, &vent_line);
    add_vertically(map, width, &vent_line);
}

fn add_to_map(map : &mut Vec<i32>, width : usize, vent_line : &Line) {
    add_horizontally(map, width, &vent_line);
    add_vertically(map, width, &vent_line);
    add_diagonally(map, width, &vent_line);
}

fn add_horizontally(map : &mut Vec<i32>, width : usize, vent_line : &Line) {
    let from = &vent_line.from;
    let to = &vent_line.to;

    if from.y == to.y {
        let mut x = from.x;
        let y = from.y; 
        if x < to.x {
            while x <= to.x {
                let index = convert_coordinate_to_index(x, y, width);
                map[index] = map[index] + 1;
                x = x + 1;
            }
        } else if x > to.x {
            while x >= to.x {
                let index = convert_coordinate_to_index(x, y, width);
                map[index] = map[index] + 1;
                x = x - 1;
            }
        }
    }
}

fn add_vertically(map : &mut Vec<i32>, width : usize, vent_line : &Line) {
    let from = &vent_line.from;
    let to = &vent_line.to;

    if from.x == to.x {
        let x = from.x;
        let mut y = from.y; 
        if y < to.y {
            while y <= to.y {
                let index = convert_coordinate_to_index(x, y, width);
                map[index] = map[index] + 1;
                y = y + 1;
            }
        } else if y > to.y {
            while y >= to.y {
                let index = convert_coordinate_to_index(x, y, width);
                map[index] = map[index] + 1;
                y = y - 1;
            }
        }
    }
}

fn add_diagonally(map : &mut Vec<i32>, width : usize, vent_line : &Line) {
    let from = &vent_line.from;
    let to = &vent_line.to;

    if from.x != to.x && from.y != to.y {
        // from.x < to.x -> x + 1
        // from.x > to.x -> x - 1
        // from.y < to.y -> y + 1
        // from.y > to.y -> y - 1
        let mut x_inc = 1;
        let mut y_inc = 1;
        if from.y > to.y {
            y_inc = -1;
        }
        if from.x < to.x {
            let mut x = from.x;
            let mut y = from.y; 

            while x <= to.x {
                let index = convert_coordinate_to_index(x, y, width);
                map[index] = map[index] + 1;
                x = x + x_inc;
                y = y + y_inc;
            }
        } else if from.x > to.x {
            x_inc = -1;

            let mut x = from.x;
            let mut y = from.y; 

            while x >= to.x {
                let index = convert_coordinate_to_index(x, y, width);
                map[index] = map[index] + 1;
                x = x + x_inc;
                y = y + y_inc;
            }

        }
    }
}

fn convert_coordinate_to_index(x : i32, y : i32, map_width : usize) -> usize {
    let map_width : i32 = map_width.try_into().unwrap();
    let result = (x + y * map_width).try_into().unwrap();
    println!("index: {} x: {} y: {}", result, x, y);
    result
}
