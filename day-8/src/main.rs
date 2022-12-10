use std::{io::{self}, collections::HashMap, vec, num};
use std::cmp;

struct Forest {
    trees: Vec<Vec<i8>>,
    rows: usize,
    columns: usize
}

impl Forest {
    fn from_lines(lines: Vec<String>) -> Forest {
        let char_map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect();
        let mut trees: Vec<Vec<i8>> = char_map.into_iter().map(|col|col.into_iter().map(|c| c.to_string().parse::<i8>().unwrap()).collect::<Vec<i8>>()).collect();

        let columns = lines[0].len();
        let rows = lines.len();
    
        let mut new_forest = Forest{ trees: trees, rows: rows, columns: columns };
        return new_forest;
    }

    fn scenic_scoring(&self) -> u32 {
        let mut score: Vec<Vec<u32>> = vec![vec![0;self.columns];self.rows];
        for r in 0..self.rows {
            for c in 0..self.columns {
                // look east
                let mut num_trees = 0u32;
                for east in c+1..self.columns {
                    num_trees = num_trees + 1;
                    if self.trees[r][c] <= self.trees[r][east] {
                        break;
                    }
                }
                score[r][c] = num_trees;
            }
        }
        for r in 0..self.rows {
            for c in 0..self.columns { // First column will always be one, so skip
                // look west
                let mut num_trees = 0u32;
                for west in (0..c).rev() {
                    num_trees = num_trees + 1;
                    if self.trees[r][c] <= self.trees[r][west] {
                        break;
                    }
                }
                score[r][c] = score[r][c] * num_trees;
            }
        }
        for c in 0..self.columns {
            for r in 0..self.rows {            
                // look down
                let mut num_trees = 0u32;
                for south in r+1..self.rows {
                    num_trees = num_trees + 1;
                    if self.trees[r][c] <= self.trees[south][c] {
                        break;
                    }
                }
                score[r][c] = score[r][c] * num_trees;
            }
        }
        for c in 0..self.columns {
            for r in 0..self.rows {            
                // look up
                let mut num_trees = 0u32;
                for north in (0..r).rev() {
                    num_trees = num_trees + 1;
                    if self.trees[r][c] <= self.trees[north][c] {
                        break;
                    }
                }
                score[r][c] = score[r][c] * num_trees;
            }
        }

        let largest_scenic_score = score.iter().fold(0,|acc: u32, col| {
            let col_max = col.iter().max().unwrap().to_owned();
            cmp::max(acc,col_max)
        });
        return largest_scenic_score;
    }

    fn find_visible(&self) -> usize {
        let mut visible: Vec<Vec<bool>> = vec![vec![false;self.columns-2];self.rows-2];
        // from left
        for r in 1..self.rows-1 {
            let mut max_height: i8 = self.trees[r][0];   // left most tree height
            for c in 1..self.columns-1 {
                // above direction max height and currently invisible
                if !visible[r-1][c-1] && self.trees[r][c] > max_height {
                    visible[r-1][c-1] = true;
                    max_height = self.trees[r][c];
                }
            }
        }
        // from right
        for r in 1..self.rows-1 {
            let mut max_height: i8 = self.trees[r][self.columns-1];   // left most tree height
            for c in (1..self.columns-1).rev() {
                // above direction max height and currently invisible
                if self.trees[r][c] > max_height {
                    visible[r-1][c-1] = true;
                    max_height = self.trees[r][c];
                }
            }
        }
        // from top
        for c in 1..self.columns-1 {
            let mut max_height: i8 = self.trees[0][c];   // top most tree height
            for r in 1..self.rows-1 {
                // above direction max height and currently invisible
                if self.trees[r][c] > max_height {
                    visible[r-1][c-1] = true;
                    max_height = self.trees[r][c];
                }
            }
        }
        // from bottom
        for c in 1..self.columns-1 {
            let mut max_height: i8 = self.trees[self.rows-1][c];   // top most tree height
            for r in (1..self.rows-1).rev() {
                // above direction max height and currently invisible
                if self.trees[r][c] > max_height {
                    visible[r-1][c-1] = true;
                    max_height = self.trees[r][c];
                }
            }
        }

        let num_visible_trees = visible.into_iter().fold(0,|acc,col| {
            acc + col.iter().fold(0, |acc, &v| {
                if v {
                    acc + 1
                }
                else {
                    acc
                }
            })
        }) + (self.rows * 2) + (self.columns - 2) * 2;
        
        return num_visible_trees;
    }
}


fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();
    let mut forest = Forest::from_lines(lines);

    println!("Part 1\r\n{}", "-".repeat(10));
    println!("Number of visible trees: {}\r\n", forest.find_visible());

    println!("Part 2\r\n{}", "-".repeat(10));
    println!("Largest scenic score: {}\r\n", forest.scenic_scoring());

}
