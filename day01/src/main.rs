use clap::Parser;
use day01::{part1, part2};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Puzzle input file
    input_file: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let input = match read_input(&args.input_file) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(1);
        }
    };

    match part1(&input) {
        Some(answer) => println!("Part 1: {}", &answer),
        None => println!("Part 1: Not found"),
    }
    match part2(&input) {
        Some(answer) => println!("Part 2: {}", &answer),
        None => println!("Part 2: Not found"),
    }

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<Vec<i32>>, String> {
    let input_file = File::open(filename).map_err(|err| err.to_string())?;

    let lines = BufReader::new(input_file)
        .lines()
        .zip(1..)
        .map(|(line, line_num)| line.map(|l| (line_num, l)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| err.to_string())?;

    lines
        .as_slice()
        .split(|(_, line)| line.is_empty())
        .map(|block| {
            block
                .iter()
                .map(|(line_num, line)| {
                    line.parse().map_err(|err: ParseIntError| {
                        format!("Line {}: {}", line_num, err)
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}
