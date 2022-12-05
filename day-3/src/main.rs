use std::io::{self};

fn main() {

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));
    let part_1_priority_sum = lines.iter().fold(0, | acc, sack | {
        let compartments: Vec<&[u8]> = sack.as_bytes().chunks(sack.len() / 2).collect();
        let mut dup = *(compartments[0].iter().filter(|&item| compartments[1].contains(&item) == true).nth(0).unwrap());
        if dup >= b'a' {
            dup = dup - b'`'; // 1-26
        }
        else { 
            dup = dup - b'&'; // 27-52
        }
        acc + dup as u32
    });
    println!("Priority score sum: {}\n", part_1_priority_sum);

    println!("Part 2\r\n{}", "-".repeat(10));
    let part_2_priority_sum = lines.chunks(3).fold(0, | acc, group| {
        let mut dup = *(group[0].as_bytes().iter().filter(|&a| group[1].as_bytes().contains(&a) && group[2].as_bytes().contains(&a)).nth(0).unwrap());
        if dup >= b'a' {
            dup = dup - b'`'; // 1-26
        }
        else { 
            dup = dup - b'&'; // 27-52
        }
        acc + dup as u32
    });
    println!("Groups priority score sum: {}\n", part_2_priority_sum);
}
