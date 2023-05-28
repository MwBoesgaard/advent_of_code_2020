use std::fs::read_to_string;
use regex::Regex;

pub fn part_1() -> i32 {
    let contents = read_to_string("src/input/day04.txt").unwrap();
    let potential_passports: Vec<&str> = contents.split("\r\n\r\n").collect();

    let result: i32 = potential_passports.into_iter().filter(|pp| valid_passport(pp)).count() as i32;

    println!("Day 4 - Part 1: {:?}", result);
    return result;
}

pub fn part_2() -> i32 {
    let contents = read_to_string("src/input/day04.txt").unwrap();
    let potential_passports: Vec<&str> = contents.split("\r\n\r\n").collect();

    let mut valid_passport_counter = 0;

    for passport in potential_passports {
        if !valid_passport(passport) {
            continue;
        }

        if !valid_digit_range(passport, "byr".to_string(), 1920, 2002) {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            continue;
        }

        if !valid_digit_range(passport, "iyr".to_string(), 2010, 2020) {
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            continue;
        }

        if !valid_digit_range(passport, "eyr".to_string(), 2020, 2030) {
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            continue;
        }

        if !valid_height(passport) {
            // hgt (Height) - a number followed by either cm or in:
            // if cm, the number must be at least 150 and at most 193.
            // if in, the number must be at least 59 and at most 76.
            continue;
        }

        if !valid_hair(passport) {
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            continue;
        }

        if !valid_eyes(passport) {
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            continue;
        }

        if !valid_id(passport) {
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            continue;
        }

        valid_passport_counter += 1;
    }
    println!("Day 4 - Part 2: {:?}", valid_passport_counter);
    return valid_passport_counter;
}

pub fn valid_id(potential_passport: &str) -> bool {
    let re = Regex::new(r"pid:[0-9]{9}\b").unwrap();
    return match re.captures(potential_passport) {
        Some(_) => return true,
        None => false
    }
}

pub fn valid_eyes(potential_passport: &str) -> bool {
    let re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    return match re.captures(potential_passport) {
        Some(_) => return true,
        None => false
    }
}

pub fn valid_hair(potential_passport: &str) -> bool {
    let re = Regex::new(r"hcl:#[0-9a-f]{6}").unwrap();
    return match re.captures(potential_passport) {
        Some(_) => return true,
        None => false
    }
}

pub fn valid_height(potential_passport: &str) -> bool {
    let re = Regex::new(r"hgt:([0-9]{2,3})(cm|in)").unwrap();
    return match re.captures(potential_passport) {
        Some(m) => {
            let num: u32 = m.get(1).unwrap().as_str().parse().unwrap();
            let unit: String = m.get(2).unwrap().as_str().parse().unwrap();

            if unit == "cm" {
                return num >= 150 && num <= 193
            } else if unit ==  "in" {
                return num >= 59 && num <= 76
            } else {
                return false;
            }
        }
        None => false
    }
}

pub fn valid_digit_range(potential_passport: &str, tag: String, min: u32, max: u32) -> bool {
    let re = Regex::new(format!("{tag}:([0-9]{{4}})").as_str()).unwrap();
    return match re.captures(potential_passport) {
        Some(m) => {
            let num: u32 = m.get(1).unwrap().as_str().parse().unwrap();
            return num >= min && num <= max
        }
        None => false
    }
}

pub fn valid_passport(potential_passport: &str) -> bool {
    return potential_passport.contains("byr")
    && potential_passport.contains("iyr")
    && potential_passport.contains("eyr")
    && potential_passport.contains("hgt")
    && potential_passport.contains("hcl")
    && potential_passport.contains("ecl")
    && potential_passport.contains("pid");
}