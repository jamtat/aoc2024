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

impl Cli {
    pub fn line_reader(&self) -> impl Iterator<Item = String> + '_ {
        let f = File::open(&self.input_file()).unwrap();

        BufReader::new(f).lines().map(|l| l.unwrap())
    }

    pub fn input_string(&self) -> String {
        read_to_string(&self.input_file()).unwrap()
    }

    pub fn input_file(&self) -> std::path::PathBuf {
        if let Some(f) = &self.input {
            f.clone()
        } else {
            let day_name = env::args()
                .last()
                .unwrap()
                .rsplit_once("/")
                .unwrap()
                .1
                .to_owned();
            format!("input/{}.txt", day_name).into()
        }
    }
}

pub fn parse() -> Cli {
    Cli::parse()
}
