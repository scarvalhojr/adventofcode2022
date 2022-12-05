use clap::{crate_description, Parser};
use day04::{part1, part2, AssignmentPair};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Puzzle input file
    input_file: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    println!(crate_description!());

    let input = match read_input(&args.input_file) {
        Ok(data) => data,
        Err(err) => {
            println!("Failed to read input: {}", err);
            exit(1);
        }
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<AssignmentPair>, String> {
    let input_file = File::open(filename).map_err(|err| err.to_string())?;

    BufReader::new(input_file)
        .lines()
        .zip(1..)
        .map(|(line, line_num)| {
            line.map_err(|err| (line_num, err.to_string()))
                .and_then(|value| value.parse().map_err(|err| (line_num, err)))
        })
        .collect::<Result<_, _>>()
        .map_err(|(line_num, err)| format!("Line {}: {}", line_num, err))
}
