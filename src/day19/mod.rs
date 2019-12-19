use itertools::Itertools;
use std::collections::{VecDeque};

struct Cpu {
	ip: usize,
	relative_offset: i64,
	data: Vec<i64>,
	input_buffer: VecDeque<i64>,
	output_buffer: VecDeque<i64>,
}

impl Cpu {
	fn new(data: Vec<i64>, inputs: &[i64]) -> Cpu {
		let mut result = Cpu { data: data, relative_offset: 0, ip: 0, input_buffer: VecDeque::new(), output_buffer: VecDeque::new()};
		result.input_buffer.extend(inputs);
		return result;
	}

	fn read_mem(&self, addr: usize) -> i64 {
		if addr >= self.data.len() {
			0
		} else {
			self.data[addr]
		}
	}

	fn write_mem(&mut self, addr: usize, val: i64) {
		if addr >= self.data.len() {
			self.data.resize_with(addr+1, Default::default);
		}
		self.data[addr] = val;
	}

	fn run(&mut self) -> bool {
		while self.data[self.ip] != 99 {
			let op = self.data[self.ip] % 100;
			let addressing_modes = (self.data[self.ip] as usize) / 100;

			let get_param = |param: usize| -> i64 {
				let mode = (addressing_modes / (10usize.pow(param as u32 - 1))) % 10;
				match mode {
					0 => self.read_mem(self.data[self.ip + param] as usize),
					1 => self.data[self.ip + param],
					2 => self.read_mem((self.data[self.ip + param] + self.relative_offset) as usize),
					_ => unreachable!("Unexpected addressing mode: {}", mode),
				}
			};

			let get_dest = |param: usize| -> usize {
				let mode = (addressing_modes / (10usize.pow(param as u32 - 1))) % 10;
				match mode {
					0 => self.data[self.ip + param] as usize,
					2 => (self.data[self.ip + param] + self.relative_offset) as usize,
					_ => unreachable!("Unexpected addressing mode: {}", mode),
				}
			};

			match op {
				1 => { // ADD
					let dest = get_dest(3);
					let result = get_param(1) + get_param(2);
					self.write_mem(dest, result);
					self.ip += 4;
				},
				2 => { // MUL
					let dest = get_dest(3);
					let result = get_param(1) * get_param(2);
					self.write_mem(dest, result);
					self.ip += 4;
				},
				3 => { // INPUT
					if self.input_buffer.is_empty() {
						return false;
					}
					let dest = get_dest(1);
					let input = self.input_buffer.pop_front().unwrap();
					self.write_mem(dest, input);
					self.ip += 2;
				},
				4 => { // OUTPUT
					let output_val = get_param(1);
					self.output_buffer.push_back(output_val);
					self.ip += 2;
				}
				5 => { // jump if true
					if get_param(1) != 0 {
						self.ip = get_param(2) as usize;
					} else {
						self.ip += 3;
					}
				}
				6 => { // jump if false
					if get_param(1) == 0 {
						self.ip = get_param(2) as usize;
					} else {
						self.ip += 3;
					}
				}
				7 => { // less than
					let dest = get_dest(3);
					let result = if get_param(1) < get_param(2) { 1 } else { 0 };
					self.write_mem(dest, result);
					self.ip += 4;
				}
				8 => { // equals
					let dest = get_dest(3);
					let result = if get_param(1) == get_param(2) { 1 } else { 0 };
					self.write_mem(dest, result);
					self.ip += 4;
				}
				9 => { // # Adjust Relative Offsset
					self.relative_offset += get_param(1);
					self.ip += 2;
				}
				_ => unreachable!("Unexpected opcode: {}", self.data[self.ip]),
			}
		}

		return true;
	}
}

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