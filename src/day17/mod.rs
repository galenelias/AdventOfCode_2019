use intcode::Cpu;
use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

const ROWS: usize = 49;
const COLS: usize = 57;

type Pos = (usize, usize);

enum CompassDirection {
    North,
    East,
    South,
    West
}

fn build_grid(output: &mut VecDeque<i64>) -> Vec<Vec<char>> {
	let mut grid = vec![vec!['.'; COLS+2]; ROWS+2];

	for r in 1..=ROWS {
		for c in 1..=COLS {
			let ch = output.pop_front().unwrap() as u8 as char;
			grid[r][c] = ch;
		}

		output.pop_front(); //newline
	}

	output.pop_front(); //newline
	return grid;
}

fn print_grid (grid: &Vec<Vec<char>>) {
	for row in grid {
		println!("{}", row.iter().collect::<String>());
	}
}

fn count_scaffolds(grid: &Vec<Vec<char>>) -> usize {
	grid.iter().map(|row| row.iter().filter(|&ch| ch == &'#').count()).sum::<usize>()
}

fn is_intersection(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
	grid[pos.0][pos.1] == '#' && grid[pos.0-1][pos.1] == '#' && grid[pos.0+1][pos.1] == '#' && grid[pos.0][pos.1-1] == '#' && grid[pos.0][pos.1+1] == '#'
}

fn part1(data: Vec<i64>) {
	let mut cpu = Cpu::new(data.clone(), &[]);
	cpu.run();

	let grid = build_grid(&mut cpu.output_buffer);
	let mut alignments = 0;

	for r in 1..grid.len()-1 {
		for c in 1..grid[r].len()-1 {
			if is_intersection(&grid, (r, c)) {
				alignments += (r-1) * (c-1); // We added an extra padding row/column, so need to subtract 1
			}
		}
	}

	println!("Part 1: {}", alignments);
}

fn find_bot(grid: &Vec<Vec<char>>) -> ((usize, usize), CompassDirection) {
	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			let ch = grid[r][c];
			if ch == '<' {
				return ((r, c), CompassDirection::West);
			} else if ch == '>' {
				return ((r, c), CompassDirection::East);
			} else if ch == '^' {
				return ((r, c), CompassDirection::North);
			} else if ch == 'v' {
				return ((r, c), CompassDirection::South);
			}
		}
	}
	unreachable!();
}

fn move_pos(pos: &Pos, dir: &CompassDirection) -> (usize, usize) {
	match dir {
		CompassDirection::North => (pos.0 - 1, pos.1),
		CompassDirection::South => (pos.0 + 1, pos.1),
		CompassDirection::West => (pos.0, pos.1 - 1),
		CompassDirection::East => (pos.0, pos.1 + 1),
	}
}

fn turn_right(dir: &CompassDirection) -> CompassDirection {
	match dir {
		CompassDirection::North => CompassDirection::East,
		CompassDirection::East => CompassDirection::South,
		CompassDirection::South=> CompassDirection::West,
		CompassDirection::West => CompassDirection::North,
	}
}

fn turn_left(dir: &CompassDirection) -> CompassDirection {
	match dir {
		CompassDirection::North => CompassDirection::West,
		CompassDirection::West => CompassDirection::South,
		CompassDirection::South=> CompassDirection::East,
		CompassDirection::East => CompassDirection::North,
	}
}

fn enumerate_paths(grid: &Vec<Vec<char>>, mut pos: Pos, dir: CompassDirection, path_so_far: &str, mut step_count: usize, mut visited: HashSet<Pos>, total_nodes: usize) -> Vec<String> {
	let mut result = vec![];
	visited.insert(pos);

	// Walk straight, branching off recursive calls attempting to turn left/right at every opportunity, and then terminate once we can't walk forward anymore
	loop {
		// Try turning left
		let left_dir = turn_left(&dir);
		let left_pos = move_pos(&pos, &left_dir);
		if grid[left_pos.0][left_pos.1] == '#' && !visited.contains(&left_pos) {
			result.append(&mut enumerate_paths(grid, left_pos, left_dir, format!("{},{},L", path_so_far, step_count).as_ref(), 1, visited.clone(), total_nodes));
		}

		// Try turning right
		let right_dir = turn_right(&dir);
		let right_pos = move_pos(&pos, &right_dir);
		if grid[right_pos.0][right_pos.1] == '#' && !visited.contains(&right_pos) {
			result.append(&mut enumerate_paths(grid, right_pos, right_dir, format!("{},{},R", path_so_far, step_count).as_ref(), 1, visited.clone(), total_nodes));
		}

		pos = move_pos(&pos, &dir);

		// Try to keep moving forward if we can
		// Stop if we run out track, or try to walk over a non-intersection which we've already visited
		if grid[pos.0][pos.1] != '#' || (visited.contains(&pos) && !is_intersection(&grid, pos)) {
			break;
		}

		visited.insert(pos);
		step_count += 1;

		// Walked to the last square! Shave off our initial ',0,' which is at the start of our path due to bootstrapping the algorithm
		if visited.len() == total_nodes + 1 {
			result.push(format!("{},{}", path_so_far[3..].to_string(), step_count));
			break;
		}
	}

	return result;
}

fn chunk_len(chunk: &[&str]) -> usize {
	chunk.iter().map(|s| s.len()).sum::<usize>() + chunk.len() - 1
}

fn validate_compression(chunks: &[&[&str]], path: &[&str]) -> Option<String> {
	let mut pos = 0;
	let mut compressed_string = String::new();

	if !chunks.iter().all(|chunk| chunk_len(chunk) <= 20) {
		return None;
	}
	
	while pos < path.len() {
		let mut found = false;
		for (i, chunk) in chunks.iter().enumerate() {
			if chunk.len() + pos > path.len() {
				continue;
			}

			if &&path[pos..pos+chunk.len()] == chunk {
				found = true;
				pos += chunk.len();
				if !compressed_string.is_empty() {
					compressed_string.push(',');
				}
				compressed_string.push(('A' as u8 + i as u8) as char);

				break;
			}
		}

		if !found {
			return None;
		}
	}

	if compressed_string.len() <= 20 {
		Some(compressed_string)
	} else {
		None
	}
}

fn compress_path(path_str: &str) -> Option<Vec<String>> {
	let path = path_str.split(',').collect_vec();
	let a_start = 0;

	for a_end in a_start+1..path.len() {
		let pat_a = &path[a_start..a_end];

		if chunk_len(pat_a) > 20 {
			break;
		}

		for b_start in a_end..path.len() {
			for b_end in b_start+1..=path.len() {
				let pat_b = &path[b_start..b_end];
				if chunk_len(pat_b) > 20 {
					break;
				}

				for c_start in b_end..path.len() {
					for c_end in c_start+1..=path.len() {

						if let Some(comp_str) = validate_compression(&[&path[a_start..a_end], &path[b_start..b_end], &path[c_start..c_end]], &path) {
							let mut result = Vec::new();
							result.push(comp_str);
							result.push(path[a_start..a_end].join(","));
							result.push(path[b_start..b_end].join(","));
							result.push(path[c_start..c_end].join(","));
							return Some(result);
						}
					}
				}
			}
		}
	}

	return None;
}


fn part2(mut data: Vec<i64>) {
	data[0] = 2;
	let mut cpu = Cpu::new(data, &[]);
	cpu.run();

	let grid = build_grid(&mut cpu.output_buffer);
	cpu.output_buffer.clear();
	let bot_pos = find_bot(&grid);
	let total_scaffolds = count_scaffolds(&grid);
	
	print_grid(&grid);

	let mut paths = enumerate_paths(&grid, bot_pos.0, bot_pos.1, "", 0, HashSet::new(), total_scaffolds);

	println!("Found {} total paths", paths.len());
	paths.sort_by_key(|path| path.len());
	for path in paths {
		println!("Path: {}", path);
		if let Some(result) = compress_path(&path) {
			for res in &result {
				cpu.input_buffer.extend(res.chars().map(|ch| ch as u8 as i64));
				cpu.input_buffer.push_back(10);
			}
			cpu.input_buffer.push_back('n' as u8 as i64);
			cpu.input_buffer.push_back(10);

			cpu.run();

			println!("{}", cpu.output_buffer.iter().map(|&i| i as u8 as char).collect::<String>());
			if cpu.output_buffer.back().unwrap() > &256 {
				println!("Part 2: {}", cpu.output_buffer.back().unwrap());
			}
			break;
		}
	}
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	part1(data.clone());
	part2(data);
}