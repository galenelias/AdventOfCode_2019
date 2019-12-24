use itertools::Itertools;
use std::collections::{HashSet, HashMap};

type Grid = [[char; 5]; 5];
const EMPTY_GRID: Grid = [['.'; 5]; 5];

fn adjacent_bugs1(grid: &Grid, row: usize, col: usize) -> usize {
	let mut result = 0;
	for &(dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
		if row as i32 + dr >= 0 && row as i32 + dr < 5 && col as i32 + dc >= 0 && col as i32 + dc < 5 {
			if (dr != 0 || dc != 0) && grid[(row as i32 + dr) as usize][(col as i32 + dc) as usize] == '#' {
				result += 1;
			}
		}
	}
	return result;
}

fn step1(grid: Grid) -> Grid {
	let mut new_grid = grid.clone();
	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			let ch = grid[r][c];
			if ch == '#' {
				if adjacent_bugs1(&grid, r, c) != 1 {
					new_grid[r][c] = '.';
				}
			} else if ch == '.' {
				let adj = adjacent_bugs1(&grid, r, c);
				if adj == 1 || adj == 2 {
					new_grid[r][c] = '#';
				}
			}
		}
	}

	return new_grid;
}


fn adjacent_bugs2(grids: &HashMap<i32, Grid>, level: &i32, row: i32, col: i32) -> usize {
	let mut result = 0;
	let grid = grids.get(level).unwrap_or(&EMPTY_GRID);
	for &(dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
		if dr == 0 && dc == 0 {
			continue;
		} else if row + dr == -1 {
			result += if grids.get(&(level-1)).unwrap_or(&EMPTY_GRID)[1][2] == '#' { 1 } else { 0 };
		} else if row + dr == 5 {
			result += if grids.get(&(level-1)).unwrap_or(&EMPTY_GRID)[3][2] == '#' { 1 } else { 0 };
		} else if col + dc == -1 {
			result += if grids.get(&(level-1)).unwrap_or(&EMPTY_GRID)[2][1] == '#' { 1 } else { 0 };
		} else if col + dc == 5 {
			result += if grids.get(&(level-1)).unwrap_or(&EMPTY_GRID)[2][3] == '#' { 1 } else { 0 };
		} else if row + dr == 2 && col + dc == 2 {
			// Hit inner level
			let (inner_rows, inner_cols) = if row == 1 {
				(0..1, 0..5) // coming in from top
			} else if row == 3 {
				(4..5, 0..5) // coming in from bottom
			} else if col == 1 {
				(0..5, 0..1) // coming in from left
			} else if col == 3 {
				(0..5, 4..5)
			} else {
				unreachable!()
			};
			let inner_grid = grids.get(&(level + 1)).unwrap_or(&EMPTY_GRID);
			for r in inner_rows {
				for c in inner_cols.clone() {
					result += if inner_grid[r][c] == '#' { 1 } else { 0 };
				}
			}
		} else {
			result += if grid[(row + dr) as usize][(col + dc) as usize] == '#' { 1 } else { 0 };
		}
	}
	return result;
}

fn step_grids(grids: HashMap<i32, Grid>) -> HashMap<i32, Grid> {
	let mut result = HashMap::new();

	let min_level = grids.keys().min().unwrap();
	let max_level = grids.keys().max().unwrap();

	for level in (min_level-1)..=(max_level+1) {
		let prev_grid = grids.get(&level).unwrap_or(&EMPTY_GRID);
		let mut grid = prev_grid.clone();
		for r in 0..grid.len() {
			for c in 0..grid[r].len() {
				if r == 2 && c == 2 {
					continue;
				}
				let ch = grid[r][c];
				if ch == '#' && adjacent_bugs2(&grids, &level, r as i32, c as i32) != 1 {
					grid[r][c] = '.';
				} else if ch == '.' {
					let adj = adjacent_bugs2(&grids, &level, r as i32, c as i32);
					if adj == 1 || adj == 2 {
						grid[r][c] = '#';
					}
				}
			}
		}
		
		// Don't save the level if it's empty
		if grid.iter().any(|row| row.iter().any(|ch| ch == &'#')) {
			result.insert(level, grid);
		}
	}

	return result;
}


fn print_grid(grid: &Grid) {
	for row in grid {
		println!("{}", row.iter().collect::<String>());
	}
}

fn print_grids(grids: &HashMap<i32, Grid>) {
	let mut keys = grids.keys().collect_vec();
	keys.sort();
	for level in keys {
		println!("Level {}:", level);
		for row in grids.get(level).unwrap() {
			println!("{}", row.iter().collect::<String>());
		}

	}
}

fn part1(mut grid: Grid) {
	let mut seen: HashSet<Grid> = HashSet::new();
	for i in 0.. {
		grid = step1(grid);
		if !seen.insert(grid.clone()) {
			println!("Found loop after {} iterations", i);
			print_grid(&grid);

			let mut part1 = 0;
			for r in 0..5 {
				for c in 0..5 {
					if grid[r][c] == '#' {
						let bit = r * 5 + c;
						part1 += 1 << bit;
					}
				}
			}
			println!("Part 1: {}", part1);
			break;
		}
	}
}

fn part2(grid: Grid) {
	let mut grids: HashMap<i32, Grid> = HashMap::new();
	grids.insert(0, grid);

	for _ in 0..200 {
		grids = step_grids(grids);
	}
	print_grids(&grids);

	let total_bugs = grids.values().map(|grid| grid.iter().map(|row| row.iter().filter(|&ch| ch == &'#').count()).sum::<usize>()).sum::<usize>();
	println!("Part 2: {}", total_bugs);
}

pub fn solve(inputs : Vec<String>) {
	let input_grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	let mut grid = [['.'; 5]; 5];
	for r in 0..5 {
		for c in 0..5 {
			grid[r][c] = input_grid[r][c];
		}
	}

	part1(grid.clone());
	part2(grid.clone());
}