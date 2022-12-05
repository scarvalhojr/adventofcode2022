use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct AssignmentPair {
    elf1: RangeInclusive<i32>,
    elf2: RangeInclusive<i32>,
}

impl AssignmentPair {
    fn partially_overlaps(&self) -> bool {
        self.elf1.contains(self.elf2.start())
            || self.elf1.contains(self.elf2.end())
            || self.elf2.contains(self.elf1.start())
            || self.elf2.contains(self.elf1.end())
    }

    fn fully_overlaps(&self) -> bool {
        (self.elf1.contains(self.elf2.start())
            && self.elf1.contains(self.elf2.end()))
            || (self.elf2.contains(self.elf1.start())
                && self.elf2.contains(self.elf1.end()))
    }
}

pub fn part1(input: &[AssignmentPair]) -> usize {
    input.iter().filter(|pair| pair.fully_overlaps()).count()
}

pub fn part2(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|pair| pair.partially_overlaps())
        .count()
}

impl FromStr for AssignmentPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_int = |s: &str| -> Result<i32, Self::Err> {
            s.parse()
                .map_err(|err| format!("Invalid number '{s}' in range: {err}"))
        };

        let parse_range = |s: &str| -> Result<RangeInclusive<i32>, Self::Err> {
            if let Some((start, end)) = s.split_once('-') {
                Ok(RangeInclusive::new(parse_int(start)?, parse_int(end)?))
            } else {
                Err(format!("Invalid range: {s}"))
            }
        };

        if let Some((range1, range2)) = s.split_once(',') {
            Ok(Self {
                elf1: parse_range(range1)?,
                elf2: parse_range(range2)?,
            })
        } else {
            Err(format!("Invalid assignment pair: {s}"))
        }
    }
}
