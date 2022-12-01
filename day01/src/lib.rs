use std::collections::BinaryHeap;

pub fn part1(input: &[Vec<i32>]) -> Option<i32> {
    input.iter().map(|calories| calories.iter().sum()).max()
}

pub fn part2(input: &[Vec<i32>]) -> Option<i32> {
    let mut heap = input
        .iter()
        .map(|calories| calories.iter().sum())
        .collect::<BinaryHeap<i32>>();

    Some(heap.pop()? + heap.pop()? + heap.pop()?)
}
