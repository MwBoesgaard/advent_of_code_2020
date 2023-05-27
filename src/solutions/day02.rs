use std::fs::read_to_string;

pub fn part_1() -> i32 {
    let contents = read_to_string("src/input/day02.txt").unwrap();
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let counter: i32 = lines.iter()
    .map(|l| Password::from(l.clone()))
    .filter(|p| p.is_valid())
    .count() as i32;

    println!("Day 2 - Part 1: {}", counter);
    return counter;
}

pub fn part_2() -> i32 {
    let contents = read_to_string("src/input/day02.txt").unwrap();
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let counter: i32 = lines.iter()
    .map(|l| Password::from(l.clone()))
    .filter(|p| p.is_valid_part2())
    .count() as i32;

    println!("Day 2 - Part 2: {}", counter);
    return counter;
}


#[derive(Debug)]
struct Password {
    min: u32,
    max: u32,
    key: char,
    code: String
}

impl Password {
    fn from(line: String) -> Password {
        let line_parts: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
        let range: String = line_parts[0].clone();
        let key_part: String = line_parts[1].clone();
        let rest: String = line_parts[2].clone();

        let split_range: Vec<String> = range.split("-").map(|x| x.to_string()).collect();

        return Password {
            min: split_range[0].clone().parse().unwrap(),
            max: split_range[1].clone().parse().unwrap(),
            key: key_part.chars().nth(0).unwrap(),
            code: rest
        }
    }

    fn is_valid(&self) -> bool {
        let count = self.code.chars().filter(|c| c == &self.key).count() as u32;
        if count <= self.max && count >= self.min {
            return true
        }
        return false;
    }

    fn is_valid_part2(&self) -> bool {
        let has_index_min: bool = self.code.chars().nth(self.min as usize - 1).map(|c| c == self.key).unwrap_or(false);
        let has_index_max: bool = self.code.chars().nth(self.max as usize - 1).map(|c| c == self.key).unwrap_or(false);

        return has_index_min ^ has_index_max;
    }
}