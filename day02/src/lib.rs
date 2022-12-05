use std::str::FromStr;
use Hand::*;
use Outcome::*;
use Strategy::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
pub enum Strategy {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Hand {
    fn wins_over(self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn loses_to(self) -> Self {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn play(self, other: Hand) -> Outcome {
        if self == other {
            Draw
        } else if self.wins_over() == other {
            Win
        } else {
            Lose
        }
    }

    fn choose_to(self, goal: Outcome) -> Self {
        match goal {
            Win => self.loses_to(),
            Draw => self,
            Lose => self.wins_over(),
        }
    }

    fn score(self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl Outcome {
    fn score(self) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

pub fn part1(input: &[(Hand, Strategy)]) -> i32 {
    input
        .iter()
        .map(|&(opponent, strategy)| {
            let myhand = match strategy {
                X => Rock,
                Y => Paper,
                Z => Scissors,
            };
            myhand.score() + myhand.play(opponent).score()
        })
        .sum()
}

pub fn part2(input: &[(Hand, Strategy)]) -> i32 {
    input
        .iter()
        .map(|(opponent, strategy)| {
            let goal = match strategy {
                X => Lose,
                Y => Draw,
                Z => Win,
            };
            let myhand = opponent.choose_to(goal);
            myhand.score() + goal.score()
        })
        .sum()
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            _ => Err(format!("Invalid hand '{s}'")),
        }
    }
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(X),
            "Y" => Ok(Y),
            "Z" => Ok(Z),
            _ => Err(format!("Invalid strategy '{s}'")),
        }
    }
}
