use std::{io::{self}, collections::HashMap, vec, borrow::Borrow};


struct FSFile {
    name: String,
    size: usize
}

struct FSDir {
    name: String,
    files: Vec<FSFile>,
    child_dirs: Vec<String>,
    file_size: usize,
    tree_size: usize
}

struct FSParser {
    lines: Vec<String>,
    idx: usize,
    dirs: HashMap<String, FSDir>,
    pwd: Vec<String>
}

impl FSParser {
    fn new(lines: Vec<String>) -> Self {
        return FSParser { lines: lines, idx: 0, dirs: HashMap::new(), pwd: vec![] };
    }

    fn parse(mut self) -> Self {
        self.dirs.insert("/".to_string(), FSDir { name: "".to_string(), files: vec![], child_dirs: vec![], file_size: 0, tree_size: 0 });
        while self.idx < self.lines.len() {
            self.apply_commands();
        };

        // update subdirectory sizes, depth-first
        self.update_tree_size("/");

        self
    }

    fn get_tree_size(&self, path: &str) -> usize {
        let root = self.dirs.get("/").unwrap().tree_size;
        return 0;
    }

    /// Depth-first reconciliation of tree_size for each directory
    fn update_tree_size(&mut self, path: &str) -> usize {
        //let mut cur_dir = self.dirs.get_mut(path).unwrap();
        let mut tree_size = 0;

        for subdir in self.dirs.get_mut(path).unwrap().child_dirs.clone() {
            let child_path = join_path(path, subdir.as_str());
            tree_size = tree_size + self.update_tree_size(child_path.as_str());
        }

        let mut cur_dir = self.dirs.get_mut(path).unwrap();
        cur_dir.tree_size = tree_size + cur_dir.file_size;

        return cur_dir.tree_size;
    }

    fn get_pwd(&mut self) -> String {
        let mut root = String::from("/");
        let s = self.pwd.join("/");
        root.push_str(s.as_str());

        return root;
    }

    /// Add a directory if it does not exist
    fn add_dir(&mut self, name: &str) {
        let pwd = self.get_pwd().to_string();
        let path = join_path(pwd.as_str(), name);
        if !self.dirs.contains_key(&path) {
            self.dirs.insert(path, FSDir { name: name.to_string(), files: vec![], child_dirs: vec![], file_size: 0, tree_size: 0 });
        
            // add child_dir in parent
            self.dirs.get_mut(&pwd).unwrap().child_dirs.push(name.to_string());
        }
    }

    /// Apply commands
    fn apply_commands(&mut self) {
        let cur_line = self.lines[self.idx].to_string();
        if cur_line.to_string().starts_with("$ cd ") {
            let next_dir = cur_line.strip_prefix("$ cd ").unwrap();
            self.cwd(next_dir);
            self.idx = self.idx + 1;
        }
        // dir listing
        if cur_line.to_string().starts_with("$ ls") {
            self.idx = self.idx + 1;
            self.ls_dir()
        }

    }

    fn ls_dir(&mut self) {
        'ls: loop {
            let pwd = self.get_pwd();
            if self.idx >= self.lines.len() || self.lines[self.idx].starts_with("$") {
                return;
            }
            let mut ls_entry = self.lines[self.idx].split(" ");
            match ls_entry.next().unwrap() {
                "dir" => {
                    let dir_name = ls_entry.next().unwrap();
                    self.add_dir(dir_name.to_string().as_str());
                }, 
                file_size_str => {
                    let file_size = file_size_str.parse::<usize>().unwrap();
                    let file_name = ls_entry.next().unwrap();
                    let f = FSFile{ name: file_name.to_string(), size: file_size };
                    let mut cur_dir = self.dirs.get_mut(&pwd).unwrap();
                    cur_dir.files.push(f);
                    cur_dir.file_size = cur_dir.file_size + file_size;
                }
            }
            self.idx = self.idx + 1;
        }
    }

    /// Change working directory
    fn cwd(&mut self, dir: &str) {
        if dir == "/" {
            self.pwd.clear();
        }
        else if dir == ".." {
            if self.pwd.len() > 0 {
                self.pwd.pop();
            }
        }
        else {
            self.pwd.push(dir.to_string());
        }
    }

    /// Get parent path from current path
    fn parent_path(&self, path: &str) -> String {
        if path.len() <= 1 {
            return "/".to_string();
        }
        let parent_slash = path.rfind("/").unwrap();
        let slash = path.len()-parent_slash;
        return path[..slash - 1].to_string();
    }
}

fn join_path(parent: &str, name: &str) -> String {
    let mut p = parent.to_string();
    if !p.ends_with("/") {
        p.push_str("/");
    }
    p.push_str(name);
    return p;
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().flatten().collect();

    println!("Part 1\r\n{}", "-".repeat(10));

    let mut parser = FSParser::new(lines).parse();

    let mut filtered_tree_size = parser.dirs.iter().filter(|(_path, cur_dir)| cur_dir.tree_size <= 100000).fold(0, |acc,(_path, cur_dir)| {
        acc + cur_dir.tree_size
    });
    println!("Total size dirs under 100kB: {}", filtered_tree_size);

    let mut root_tree_size = parser.dirs.iter().filter(|c| c.0.to_string() == "/".to_string()).fold(0, |acc,(_path, cur_dir)| {
        acc + cur_dir.tree_size
    });
  

    println!("Part 2\r\n{}", "-".repeat(10));

    const FREE_SPACE_NEEDED: usize = 30000000;
    const TOTAL_DISK_SPACE: usize = 70000000;
    let free_space_avail = TOTAL_DISK_SPACE - root_tree_size;
    let free_space_needed = FREE_SPACE_NEEDED - free_space_avail;
    println!("Free space available: {}", free_space_avail);
    println!("Additional space needed: {}", free_space_needed);

    let mut candidate_dirs = parser.dirs.iter().filter(|&(path, cur_dir)| cur_dir.tree_size >= free_space_needed).map(|c| {
        return (c.0.to_string(), c.1.tree_size);
    }).collect::<Vec<_>>();
    candidate_dirs.sort_by_key(|c| c.1);
    let smallest_dir_to_delete = candidate_dirs.iter().nth(0).unwrap();

    println!("Delete dir {} which has a total size of {}", smallest_dir_to_delete.0, smallest_dir_to_delete.1);

   
}