use std::{fs::read_to_string, collections::HashSet};

pub fn part_1() -> i32 {
    let contents = read_to_string("src/input/day05.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut seat_ids: Vec<u32> = vec![];

    for binary_info in lines {
        let (row_info, column_info) = binary_info.split_at(7);
        let row = recursive_binary_to_row(row_info.chars().collect(), 0, 127, 'F', ('F', 'B'));
        let column = recursive_binary_to_row(column_info.chars().collect(), 0, 7, 'L', ('L', 'R'));
        seat_ids.push(row * 8 + column);
    }
    let result = *seat_ids.iter().max().unwrap() as i32;
    println!("Day 5 - Part 1: {:?}", result);
    return result;
}

pub fn part_2() -> i32 {
    let contents = read_to_string("src/input/day05.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut seat_row_col: Vec<(u32, u32)> = vec![];

    for binary_info in lines {
        let (row_info, column_info) = binary_info.split_at(7);
        let row = recursive_binary_to_row(row_info.chars().collect(), 0, 127, 'F', ('F', 'B'));
        let column = recursive_binary_to_row(column_info.chars().collect(), 0, 7, 'L', ('L', 'R'));
        seat_row_col.push((row, column));
    }
    let seats_taken: HashSet<(u32, u32)> = HashSet::from_iter(seat_row_col);
    let mut seats_all: HashSet<(u32, u32)> = HashSet::new();
    for row in 0..127 {
        for col in 0..7 {
            seats_all.insert((row, col));
        }
    }

    let my_seat: (u32, u32) = *seats_all.difference(&seats_taken).filter(|(row, _)| *row < 110 && *row > 10).next().unwrap();
    let (my_row, my_col) = my_seat;
    let result = (my_row * 8 + my_col) as i32; 

    println!("Day 5 - Part 2: {:?}", result);
    return result;
} 

fn recursive_binary_to_row(binary: Vec<char>, low: u32, high: u32, last_symbol: char, low_high_symbols: (char, char)) -> u32 {
    let (down, up) = low_high_symbols;
    let mid_float: f64 = ((high - low) as f64 / 2.0) + low as f64;
    
    return match binary.split_first() {
        Some((first, rest)) => {
            let mid = if *first == down { mid_float.floor() } else { mid_float.ceil() } as u32;

            match first {
                _ if *first == down => recursive_binary_to_row(rest.to_vec(), low, mid, down, low_high_symbols),
                _ if *first == up => recursive_binary_to_row(rest.to_vec(), mid, high, up, low_high_symbols),
                _ => panic!("This is be unpossible.")
            }
        },
        None => if last_symbol == down { low } else { high }
    };
}