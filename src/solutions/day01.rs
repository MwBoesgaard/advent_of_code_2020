use std::fs::read_to_string;

pub fn part_1() -> i32 {
    let contents = read_to_string("src/input/day01.txt").unwrap();
    let lines: Vec<i32> = contents.lines().map(|x| x.parse().unwrap()).collect();

    let mut result: i32 = 0;

    for num1 in &lines {
        for num2 in &lines {
            if num1 + num2 == 2020 {
                result =  num1 * num2;
            }
        }
    }

    println!("Day 1 - Part 1: {}", result);
    return result;
}

pub fn part_2() -> i32 {
    let contents = read_to_string("src/input/day01.txt").unwrap();
    let lines: Vec<i32> = contents.lines().map(|x| x.parse().unwrap()).collect();

    let mut result: i32 = 0;

    for num1 in &lines {
        for num2 in &lines {
            for num3 in &lines {
                if num1 + num2 + num3 == 2020 {
                    result =  num1 * num2 * num3;
                }
            }
        }
    }
    println!("Day 1 - Part 2: {}", result);
    return result;
}