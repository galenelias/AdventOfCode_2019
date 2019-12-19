use itertools::Itertools;
use std::collections::{VecDeque, HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;

// Find distance to each remaining key
fn bfs_to_keys(grid: &Vec<Vec<char>>, start_pos: &(usize, usize), current_keys: &[char]) -> HashMap<char, usize> {
	let mut result = HashMap::new();
	let mut q: VecDeque<((usize, usize), usize)> = VecDeque::new();
	let mut visited = HashSet::new();

	q.push_back((start_pos.clone(), 0));
	while !q.is_empty() {
		let (pos, steps) = q.pop_front().unwrap();

		if visited.contains(&pos) {
			continue;
		}

		let ch = grid[pos.0][pos.1];
	
		visited.insert(pos);
		if ch.is_alphabetic() {
			if ch.is_ascii_uppercase() && !current_keys.contains(&ch.to_ascii_lowercase()) {
				// Hit a door we don't have a key for yet
				continue;
			} else if ch.is_ascii_lowercase() && !current_keys.contains(&ch) && !result.contains_key(&ch) {
				result.insert(ch, steps);
				continue;
			}
		}

		// Now branch out...
		let mut try_dir = | new_pos: (usize,  usize) | {
			if grid[new_pos.0][new_pos.1] != '#' && !visited.contains(&new_pos) {
				q.push_back((new_pos, steps + 1));
			}
		};

		try_dir((pos.0 - 1, pos.1));
		try_dir((pos.0 + 1, pos.1));
		try_dir((pos.0, pos.1 - 1));
		try_dir((pos.0, pos.1 + 1));
	}	

	return result;
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct BfsNode {
	steps: usize,
	pos: (usize, usize),
	keys: Vec<char>,
}

impl Ord for BfsNode {
	fn cmp(&self, other: &Self) -> Ordering {
		self.steps.cmp(&other.steps).reverse()
	}
}

impl PartialOrd for BfsNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}
fn keys_to_num(keys: &[char]) -> u64 {
	let mut result = 0;
	for ch in keys {
		let offset = *ch as u32 - 'a' as u32;
		result |= 1 << offset;
	}
	result
}

fn meta_bfs(grid: &Vec<Vec<char>>, start_pos: &(usize, usize), key_positions: &HashMap<char, (usize, usize)>) -> usize {
	let mut visited = HashSet::new();
	let mut heap = BinaryHeap::new();

	heap.push(BfsNode{steps: 0, pos: start_pos.clone(), keys: vec![]});
	while !heap.is_empty() {
		let node = heap.pop().unwrap();

		// println!("{:?} in {}", node.keys, node.steps);

		let key_hash = keys_to_num(&node.keys);
		if !visited.insert((key_hash, node.pos)) {
			continue;
		}

		// Terminating condition
		if node.keys.len() == key_positions.len() {
			return node.steps;
		}

		let reachable_keys = bfs_to_keys(&grid, &node.pos, &node.keys);
		for (key, key_steps) in reachable_keys {
			let mut new_keys = node.keys.clone();
			new_keys.push(key);
			let key_pos = key_positions.get(&key).unwrap();
			heap.push(
				BfsNode {
					steps: node.steps + key_steps,
					pos: *key_pos,
					keys: new_keys,
				}
			);
		}
	}
	unreachable!("Didn't find solution!");
}

pub fn solve(inputs : Vec<String>) {
	let grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	let mut start_pos = (0, 0);
	let mut key_positions = HashMap::new();

	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			if grid[r][c] == '@' {
				start_pos = (r, c);
			} else if grid[r][c].is_ascii_lowercase() {
				key_positions.insert(grid[r][c], (r, c));
			}
		}
	}

	let part1 = meta_bfs(&grid, &start_pos, &key_positions);
	println!("Part 1: {}", part1);
}