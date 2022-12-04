use std::io::{self};

struct Elf {
    food_item_calories: Vec<u32>,
    total_calories: u32
}

impl Elf {
    fn add_item(&mut self, calories: u32) {
        self.food_item_calories.push(calories);
        self.total_calories += calories;
    }

    fn from_stdin(stdin: &std::io::Stdin) -> Option<Elf> {
        let mut line: String = String::new();
        let mut new_elf = Elf { food_item_calories: Vec::new(), total_calories: 0};
        
        loop {
            match stdin.read_line(&mut line) {
                Err(_e) => {
                    panic!("Throw the switch Vern, she's pumping mud");
                },
                Ok(num_bytes) => {
                    if num_bytes == 0 || line == "\n" {
                        if new_elf.total_calories == 0 {
                            return None;
                        }
                        else {
                            return Some(new_elf);
                        }
                    }
                    new_elf.add_item(line.trim().parse().expect("Invalid number format"));
                    line.clear();
                }
            }
            line.clear();
        }
    }
}

fn main() {
    println!("Part 1\r\n{}", "-".repeat(10));

    let stdin = io::stdin();
    let mut elves: Vec<Elf> = vec!();

    while let Some(elf) = Elf::from_stdin(&stdin) {
        elves.push(elf);
    }

    // Sort by totals
    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    println!("Elf with most calories: {}", elves.iter().nth(0).unwrap().total_calories);

    println!("Top 3 Elves by total_calories: {}", elves[..3].iter().fold(0, |acc, val| acc + val.total_calories));

}
