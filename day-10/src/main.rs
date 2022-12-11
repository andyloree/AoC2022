use std::{io::{self}};

struct CommDevice {
    cycle: usize,
    x: i32,
    signals: Vec<i32>,
    sprite: i32,
    crt: Vec<char>
}

impl CommDevice {
    fn new() -> Self {
        return CommDevice { cycle: 0, x: 1, signals: vec![], sprite: 0, crt: vec!['.'; 40 * 6] };
    }

    fn execute_instruction(&mut self, line: String) {
        let mut parse = line.split(" ");
        let cmd = parse.next().unwrap();

        match cmd {
            "noop" => self.noop(),
            "addx" => {
                let literal = parse.next().unwrap().parse::<i32>().unwrap();
                self.addx(literal);
            },
            _ => panic!("Throw the switch Vern!")
        }
    }

    fn draw_pixel(&mut self) {
        self.sprite = self.x - 1;
        let pixel = self.cycle as i32 % 40;
        let buffer_pos = self.cycle % (40 * 6);
        if self.sprite <= pixel && self.sprite + 2 >= pixel {
            self.crt[buffer_pos] = '#';
        }
        else {
            self.crt[buffer_pos] = '.';
        }
    }

    fn tick(&mut self) {
        self.cycle = self.cycle + 1;
        // Log signal on every 40th cycle offset +20
        if (self.cycle as i32 - 20) % 40 == 0 {
            println!("{} x {} = {}", self.cycle, self.x, self.x * (self.cycle as i32));
            self.signals.push(self.x * (self.cycle as i32));
        }
    }

    fn addx(&mut self, literal: i32) {
        self.draw_pixel();
        self.tick();
        self.draw_pixel();
        self.tick();
        self.x = self.x + literal;
    }

    fn noop(&mut self) {
        self.draw_pixel();
        self.tick();
    }

    fn render(&self) {
        self.crt.chunks(40).for_each(|pixels| {
            println!("{}", pixels.iter().cloned().collect::<String>());
        })
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    let mut comm = CommDevice::new();
    
    println!("Part 1\r\n{}", "-".repeat(10));
    for line in lines.iter() {
        comm.execute_instruction(line.to_string());
    }
    let signal_sum: i32 = comm.signals.iter().sum();
    println!("Signal sum: {}\r\n", signal_sum);

    println!("Part 2\r\n{}", "-".repeat(10));
    comm.render();
}
