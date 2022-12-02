use clap::Parser;
use day02::{part1, part2, Hand, Strategy};
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

fn read_input(filename: &str) -> Result<Vec<(Hand, Strategy)>, String> {
    let input_file = File::open(filename).map_err(|err| err.to_string())?;

    BufReader::new(input_file)
        .lines()
        .zip(1..)
        .flat_map(|(line, line_num)| {
            line.map_err(|err| (line_num, err.to_string()))
                .map(|value| {
                    let mut entries = value.split(' ').collect::<Vec<_>>();
                    let entry2 = entries.pop().map(|e| e.parse());
                    let entry1 = entries.pop().map(|e| e.parse());
                    match (entry1, entry2, entries.pop()) {
                        (Some(Ok(hand)), Some(Ok(strategy)), None) => {
                            Ok((hand, strategy))
                        }
                        _ => Err(format!("Invalid input in line {line_num}")),
                    }
                })
        })
        .collect::<Result<Vec<_>, _>>()
}
