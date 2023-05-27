use std::fs::read_to_string;

pub fn part_1() -> i32 {
    let contents = read_to_string("src/input/day01.txt").unwrap();
    let lines: Vec<i32> = contents.lines().map(|x| x.parse().unwrap()).collect();
    
    // Find the two numbers that, when added together, sum to 2020.
    // Return their product.

    for num1 in &lines {
        for num2 in &lines {
            if num1 + num2 == 2020 {
                let solution =  num1 * num2;
                println!("The product is: {}", solution);
                return solution;
            }
        }
    }
    return 0;
}

pub fn part_2() -> i32 {
    let contents = read_to_string("src/input/day01.txt").unwrap();
    let lines: Vec<i32> = contents.lines().map(|x| x.parse().unwrap()).collect();
    
    // Find the three numbers that, when added together, sum to 2020.
    // Return their product.

    for num1 in &lines {
        for num2 in &lines {
            for num3 in &lines {
                if num1 + num2 + num3 == 2020 {
                    let solution =  num1 * num2 * num3;
                    println!("The product is: {}", solution);
                    return solution;
                }
            }
        }
    }
    return 0;
}