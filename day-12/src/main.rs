use std::{io::{self}, collections::BinaryHeap, vec};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct MinVertex {
    idx: usize,
    dist: usize
}

impl Ord for MinVertex {
    fn cmp(&self, other: &MinVertex) -> Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for MinVertex {
    fn partial_cmp(&self, other: &MinVertex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Coordinate {
    row: i32,
    col: i32
}

struct Map {
    elevations: Vec<char>,
    rows: usize,
    columns: usize,
    start_idx: usize,
    end_idx: usize
}

impl Map {
    fn new(lines: Vec<String>) -> Self {
        let rows = lines.len();
        let columns = lines[0].len();
        let elevations = lines.iter().flat_map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<char>>();
        let start_idx = elevations.iter().enumerate().find(|(_,&c)| c == 'S').map(|(idx, _)| idx).unwrap();
        let end_idx  = elevations.iter().enumerate().find(|(_,&c)| c == 'E').map(|(idx, _)| idx).unwrap();
        return Map { elevations: elevations, rows: rows, columns: columns, start_idx: start_idx, end_idx: end_idx }
    }

    fn to_coordinate(&self, idx: usize) -> Coordinate {
        let row = (idx / self.columns) as i32;
        let col = (idx % self.columns) as i32;
        return Coordinate{ row: row, col: col};
    }


    fn get_elevation(&self, idx: usize) -> char {
        // map S to a and E to z
        return match self.elevations[idx] {
            'S' => 'a',
            'E' => 'z',
             x  =>  x
        };
    }

    fn neighbors(&self, idx: usize) ->  Vec<usize> {
        let shift: [i32;4] = [-(self.columns as i32), -1, 1, self.columns as i32];
        let mut neigh: Vec<usize> = vec![];
        let coord = self.to_coordinate(idx);

        for offset in shift {
            if !((coord.row == 0 && offset < -1) ||
                 (coord.row == self.rows as i32  - 1 && offset  > 1) ||
                 (coord.col == 0 && offset == -1 ) ||
                 (coord.col == self.columns as i32 - 1 && offset == 1)) {
                    // Must be at, below, or one level above our current height to be a "neighbor"
                    if self.get_elevation((idx as i32 + offset) as usize) <= char::from_u32(self.get_elevation(idx) as u32 + 1).unwrap() {
                        neigh.push((idx as i32 + offset) as usize);
                    }
                 }
        }
        return neigh;
    }

    fn dijkstra(&mut self) -> Option<(usize, Vec<usize>)> {
        let mut dist: Vec<usize> = vec![usize::MAX; self.elevations.len()];
        let mut prev: Vec<usize> = vec![usize::MAX; self.elevations.len()];
        let mut pqueue: BinaryHeap<MinVertex> = BinaryHeap::new();

        dist[self.start_idx] = 0;
        pqueue.push(MinVertex { idx: self.start_idx, dist: 0 });


        // Find next lowest cost node (priority queue)
        while let Some( MinVertex {idx, dist: idx_dist}) = pqueue.pop() {
            if idx == self.end_idx {
                // reconstruct path
                let mut path: Vec<usize> = vec![];
                let mut cur_idx = self.end_idx;

                while prev[cur_idx] != usize::MAX {
                    path.push(cur_idx);
                    cur_idx = prev[cur_idx];
                }
                path.push(self.start_idx);

                return Some((idx_dist, path));
            }

            for neigh_idx in self.neighbors(idx) {
                let alt = dist[idx] + self.elevations[neigh_idx] as usize;
                if alt < dist[neigh_idx] {
                    dist[neigh_idx] = alt;
                    prev[neigh_idx] = idx;
                    pqueue.push( MinVertex { idx: neigh_idx, dist: idx_dist + 1 }); // Everything has a one (or its not included)
                }
            }
        }
        return None;
    }

    fn scenic_path(&mut self) -> (usize, usize, Vec<usize>) {
        let mut cur_min: (usize, usize, Vec<usize>) = (usize::MAX, usize::MAX, vec![]);

        // find all a's
        let potential_starts = self.elevations.iter().enumerate().filter(|(_, &c)| c == 'S' || c == 'a' )
                                                                    .map(|(idx, _)| idx).collect::<Vec<_>>();

        let orig_start_idx = self.start_idx;
        for start_idx in potential_starts {
            self.start_idx = start_idx;
            let current = self.dijkstra();
            match current {
                Some((steps, path)) => 
                    // Only track smallest steps found
                    if steps < cur_min.1 {
                        cur_min.0 = start_idx;
                        cur_min.1 = steps;
                        cur_min.2 = path;
                    }
                None => ()
            }
        }
        self.start_idx = orig_start_idx;
        
        return cur_min;
    }

}



fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));
    let mut map = Map::new(lines.clone());
    map.to_coordinate(map.start_idx);
    let results = map.dijkstra();

    println!("Fewest possible steps: {}\r\n", results.unwrap().0);

    println!("Part 2\r\n{}", "-".repeat(10));
    let results = map.scenic_path();
    println!("Fewest possible steps: {}\r\n", results.1);
}
