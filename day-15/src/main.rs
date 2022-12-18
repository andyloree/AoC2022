use std::{io::{self}};

#[derive(Copy, Clone,PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(point_str: String) ->Self {
        let mut parts = point_str.split(",");
        let x = parts.next().unwrap().parse::<i32>().unwrap();
        let y = parts.next().unwrap().parse::<i32>().unwrap();
        return Point { x: x, y: y };
    }

    fn dist(&self, other: &Point) -> i32 {
        return i32::abs(self.x - other.x) + i32::abs(self.y - other.y);
    }
}

#[derive(Copy, Clone)]
struct Sensor {
    center: Point,
    nearest: Point,
    dist: i32
}

impl  Sensor {
    fn new(line: String) -> Self {
        let mut parser = line.split(": closest beacon is at x=");
        let center = Point::new(parser.next().unwrap().replace("Sensor at x=", "").replace(" y=", ""));
        let nearest = Point::new(parser.next().unwrap().replace(" y=", ""));
        let dist = center.dist(&nearest);
        return Sensor { center: center, nearest: nearest, dist: dist };
    }

    fn is_covered(&self, target: Point) -> bool {
        let target_dist = self.center.dist(&target);
        if self.nearest == target {
            return false;
        }
        else if target_dist <= self.dist {
            return true;
        }
        else {
            return false;
        }
    }

    fn perimeter(&self) -> Vec<Point> {
        let mut points = Vec::new();

        for dx in 0..=self.dist + 1 {
            let dy = self.dist + 1 - dx;
            points.push(Point{x: self.center.x + dx, y: self.center.y + dy});
            points.push(Point{x: self.center.x + dx, y: self.center.y - dy});
            points.push(Point{x: self.center.x - dx, y: self.center.y + dy});
            points.push(Point{x: self.center.x - dx, y: self.center.y - dy});
        }
        return points;
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    let sensors = lines.iter().map(|line| Sensor::new(line.to_string())).collect::<Vec<Sensor>>();

    println!("Part 1\r\n{}", "-".repeat(10));
    let y_target = 2000000;
    // Filter our sensors down to only those with their radius in our y_target
    let mut candidates = sensors.iter().filter(|&sensor| {
        let y_min = sensor.center.y - sensor.dist;
        let y_max = sensor.center.y + sensor.dist;
        return y_min <= y_target && y_max >= y_target;
    }).map(|s| s.clone()).collect::<Vec<Sensor>>();

    let x_min = candidates.iter().map(|sensor| sensor.center.x - sensor.dist).min().unwrap();
    let x_max = candidates.iter().map(|sensor| sensor.center.x + sensor.dist).max().unwrap();

    // Sweep x across each sensor, checking for coverage
    let mut no_beacons = 0;
    for x in x_min..=x_max {
        let target = Point { x: x, y: y_target };
        for sensor in &mut candidates {
            if sensor.is_covered(target) {
                no_beacons = 1 + no_beacons;
                break;
            }
        }
    }
    println!("Cannot contain beacons: {}\r\n", no_beacons);

    println!("Part 2\r\n{}", "-".repeat(10));

    // Take perimeter + 1 of each sensor's circle, we know our point must lie
    // at an edge, so get each point and check the other sensors for collision
    // The one without is our distress becon
    let limit = 4000000;
    'sensors: for sensor in &sensors {
        'perimeter: for candidate in sensor.perimeter() {
            if candidate.x < 0 || candidate.y < 0 || candidate.x > limit || candidate.y > limit {
                continue;
            }
            // Circuling other sensors in a radius, hoping to find our point
            for other in &sensors {
                if candidate.dist(&other.center) <= other.dist {
                    continue 'perimeter;
                }
            }
            // Found it
            println!("Distress frequency: {}", candidate.x as i64 * 4000000 + candidate.y as i64);
            break 'sensors;
        }
    }    
}
