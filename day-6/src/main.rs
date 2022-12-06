use std::io::{self};

fn get_start_of_packet(datastream: String, window_size: usize) -> Option<usize> {

    let chars = datastream.chars().collect::<Vec<_>>();

    let windows = chars.windows(window_size).enumerate();

    let mut start_of_packet: usize = 0;
    'window: for (stream_position, window) in windows {
        let mut has_dup = false;
        'unique_check: for idx in 0..window_size - 1 {
            for check_idx in idx + 1..window_size {
                if window[idx] == window[check_idx] {
                    has_dup = true;
                    break 'unique_check;
                }
            }
        }
        if !has_dup {
            // We have a unique character set window, so add window to position for start of message
            return Some(stream_position + window_size);
        }
    }
    return None;
}

fn main() {
    let stdin = io::stdin();
    //let signal = stdin.lines().nth(0).unwrap().unwrap().chars().collect::<Vec<_>>();
    let datastream = stdin.lines().nth(0).unwrap().unwrap();

    println!("Part 1\r\n{}", "-".repeat(10));
    let start_of_packet = get_start_of_packet(datastream.to_string(), 4);
    println!("Number of characters before start of packet: {:?}\n", start_of_packet.unwrap());


    println!("Part 2\r\n{}", "-".repeat(10));
    let start_of_packet = get_start_of_packet(datastream.to_string(), 14);
    println!("Number of characters before start of packet: {:?}\n", start_of_packet.unwrap());
}
