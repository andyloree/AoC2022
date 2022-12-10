use std::{io::{self}, collections::{HashMap}};

struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new() -> Self {
        Default::default()
    }
    fn shift(&mut self, delta: &Point) {
        self.x = self.x + delta.x;
        self.y = self.y + delta.y;
    }

    fn to_string(&self) -> String {
        return format!("{},{}", self.x.to_string(), self.y.to_string());
    }

    fn dist(&self, other: &Point) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

impl Default for Point {
    #[inline]
    fn default() -> Self {
        Point {
            x: 0,
            y: 0
        }
    }
}

struct Rope {
    knots: Vec<Point>,
    tracks: HashMap<String,u32>
}

impl Rope {
    fn new(num_knots: usize) -> Rope {
        let mut knots: Vec<Point> = Vec::new();
        (0..num_knots).for_each(|_idx| knots.push(Point::new()));
        return Rope { knots: knots, tracks: HashMap::new()};
    }

    fn move_command(&mut self, line: String) {
        let mut input = line.split(" ");
        let direction = input.next().unwrap();
        let length = input.next().unwrap().parse::<i32>().unwrap();
        let mut delta = Point{x: 0, y: 0};
        match direction {
            "L" => delta.x = -1,
            "R" => delta.x = 1,
            "U" => delta.y  = 1,
            "D" => delta.y = -1,
            _ => panic!("Throw the switch vern!")
        }
        for _idx in 0..length {
            self.knots[0].shift(&delta);    // Shift the head one direction
            // Move tail knots if needed
            for tail_idx in 1..self.knots.len() {
                if !self.move_tail(tail_idx) {
                    break;
                }
            }
        }
    }

    fn move_tail(&mut self, tail_idx: usize) -> bool {
        let head_idx = tail_idx - 1;
        let dist = self.knots[tail_idx].dist(&self.knots[head_idx]);
        let mut delta_x = self.knots[head_idx].x - self.knots[tail_idx].x;
        let mut delta_y = self.knots[head_idx].y - self.knots[tail_idx].y;
        let mut moved = false;

        // Same row or column
        if  (self.knots[tail_idx].x == self.knots[head_idx].x ||
            self.knots[tail_idx].y == self.knots[head_idx].y) &&
            dist > 1 {
                delta_x = delta_x - delta_x.signum();
                delta_y = delta_y - delta_y.signum();
                self.knots[tail_idx].shift(&Point{x: delta_x, y: delta_y });
                moved = true;
        }
        else if dist > 2 {  // diagnoal move
            if delta_x >= 0 && delta_y >= 0 {       // Upper right
                self.knots[tail_idx].shift(&Point{x: 1, y: 1 });
            }
            else if delta_x >= 0 && delta_y < 0 {   // Lower right
                self.knots[tail_idx].shift(&Point{x: 1, y: -1 });
            }
            else if delta_x < 0 && delta_y >= 0 {   // Upper left
                self.knots[tail_idx].shift(&Point{x: -1, y: 1 });
            }
            else if delta_x < 0 && delta_y < 0  {   // Lower left
                self.knots[tail_idx].shift(&Point{x: -1, y: -1 });
            }
            moved = true;
        }
        
        // Track tail
        if tail_idx == self.knots.len() - 1 {
            self.tracks.entry(self.knots[tail_idx].to_string()).or_insert_with(|| {
                0
            });
        }
        return moved;
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    let mut rope = Rope::new(2);
    println!("Part 1\r\n{}", "-".repeat(10));
    for line in lines.iter() {
        rope.move_command(line.to_string());
    }
    println!("Number of tail tracks: {}\r\n", rope.tracks.keys().len());


    let mut rope = Rope::new(10);
    println!("Part 2\r\n{}", "-".repeat(10));
    for line in lines.iter() {
        rope.move_command(line.to_string());
    }
    println!("Number of tail tracks: {}", rope.tracks.keys().len());


}

