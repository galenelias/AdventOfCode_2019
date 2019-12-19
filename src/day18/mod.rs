use itertools::Itertools;
use std::collections::{VecDeque, HashSet};


fn bfs(grid: &Vec<Vec<char>>, start_pos: &(usize, usize), total_keys: usize) -> usize {
	
	let mut q: VecDeque<((usize, usize), usize, Vec<char>, HashSet<(usize, usize)>)> = VecDeque::new();

	q.push_back((start_pos.clone(), 0, vec![], HashSet::new()));
	while !q.is_empty() {
		let (pos, steps, mut keys, mut visited) = q.pop_front().unwrap();

		if steps % 100 == 0 {
			println!("BFS: {:?}, {}, {:?}, visited.len={}", pos, steps, keys, visited.len());
		}
		let ch = grid[pos.0][pos.1];
	
		visited.insert(pos);
		if ch.is_alphabetic() {
			if ch.is_ascii_lowercase() && !keys.contains(&ch) {
				keys.push(ch);
				if keys.len() == total_keys {
					return steps;
				}
				visited.clear();
			}
			if ch.is_ascii_uppercase() && !keys.contains(&ch.to_ascii_lowercase()) {
				// Hit a door we don't have a key for yet
				continue;
			}
		}

		// Now branch out...
		let mut try_dir = | new_pos: (usize,  usize) | {
			if grid[new_pos.0][new_pos.1] != '#' && !visited.contains(&new_pos) {
				q.push_back((new_pos, steps + 1, keys.clone(), visited.clone()));
			}
		};

		try_dir((pos.0 - 1, pos.1));
		try_dir((pos.0 + 1, pos.1));
		try_dir((pos.0, pos.1 - 1));
		try_dir((pos.0, pos.1 + 1));

	}	

	return 0;
}

pub fn solve(inputs : Vec<String>) {
	let grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	let mut start_row = 0;
	let mut start_col = 0;
	let mut total_keys = 0;

	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			if grid[r][c] == '@' {
				start_row = r;
				start_col = c;
			} else if grid[r][c].is_ascii_lowercase() {
				total_keys += 1;
			}
		}
	}

	let part1 = bfs(&grid, &(start_row, start_col), total_keys);

	println!("Start: {:?}", (start_row, start_col));
	println!("Total keys: {}", total_keys);
	println!("Part 1: {}", part1);

}