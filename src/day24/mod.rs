use itertools::Itertools;
use std::collections::HashSet;

fn adjacent_bugs(grid: &[[char; 5]; 5], row: usize, col: usize) -> usize {
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

fn step(grid: [[char; 5]; 5]) -> [[char; 5]; 5] {
	let mut new_grid = grid.clone();
	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			let ch = grid[r][c];
			if ch == '#' {
				if adjacent_bugs(&grid, r, c) != 1 {
					new_grid[r][c] = '.';
				}
			} else if ch == '.' {
				let adj = adjacent_bugs(&grid, r, c);
				if adj == 1 || adj == 2 {
					new_grid[r][c] = '#';
				}
			}
		}
	}

	return new_grid;
}

fn print_grid(grid: &[[char; 5]; 5]) {
	for row in grid {
		println!("{}", row.iter().collect::<String>());
	}
}

pub fn solve(inputs : Vec<String>) {
	let input_grid = inputs.iter().map(|line| line.chars().collect_vec()).collect_vec();

	let mut grid = [['.'; 5]; 5];
	for r in 0..5 {
		for c in 0..5 {
			grid[r][c] = input_grid[r][c];
		}
	}

	print_grid(&grid);

	let mut seen: HashSet<[[char; 5]; 5]> = HashSet::new();
	for i in 0.. {
		grid = step(grid);
		if !seen.insert(grid.clone()) {
			println!("Found loop after {}", i);
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