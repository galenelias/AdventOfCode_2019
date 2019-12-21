use itertools::Itertools;
use std::collections::{VecDeque, HashSet, HashMap};

fn bfs(grid: &Vec<Vec<char>>, start_pos: &(usize, usize), end_pos: &(usize, usize), portal_pairs: &HashMap<(usize, usize), (usize, usize)>) -> usize {
	let mut q: VecDeque<((usize, usize), usize)> = VecDeque::new();
	let mut visited = HashSet::new();

	q.push_back((start_pos.clone(), 0));
	while !q.is_empty() {
		let (pos, steps) = q.pop_front().unwrap();

		if !visited.insert(pos.clone()) {
			continue;
		}

		if &pos == end_pos {
			return steps;
		}

		let ch = grid[pos.0][pos.1];
	
		let mut try_dir = | new_pos: (usize,  usize) | {
			if grid[new_pos.0][new_pos.1] == '.' && !visited.contains(&new_pos) {
				q.push_back((new_pos, steps + 1));
			}
		};

		try_dir((pos.0 - 1, pos.1));
		try_dir((pos.0 + 1, pos.1));
		try_dir((pos.0, pos.1 - 1));
		try_dir((pos.0, pos.1 + 1));

		if let Some(portal_out) = portal_pairs.get(&pos) {
			q.push_back((portal_out.clone(), steps + 1));
		}
	}	

	unreachable!("Didn't find exit");
}

pub fn solve(inputs : Vec<String>) {
	let grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	let mut portals: HashMap<[char; 2], Vec<(usize, usize)>> = HashMap::new(); // ['C','D'] => Vec<(usize, usize) //[Opening1, Opening2]

	let width = grid[0].len();
	let height = grid.len();
	for r in 0..height {
		for c in 0..width {
			if grid[r][c].is_ascii_uppercase() {
				if c < width - 1 && grid[r][c+1].is_ascii_uppercase() { // Horizontally aligned portal
					let portal_name = [grid[r][c], grid[r][c+1]];
					let portal_opening = if c > 0 && grid[r][c-1] == '.' { (r, c-1) } else { (r, c+2) };

					portals.entry(portal_name).or_insert(vec![]).push(portal_opening);
				}
				else if r < height - 1 && grid[r+1][c].is_ascii_uppercase() { // Vertically aligned portal
					let portal_name = [grid[r][c], grid[r+1][c]];
					let portal_opening = if r > 0 && grid[r-1][c] == '.' { (r-1, c) } else { (r+2, c) };

					portals.entry(portal_name).or_insert(vec![]).push(portal_opening);
				}
			}
		}
	}

	let mut portal_pairs: HashMap<(usize, usize), (usize, usize)> = HashMap::new(); // Opening1 => Opening 2
	for (k, v) in &portals {
		if v.len() == 2 {
			portal_pairs.insert(v[0].clone(), v[1].clone());
			portal_pairs.insert(v[1].clone(), v[0].clone());
		}
	}

	let start = portals.get(&['A', 'A']).unwrap()[0];
	let end = portals.get(&['Z', 'Z']).unwrap()[0];

	let part1 = bfs(&grid, &start, &end, &portal_pairs);
	println!("Part 1: {}", part1);
}