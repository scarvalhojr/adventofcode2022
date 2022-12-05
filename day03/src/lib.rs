use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct Rucksack {
    compart1: HashSet<char>,
    compart2: HashSet<char>,
}

impl Rucksack {
    fn common_type(&self) -> Option<char> {
        self.compart1.intersection(&self.compart2).copied().next()
    }

    fn all_types(&self) -> HashSet<char> {
        self.compart1.union(&self.compart2).copied().collect()
    }

    fn common_types(&self, other_types: &HashSet<char>) -> HashSet<char> {
        self.all_types()
            .intersection(other_types)
            .copied()
            .collect()
    }
}

fn item_score(item: char) -> Option<u32> {
    if item.is_ascii_lowercase() {
        (item as u32 + 1).checked_sub('a' as u32)
    } else if item.is_ascii_uppercase() {
        (item as u32 + 27).checked_sub('A' as u32)
    } else {
        None
    }
}

pub fn part1(input: &[Rucksack]) -> Option<u32> {
    input
        .iter()
        .map(|rucksack| rucksack.common_type().and_then(item_score))
        .collect::<Option<Vec<_>>>()
        .map(|scores| scores.iter().sum())
}

pub fn part2(input: &[Rucksack]) -> Option<u32> {
    let mut sum = 0;

    for group in input.chunks(3) {
        let mut common = group[0]
            .common_types(&group[1].common_types(&group[2].all_types()));

        if common.len() != 1 {
            return None;
        }

        sum += common.drain().next().and_then(item_score)?;
    }

    Some(sum)
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (items1, items2) = s.split_at(s.len() / 2);
        let compart1 = items1.chars().collect();
        let compart2 = items2.chars().collect();
        Ok(Self { compart1, compart2 })
    }
}
