use std::{io::{self}, vec, cmp::Ordering};

enum Packet {
    Literal(PacketNumber),
    Op(PacketList)
}


struct PacketNumber {
    value: i8
}

struct PacketList {
    items: Vec<Packet>
}

struct PacketParser {
    tokens: Vec<char>,
    pos: usize
}

#[derive(Debug)]
enum CompareResults {
    Correct,
    Incorrect,
    Indetereminate
}

impl PacketNumber {
    fn compare(&mut self, right: &mut Packet) -> CompareResults {
        match right {
            Packet::Op(_) => {
                // Convert left to a list first
                let mut left_list = PacketList { items: vec![]};
                left_list.items.push(Packet::Literal(PacketNumber { value: self.value }));
                return left_list.compare(right);
            }
            Packet::Literal(right_number) => {
                if self.value < right_number.value {
                    return CompareResults::Correct;
                }
                else if self.value > right_number.value {
                    return CompareResults::Incorrect;
                }
                else {
                    return CompareResults::Indetereminate;
                }
            }
        };
    }
}

impl PacketList {
    fn compare(&mut self, right: &mut Packet) -> CompareResults {
        match right {
            Packet::Op(right_list) => {
                // List to list
                loop {
                    // left ran out of items, so inputs are in right order
                    if self.items.len() == 0 && right_list.items.len() > 0 {
                        return CompareResults::Correct;
                    }   // Right ran out of items, so inputs are not in the right order
                    else if self.items.len() > 0 && right_list.items.len() == 0 {
                        return CompareResults::Incorrect;
                    } // Both ran out the same time, tbd
                    else if self.items.len() == 0 && right_list.items.len() == 0 {
                        return CompareResults::Indetereminate;
                    }
                    else {
                        let mut left_item = self.items.remove(0);
                        let mut right_item = right_list.items.remove(0);
                        let results = left_item.compare(&mut right_item);
                        if !matches!(results, CompareResults::Indetereminate) {
                            return results; // Terminate we have an answer
                        }
                    }
                }
            },
            Packet::Literal(right_number) => {
                // Convert right to a list first
                let mut right_list = PacketList { items: vec![]};
                right_list.items.push(Packet::Literal(PacketNumber { value: right_number.value }));
                return self.compare(&mut Packet::Op(right_list));
            }
        };
    }
}

impl Packet{ 
    fn compare(&mut self, right: &mut Packet) -> CompareResults {
        let results = match self {
            Packet::Op(list) => list.compare(right),
            Packet::Literal(number) => number.compare(right)
        };
        return results;
    }

    fn traverse(self, depth: usize) -> (usize, Vec<i8>) {
        return match self {
            Packet::Literal(number) => (depth, vec![number.value]),
            Packet::Op(list) => {
                let mut values: Vec<i8> = vec![];
                let mut max_depth = depth;
                if list.items.len() == 0 {
                    (depth, vec![-1])   // Empty lists sorted before zero's
                }
                else {
                    for item in list.items {
                        let mut child = item.traverse(depth + 1);
                        max_depth = usize::max(max_depth, child.0);
                        values.append(&mut child.1);
                    }
                    (max_depth, values)
                }
            }
       };
    }
}

impl PacketParser {
    fn new(line: String) -> Packet {
        let mut parser = PacketParser { tokens: line.chars().collect::<Vec<char>>(), pos: 0 };
        return Packet::Op( parser._parse() );
    }
    
    fn _parse(&mut self) -> PacketList {
        let mut list: PacketList = PacketList { items: vec![] };
        assert!(self.tokens[self.pos] == '[');
        self.pos = self.pos + 1;

        while self.pos < self.tokens.len() {
            if self.tokens[self.pos].is_ascii_digit() {
                // chomp our number
                let mut len: usize = 1;
                while self.tokens[self.pos + len].is_ascii_digit() {
                    len = len + 1;
                }
                let number_str = self.tokens[self.pos..self.pos+len].iter().collect::<String>();
                list.items.push(Packet::Literal(PacketNumber { value: number_str.parse::<i8>().unwrap() }));
                self.pos = self.pos + len;
            }
            else if self.tokens[self.pos] == ',' {
                self.pos = self.pos + 1;
            }
            else if self.tokens[self.pos] == ']' {
                self.pos = self.pos + 1;  // We are done
                break;
            }
            else if self.tokens[self.pos] == '[' {
                let child = self._parse();
                list.items.push(Packet::Op(child));
            }
        }
        return list;
    }
}

struct PacketPair {
    index: usize,
    left: Option<Packet>,
    right: Option<Packet>,
    is_correct: bool
}

impl PacketPair {
    fn new(index: usize, lines: &[String]) -> PacketPair {
        assert!(lines.len() >= 2);
        let mut pp = PacketPair { index: index, left: Some(PacketParser::new(lines[0].clone())), right: Some(PacketParser::new(lines[1].clone())), is_correct: false };
        pp.is_correct = pp._correct_order();
        return pp
    }

    fn _correct_order(&mut self) -> bool {
        let mut left = Option::take(&mut self.left).unwrap();
        let mut right = Option::take(&mut self.right).unwrap();
        let results = left.compare(&mut right);
        return match results  {
            CompareResults::Correct => true,
            _ => false
        };
    }
}

struct PacketSorter {
    decoder_key: usize
}

impl PacketSorter {
    fn new(lines: Vec<String>) -> PacketSorter {
        let mut packets = lines.iter().filter(|line| line.len() > 0).map(|line| PacketParser::new(line.clone())).collect::<Vec<Packet>>();
        // add our two divider packets
        packets.push(PacketSorter::_divider(2));
        packets.push(PacketSorter::_divider(6));

        let mut flattend: Vec<(usize, Vec<i8>)> = Vec::new();
        for packet in packets  {
            flattend.push(packet.traverse(0));
        }

        flattend.sort_by(|a, b| {
            return match a.1.cmp(&b.1) {
                Ordering::Equal => a.0.cmp(&b.0),
                results => results
            };
        });

       
        // Get our divider indices
        let div_2 = flattend.iter().position(|d| {
            if d.0 == 2 && d.1.len() == 1 && d.1[0] == 2 {
                return true;
            }
            return false;
        }).unwrap() + 1;

        let div_6 = flattend.iter().position(|d| {
            if d.0 == 2 && d.1.len() == 1 && d.1[0] == 6 {
                return true;
            }
            return false;
        }).unwrap() + 1;
        return PacketSorter { decoder_key: div_2 * div_6 };
    }
    
    fn _divider(value: i8) -> Packet {
        let num = Packet::Literal(PacketNumber { value: value });
        let inner = Packet::Op( PacketList { items: vec![num] });
        let outer = Packet::Op( PacketList { items: vec![inner] });
        return outer;
    }
}


fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));
    let mut pairings = lines.chunks(3).enumerate().map(|(index, pairs)| PacketPair::new(index + 1, pairs)).collect::<Vec<PacketPair>>();
    let indicies_sum = pairings.iter_mut().filter(|pairing| pairing.is_correct ).map(|pairing| pairing.index).sum::<usize>();
    println!("Sum of correct indices: {}\r\n", indicies_sum);

    println!("Part 2\r\n{}", "-".repeat(10));
    let sorter = PacketSorter::new(lines);
    println!("Decoder key: {}\r\n", sorter.decoder_key);
}