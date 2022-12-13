use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    iter::Map,
};

type Lines = Map<std::io::Lines<BufReader<File>>, fn(Result<String, std::io::Error>) -> String>;

pub fn get_lines() -> Lines {
    let input_file_path = env::args().nth(1).expect("no input file path given");

    BufReader::new(File::open(input_file_path).unwrap())
        .lines()
        .map(Result::unwrap)
}

pub fn get_contents() -> String {
    let input_file_path = env::args().nth(1).expect("no input file path given");

    fs::read_to_string(input_file_path).unwrap()
}
