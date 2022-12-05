use std::io::{self};

fn assignments_from_line(line: &String) -> ((u32,u32),(u32,u32)) {
    let mut assignments = line.split(",");
    let mut left = assignments.next().unwrap().split("-");
    let mut right = assignments.next().unwrap().split("-");
    
    return ((left.next().unwrap().parse::<u32>().unwrap(),left.next().unwrap().parse::<u32>().unwrap()),(right.next().unwrap().parse::<u32>().unwrap(),right.next().unwrap().parse::<u32>().unwrap()));
}

fn has_overlap(assignment: ((u32,u32), (u32,u32))) -> bool {
    let ((a,b),(c,d)) = assignment;

    if (a >= c && a <= d || b <= d && b >= c) ||
    (c >= a && c <= b || d <= b && d >= a) {
        return true;
    }
    else
    {
        return false;
    }
}

fn has_full_overlap(assignment: ((u32,u32), (u32,u32))) -> bool {
    let ((a,b),(c,d)) = assignment;

    if (a >= c && a <= d && b <= d && b >= c) ||
       (c >= a && c <= b && d <= b && d >= a) {
        return true;
    }
    else
    {
        return false;
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));
    let part_1_fully_contained = lines.iter().filter(|line| has_full_overlap(assignments_from_line(&line))).count();
    println!("Number fully contained: {}\n", part_1_fully_contained);

    println!("Part 2\r\n{}", "-".repeat(10));
    let part_1_fully_contained = lines.iter().filter(|line| has_overlap(assignments_from_line(&line))).count();
    println!("Number overlap: {}\n", part_1_fully_contained);
}
