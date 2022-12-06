use std::{io::{self}, borrow::Borrow};

fn stack_builder(lines: &Vec<String>) -> (Vec<Vec<char>>, usize) {
     // Find first blank line
    let input_break = lines.iter().position(|line| line.len() == 0).unwrap();

    let num_stacks = lines[input_break-1].split_ascii_whitespace().map(|s| s.parse::<usize>().unwrap()).count();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    for index in (0..input_break-1).rev() {
        let cur_line = lines[index].replace("    ", "[] ").to_string();
        let cur_crates = cur_line.split("]").filter(|c| c.len() > 0 ).map(|c| c.chars().last().unwrap()).filter(|&c| c != ' ').collect::<Vec<_>>();
        for stack_idx in 0..num_stacks {
            if cur_crates[stack_idx] != '[' {
                stacks[stack_idx].push(cur_crates[stack_idx]);
            }
        }
    }
    return (stacks, input_break + 1);
}

fn run_move(stacks: &mut Vec<Vec<char>>, move_command: String, in_order: bool) {
    let re_moved = move_command.replace("move ","").to_string();
    let cmd_parts = re_moved.split(" from ").collect::<Vec<_>>();
    
    let num_to_move = cmd_parts[0].parse::<usize>().unwrap();
    
    let which_stacks = cmd_parts[1].split(" to ").collect::<Vec<_>>();
    
    let from_idx = which_stacks[0].parse::<usize>().unwrap() - 1;
    let to_idx = which_stacks[1].parse::<usize>().unwrap() - 1;

    if in_order {
        // move in order
        let mut hold: Vec<char> = Vec::new();
        for _ in 0..num_to_move {
            hold.push(*(&stacks[from_idx].pop().unwrap()));
        }
        for _ in 0..num_to_move {
            stacks[to_idx].push(hold.pop().unwrap());
        }
    }
    else {
        // one at a time
        for _ in 0..num_to_move {
            let cur_crate = &stacks[from_idx].pop().unwrap();
            stacks[to_idx].push(*cur_crate);
        }    
    }
}

fn peek_top_crates(stacks: Vec<Vec<char>>) -> String {
    let mut tops = String::new();

    for idx in 0..stacks.len() {
        match stacks[idx].last() {
            Some(&cur_crate) => tops.push(cur_crate),
            None => ()
        }
    }
    
    return tops;
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));
    let (mut stacks, cmd_start_at)  = stack_builder(&lines);

    let end = lines.len();
    for cmd_idx in cmd_start_at..end {
        run_move(&mut stacks, lines[cmd_idx].to_string(), false)
    }
    println!("Top crates, one at a time: {}\n", peek_top_crates(stacks));

    println!("Part 2\r\n{}", "-".repeat(10));
    let (mut stacks, cmd_start_at)  = stack_builder(&lines);

    let end = lines.len();
    for cmd_idx in cmd_start_at..end {
        run_move(&mut stacks, lines[cmd_idx].to_string(), true)
    }
    println!("Top crates in order: {}", peek_top_crates(stacks));
}
