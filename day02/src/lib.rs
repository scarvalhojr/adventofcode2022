use std::str::FromStr;
use Hand::*;
use Outcome::*;
use Strategy::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
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
    fn play(self, other: Hand) -> Outcome {
        if self == other {
            Draw
        } else if other as u8 == (self as u8 + 1) % 3 {
            Lose
        } else {
            Win
        }
    }

    fn choose_to(self, goal: Outcome) -> Self {
        if goal == Draw {
            self
        } else if goal == Win {
            Self::try_from((self as u8 + 1) % 3).unwrap()
        } else {
            Self::try_from((self as u8 + 2) % 3).unwrap()
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

impl TryFrom<u8> for Hand {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Rock as u8 => Ok(Rock),
            x if x == Paper as u8 => Ok(Paper),
            x if x == Scissors as u8 => Ok(Scissors),
            _ => Err(()),
        }
    }
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
