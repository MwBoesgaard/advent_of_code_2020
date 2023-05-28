use std::{fs::read_to_string, collections::HashSet};

pub fn part_1() -> i32 {
    let contents = read_to_string("src/input/day06.txt").unwrap();
    let groups: Vec<&str> = contents.split("\r\n\r\n").collect();
    
    let mut list_of_sizes: Vec<usize> = vec![];

    for group in groups {
        let list_of_chars: Vec<char> = group.chars().filter(|c| *c != '\n' && *c != '\r').collect();
        let set_of_chars: HashSet<char> = HashSet::from_iter(list_of_chars);
        list_of_sizes.push(set_of_chars.len());
    }

    let result: i32 = list_of_sizes.into_iter().reduce(|acc, x| acc + x).unwrap() as i32;

    println!("Day 6 - Part 1: {}", result);
    return result;
}

pub fn part_2() -> i32 {
    let contents = read_to_string("src/input/day06.txt").unwrap();
    let groups: Vec<Vec<&str>> = contents.split("\r\n\r\n").map(|chunk| chunk.split("\r\n").collect()).collect();
    
    let mut list_of_sizes: Vec<usize> = vec![];

    for group in groups {
        let mut list_of_intra_group_sets: Vec<HashSet<char>> = vec![];
        
        for subgroup in group {
            let list_of_chars: Vec<char> = subgroup.chars().filter(|c| *c != '\n' && *c != '\r').collect();
            let set_of_chars: HashSet<char> = HashSet::from_iter(list_of_chars);
            list_of_intra_group_sets.push(set_of_chars);
        }

        let intersection_of_the_group: HashSet<char> = list_of_intra_group_sets.into_iter().reduce(|a: HashSet<char>, b: HashSet<char>| a.intersection(&b).cloned().collect()).unwrap();

        list_of_sizes.push(intersection_of_the_group.len());
    }

    let result: i32 = list_of_sizes.into_iter().reduce(|acc, x| acc + x).unwrap() as i32;

    println!("Day 6 - Part 2: {}", result);
    return result;
}
