use std::{fs::read_to_string, collections::HashSet};

pub fn part_1() -> i32 {
    let contents = read_to_string("src/input/day03.txt").unwrap();
    let lines: Vec<Vec<&str>> = contents.lines().map(|l| l.split("").filter(|i| i != &"").collect()).collect();

    let mut hill = Hill::new(&lines);

    let mut sled = Sled::new(0, 0, 3, 1);

    let number_of_collisions = hill.run_sled(&mut sled);

    println!("Day 3 - Part 1: {}", number_of_collisions);
    return number_of_collisions;
}

pub fn part_2() -> i64 {
    let contents = read_to_string("src/input/day03.txt").unwrap();
    let lines: Vec<Vec<&str>> = contents.lines().map(|l| l.split("").filter(|i| i != &"").collect()).collect();

    let mut hill_1: Hill = Hill::new(&lines);
    let mut sled_1 = Sled::new(0, 0, 1, 1);
    let number_of_collisions_1 = hill_1.run_sled(&mut sled_1);

    let mut hill_2: Hill = Hill::new(&lines);
    let mut sled_2 = Sled::new(0, 0, 3, 1);
    let number_of_collisions_2 = hill_2.run_sled(&mut sled_2);

    let mut hill_3: Hill = Hill::new(&lines);
    let mut sled_3 = Sled::new(0, 0, 5, 1);
    let number_of_collisions_3 = hill_3.run_sled(&mut sled_3);

    let mut hill_4: Hill = Hill::new(&lines);
    let mut sled_4 = Sled::new(0, 0, 7, 1);
    let number_of_collisions_4 = hill_4.run_sled(&mut sled_4);

    let mut hill_5: Hill = Hill::new(&lines);
    let mut sled_5 = Sled::new(0, 0, 1, 2);
    let number_of_collisions_5 = hill_5.run_sled(&mut sled_5);

    let result: i64 = number_of_collisions_1 as i64 * number_of_collisions_2 as i64 * number_of_collisions_3 as i64 * number_of_collisions_4 as i64 * number_of_collisions_5 as i64;

    println!("Day 3 - Part 2: {:?}",  result);
    return result;
}


struct Sled {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

impl Sled {
    fn new(x: i32, y: i32, dx: i32, dy: i32) -> Sled {
        return Sled {
            x,
            y,
            dx,
            dy
        }
    }

    fn mov(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

struct Hill {
    pub height: i32,
    pub width: i32,
    pub current_width: i32,
    pub collisions: u32,
    pub trees: HashSet<(i32, i32)>
}

impl Hill {
    fn new(lines: &Vec<Vec<&str>>) -> Hill {

        let mut tree_set = HashSet::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, loc) in line.iter().enumerate() {
                if loc == &"#" {
                    tree_set.insert((col as i32, row as i32));
                }
            }
        }

        return Self {
            height: lines.len() as i32,
            width: lines.get(0).unwrap().len() as i32,
            current_width: lines.get(0).unwrap().len() as i32,
            collisions: 0,
            trees: tree_set
        }
    }

    fn move_trees(&mut self, dist: i32) {
        let new_trees: HashSet<(i32, i32)> = self.trees.iter().map(|(x, y)| (x + dist, *y)).collect();
        self.trees = new_trees;
    }

    fn run_sled(&mut self, sled: &mut Sled) -> i32 {
        while sled.y < self.height as i32 {
            sled.mov();
    
            if sled.x > self.current_width - 1 {
                self.move_trees(self.width);
                self.current_width += self.width;
            }
    
            if self.trees.contains(&(sled.x , sled.y)) {
                self.collisions += 1;
            }
        }
    
        return  self.collisions as i32;
    }
}