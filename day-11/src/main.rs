use std::{io::{self}};

enum OpType {
    OldSquared,
    TimesLiteral,
    AddLiteral
}

struct Monkey {
    items: Vec<u128>,
    operation: OpType,
    literal: Option<u128>,
    divisible: u128,
    number_inspections: usize,
    true_monkey: usize,
    false_monkey: usize,
    worry_level_control: Option<u128>
}

struct MonkeyTroop {
    monkeys: Vec<Monkey>
}

impl Monkey {
    fn new(lines: &[String], worry_level_control: Option<u128>) -> Self {
        let mut items = (lines[1].split(":").nth(1).unwrap().split(",")).map(|item| item.trim().parse::<u128>().unwrap()).collect::<Vec<u128>>();
        
        // Literal and operation type
        let mut op = OpType::AddLiteral;
        let mut literal: Option<u128> = None;
        if lines[2].ends_with("* old") {
            op = OpType::OldSquared;
        }
        else if lines[2].contains("*") {
            op = OpType::TimesLiteral;
            literal = Some(lines[2].split("* ").nth(1).unwrap().trim().parse::<u128>().unwrap());
        }
        else {
            literal = Some(lines[2].split("+ ").nth(1).unwrap().trim().parse::<u128>().unwrap());
        }

        let divisible = lines[3].split("divisible by ").nth(1).unwrap().trim().parse::<u128>().unwrap();
        let true_monkey = lines[4].split("to monkey ").nth(1).unwrap().trim().parse::<usize>().unwrap();
        let false_monkey = lines[5].split("to monkey ").nth(1).unwrap().trim().parse::<usize>().unwrap();

        let mut monkey = Monkey { items: items, operation: op, literal: literal, divisible: divisible, number_inspections: 0, true_monkey: true_monkey, false_monkey: false_monkey, worry_level_control: worry_level_control};
        return monkey;
    }

    fn run_turn(&mut self) -> Vec<(usize, u128)> {
        let mut to_be_thrown: Vec<(usize, u128)> = vec![];

        for idx in 0..self.items.len() {
            self.number_inspections = self.number_inspections + 1;
            self.items[idx] = match self.operation {
                OpType::AddLiteral => self.items[idx] + self.literal.unwrap(),
                OpType::OldSquared => self.items[idx] * self.items[idx],
                OpType::TimesLiteral => self.items[idx] * self.literal.unwrap()
            };
            if self.worry_level_control.is_some() {
                self.items[idx] = self.items[idx] / self.worry_level_control.unwrap();
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
    fn new(lines: Vec<String>, worry_level_control: Option<u128>) -> Self {
        let mut troop = MonkeyTroop { monkeys: vec![] };

        for config in lines.chunks(7).collect::<Vec<_>>() {
            troop.monkeys.push(Monkey::new(config, worry_level_control));
        }

        return troop;
    }

    fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            let thrown = self.monkeys[idx].run_turn();
            for (monkey_idx, worry_level) in thrown {
                self.monkeys[monkey_idx].items.push(worry_level);
            }
        }

        // for idx in 0..self.monkeys.len() {
        //     println!("Monkey {}: {:?}", idx, self.monkeys[idx].items);
        // }
        // println!();
    }
    
    fn monkey_business_level(&self) -> usize {
        let mut inspections = self.monkeys.iter().map(|monkey| monkey.number_inspections).collect::<Vec<usize>>();
        inspections.sort_by(|a, b| b.cmp(a));   // Descending
        inspections.truncate(2);
        return inspections[0] * inspections[1];
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
    
    // let mut troop = MonkeyTroop::new(lines, Some(3) );
    // println!("Part 2\r\n{}", "-".repeat(10));
    // for idx in 0..1000 {
    //     println!("{}",idx);
    //     troop.round();
    // }
    // println!("Monkey business: {}", troop.monkey_business_level());
}
