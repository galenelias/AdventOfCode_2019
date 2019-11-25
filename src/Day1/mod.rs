use std::collections::HashSet;
use itertools::Itertools;

pub fn solve(inputs : Vec<String>) {
	let inputs = inputs.iter().map(|line| line.parse::<i32>().unwrap()).collect_vec();

	let part1 = inputs.iter().sum::<i32>();
	println!("Part 1: {}", part1);
}