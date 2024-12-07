use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

fn example_path(example: &str) -> std::path::PathBuf {
    std::path::Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("bin/examples/{example}"))
}

pub fn example_lines(example: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(example_path(example)).unwrap())
        .lines()
        .map(|l| l.unwrap())
}

pub fn example_string(example: &str) -> String {
    read_to_string(example_path(example)).unwrap()
}
