use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));
    let part_1_total_score = lines.iter().fold(0, | acc, round | {
        acc + match round.as_ref() {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            &_ => panic!("Help!")
        }
    });
    println!("Total score: {}\n", part_1_total_score);

    println!("Part 2\r\n{}", "-".repeat(10));
    let part_2_total_score = lines.iter().fold(0, | acc, round | {
        acc + match round.as_ref() {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            &_ => panic!("Help!")
        }
    });
    println!("Total score: {}", part_2_total_score);

}
