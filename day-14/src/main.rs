use std::{io::{self}};

struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(str: String) -> Self {
        let mut parser = str.split(",");
        let x = parser.next().unwrap().parse::<usize>().unwrap();
        let y = parser.next().unwrap().parse::<usize>().unwrap();
        return Point { x: x, y: y };
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();
    let mut space: Vec<Vec<u8>> = vec![vec![0; 1000]; 1000];
    // Add our segmant walls (8)
    let mut bottom: usize = 0;
    lines.iter().for_each(|line| {
        let raw_coord = line.split(" -> ").map(|s| Point::new(s.to_string())).collect::<Vec<Point>>();
        raw_coord.windows(2).for_each(|pair| {
            bottom = bottom.max(pair[0].y).max(pair[1].y);
            if pair[0].x == pair[1].x {
                // horizontal
                let y_min = usize::min(pair[0].y, pair[1].y);
                let y_max = usize::max(pair[0].y, pair[1].y);
                for y in y_min..=y_max {
                    space[pair[0].x][y] = 8;
                }
            }
            else {
                // vertical
                let x_min = usize::min(pair[0].x, pair[1].x);
                let x_max = usize::max(pair[0].x, pair[1].x);
                for x in x_min..=x_max {
                    space[x][pair[0].y] = 8;
                }
            }
        });
    });

    println!("Part 1\r\n{}", "-".repeat(10));
    let mut counter = 0;
    'outer: loop {
        let mut grain = Point { x: 500, y: 0};
        'inner: loop {
            if grain.y > bottom {
                // terminate
                break 'outer;
            }
            if space[grain.x][grain.y+1] == 0 {
                // We can drop down one
                grain.y = grain.y + 1;
            }
            else if space[grain.x][grain.y+1] > 0 {
                // Check left first
                if space[grain.x-1][grain.y+1] == 0 {
                    grain.x = grain.x - 1;
                    grain.y = grain.y + 1;
                }
                else if space[grain.x+1][grain.y+1] == 0 {
                    grain.x = grain.x + 1;
                    grain.y = grain.y + 1;
                }
                else {
                    // Stay there
                    space[grain.x][grain.y] = 1;
                    counter = counter + 1;
                    break 'inner;
                }
            }
        }
    }
    println!("Grains of sand {}\r\n", counter);

    println!("Part 2\r\n{}", "-".repeat(10));
    // Add a floor and keep going
    for x in 0..1000 {
        space[x][bottom + 2] = 8;
    }
    'outer: loop {
        let mut grain = Point { x: 500, y: 0};
        if space[500][0] == 1 {
            // terminate
            break 'outer;
        }
        'inner: loop {
            if space[grain.x][grain.y+1] == 0 {
                // We can drop down one
                grain.y = grain.y + 1;
            }
            else if space[grain.x][grain.y+1] > 0 {
                // Check left first
                if space[grain.x-1][grain.y+1] == 0 {
                    grain.x = grain.x - 1;
                    grain.y = grain.y + 1;
                }
                else if space[grain.x+1][grain.y+1] == 0 {
                    grain.x = grain.x + 1;
                    grain.y = grain.y + 1;
                }
                else {
                    space[grain.x][grain.y] = 1;
                    counter = counter + 1;
                    break 'inner;
                }
            }
        }
    }
    println!("Grains of sand {}", counter);
}

