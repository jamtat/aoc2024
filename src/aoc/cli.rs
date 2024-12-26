use std::{
    env,
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    pub input: Option<std::path::PathBuf>,
}

fn input_path(filename: &str) -> std::path::PathBuf {
    format!("input/{filename}").into()
}

pub fn input_string(filename: &str) -> String {
    read_to_string(input_path(filename)).unwrap()
}

impl Cli {
    pub fn line_reader(&self) -> impl Iterator<Item = String> + '_ {
        let f = File::open(self.input_file()).unwrap();

        BufReader::new(f).lines().map(|l| l.unwrap())
    }

    pub fn input_string(&self) -> String {
        read_to_string(self.input_file()).unwrap()
    }

    pub fn input_file(&self) -> std::path::PathBuf {
        if let Some(f) = &self.input {
            f.clone()
        } else {
            let day_name = env::args()
                .last()
                .unwrap()
                .rsplit_once('/')
                .unwrap()
                .1
                .to_owned();
            input_path(&format!("{day_name}.txt"))
        }
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
