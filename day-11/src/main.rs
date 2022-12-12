use std::{io::{self}};

enum OpType {
    OldSquared,
    TimesLiteral,
    AddLiteral
}

struct Monkey {
    items: Vec<u64>,
    operation: OpType,
    literal: Option<u64>,
    divisible: u64,
    number_inspections: usize,
    true_monkey: usize,
    false_monkey: usize,
    worry_divisor: Option<u64>,
    worry_lcm: Option<u64>
}

struct MonkeyTroop {
    monkeys: Vec<Monkey>
}

impl Monkey {
    fn new(lines: &[String], worry_divisor: Option<u64>) -> Self {
        let mut items = (lines[1].split(":").nth(1).unwrap().split(",")).map(|item| item.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        
        // Literal and operation type
        let mut op = OpType::AddLiteral;
        let mut literal: Option<u64> = None;
        if lines[2].ends_with("* old") {
            op = OpType::OldSquared;
        }
        else if lines[2].contains("*") {
            op = OpType::TimesLiteral;
            literal = Some(lines[2].split("* ").nth(1).unwrap().trim().parse::<u64>().unwrap());
        }
        else {
            literal = Some(lines[2].split("+ ").nth(1).unwrap().trim().parse::<u64>().unwrap());
        }

        let divisible = lines[3].split("divisible by ").nth(1).unwrap().trim().parse::<u64>().unwrap();
        let true_monkey = lines[4].split("to monkey ").nth(1).unwrap().trim().parse::<usize>().unwrap();
        let false_monkey = lines[5].split("to monkey ").nth(1).unwrap().trim().parse::<usize>().unwrap();


        let mut monkey = Monkey { items: items, operation: op, literal: literal, divisible: divisible, number_inspections: 0, true_monkey: true_monkey, false_monkey: false_monkey, worry_divisor: worry_divisor, worry_lcm: None};
        return monkey;
    }

    fn run_turn(&mut self) -> Vec<(usize, u64)> {
        let mut to_be_thrown: Vec<(usize, u64)> = vec![];

        for idx in 0..self.items.len() {
            self.number_inspections = self.number_inspections + 1;
            self.items[idx] = match self.operation {
                OpType::AddLiteral => self.items[idx] + self.literal.unwrap(),
                OpType::OldSquared => self.items[idx] * self.items[idx],
                OpType::TimesLiteral => self.items[idx] * self.literal.unwrap()
            };
            if self.worry_divisor.is_some() {
                self.items[idx] = self.items[idx] / self.worry_divisor.unwrap();
            }

            if self.worry_lcm.is_some() {
                self.items[idx] = self.items[idx] % self.worry_lcm.unwrap();
            }
            if self.items[idx] % self.divisible == 0 {
                to_be_thrown.push((self.true_monkey, self.items[idx]));
            }
            else {
                to_be_thrown.push((self.false_monkey, self.items[idx]));
            }
        }
        self.items.clear(); // Everything should have been thrown

        return to_be_thrown;
    }

}

impl MonkeyTroop {
    fn new(lines: Vec<String>, worry_divisor: Option<u64>) -> Self {
        let mut troop = MonkeyTroop { monkeys: vec![] };

        for config in lines.chunks(7).collect::<Vec<_>>() {
            troop.monkeys.push(Monkey::new(config, worry_divisor));
        }

        // find lcm of all monkey divisors and set as worry level to avoid overflow math
        if worry_divisor.is_none() {
            let worry_lcm = lcm(troop.monkeys.iter().map(|m| m.divisible).collect::<Vec<u64>>());
            troop.set_worry_lcm(worry_lcm);
        }

        return troop;
    }

    fn set_worry_lcm(&mut self, worry_lcm: u64) {
        self.monkeys.iter_mut().for_each(|monkey| monkey.worry_lcm = Some(worry_lcm));
    }

    fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            let thrown = self.monkeys[idx].run_turn();
            for (monkey_idx, worry_level) in thrown {
                self.monkeys[monkey_idx].items.push(worry_level);
            }
        }
    }
    
    fn monkey_business_level(&self) -> usize {
        let mut inspections = self.monkeys.iter().map(|monkey| monkey.number_inspections).collect::<Vec<usize>>();
        inspections.sort_by(|a, b| b.cmp(a));   // Descending
        inspections.truncate(2);
        return inspections[0] * inspections[1];
    }

}


struct WorryLevel {
    number: u64,
    divisors: Vec<(u64, u64)>
}

impl WorryLevel {
    fn multiple(&mut self, literal: u64) {
        self.number = self.number * literal;
        for idx in 0..self.divisors.len() {
            self.divisors[idx].1 = self.divisors[idx].1 * literal;
        }
    }
}

/// Least common multiple vec of numbers
fn lcm(numbers: Vec<u64>) -> u64 {
    let mut temp = numbers.clone();
    
    // check all the same
    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        // Find lowest index
        match temp.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).map(|(index, _)| index) {
            Some(idx) => {
                temp[idx] = temp[idx] + numbers[idx];
            },
            None => panic!("Not possible")
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    let mut troop = MonkeyTroop::new(lines.clone(), Some(3) );
    println!("Part 1\r\n{}", "-".repeat(10));
    for _ in 0..20 {
        troop.round();
    }
    println!("Monkey business: {}", troop.monkey_business_level());

    let mut troop = MonkeyTroop::new(lines, None );
    println!("Part 2\r\n{}", "-".repeat(10));
    for _ in 0..10000 {
        troop.round();
    }
    println!("Monkey business: {}", troop.monkey_business_level());
}
