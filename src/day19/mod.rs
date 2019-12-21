use itertools::Itertools;
use intcode::Cpu;

fn is_tractored(data: &Vec<i64>, x: usize, y: usize) -> bool {
	let mut cpu = Cpu::new(data.clone(), &[]);
	cpu.input_buffer.push_back(x as i64);
	cpu.input_buffer.push_back(y as i64);
	cpu.run();
	let result = cpu.output_buffer.pop_front().unwrap();
	return result == 1;
}

fn part1(data: &Vec<i64>) {
	let mut grid = [[' '; 50]; 50];
	for y in 0..50 {
		for x in 0..50 {
			grid[y][x] = if is_tractored(&data, x, y) { '#' } else { '.' };
		}
	}
	println!("Part 1: {}", grid.iter().map(|row| row.iter().filter(|&c| c == &'#').count()).sum::<usize>());
}

fn get_y_for_x(data: &Vec<i64>, x: usize) -> usize {
	for y in x.. {
		if is_tractored(&data, x, y) {
			return y;
		}
	}
	unreachable!();
}

fn get_diag_for_x(data: &Vec<i64>, x: usize) -> usize {
	let y = get_y_for_x(data, x);
	for d in 1.. {
		if !is_tractored(&data, x - d, y + d) {
			return d;
		}
	}
	unreachable!("Unexpected")
}

const TARGET_DIM: usize = 100;

fn part2(data: &Vec<i64>) {
	let mut x = 10;

	// Logarithmic search for some x bounds
	while get_diag_for_x(data, x) < TARGET_DIM {
		x *= 10;
	}

	// Binary search for the exact cross-section
	let mut low_x = x / 10;
	let mut high_x = x;
	while low_x < high_x {
		let mid_x = (low_x + high_x) / 2;
		let height = get_diag_for_x(data, mid_x);
		if height < TARGET_DIM {
			low_x = mid_x + 1;
		} else {
			high_x = mid_x;
		}
	}

	let y = get_y_for_x(data, low_x);
	let ans_x = low_x - 99;
	let ans_y = y;

	for py in (ans_y - 10)..(ans_y + TARGET_DIM + 10) {
		for px in (ans_x - 10)..(ans_x + TARGET_DIM + 10) {
			print!("{}", if is_tractored(data, px, py) { 
				if px >= ans_x && px < ans_x + TARGET_DIM && py >= ans_y && py < ans_y + TARGET_DIM {
					'O'
				} else {
					'#'
				}
			 } else { ' ' });
		}
		println!("");
	}

	println!("Part 2: {} ({},{})", ans_x * 10000 + ans_y, ans_x, ans_y);
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	part1(&data);
	part2(&data);
}