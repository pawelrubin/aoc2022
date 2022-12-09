use std::{collections::HashMap, hash::Hash};

mod shared;

enum Entry {
    LS,
    CD(String),
    File(u32, String),
    Directory(String),
}

impl Entry {
    fn parse(line: String) -> Self {
        if line.starts_with("$ ls") {
            return Self::LS;
        }
        if line.starts_with("$ cd") {
            let (_, name) = line.split_at(4);
            return Self::CD(String::from(name));
        }
        if line.starts_with("dir") {
            let (_, name) = line.split_once(" ").unwrap();
            return Self::Directory(String::from(name));
        }
        let (size, name) = line.split_once(" ").unwrap();
        Self::File(size.parse().unwrap(), String::from(name))
    }
}

struct FsItem {
    size: u32, // 0 for directories
    name: String,
    content: HashMap<String, FsItem>, // TODO: make it a mutable reference?
}

struct State {
    cwd: String,
}

fn parse_dir(iter: impl IntoIterator<Item = Entry>) {}

fn main() {
    let mut file_system = FsItem {
        size: 0,
        name: String::from("/"),
        content: vec![],
    };

    let mut state = shared::get_lines().map(Entry::parse);
    // .for_each(|entry| {
    //     match entry {
    //     Entry::LS => cwd,
    //     Entry::Directory(name) => cwd,
    //     }
    // })
}
