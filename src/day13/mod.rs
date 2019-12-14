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

fn part1(data: Vec<i64>) {
	let mut cpu = Cpu::new(data.clone(), &[]);
	cpu.run();

	let blocks = cpu.output_buffer.iter().collect_vec().chunks(3).filter(|tile| tile[2] == &2).count();
	println!("Part 1: {}", blocks);
}

fn part2(mut data: Vec<i64>) {
	data[0] = 2; // insert 2 quarters
	let mut cpu = Cpu::new(data.clone(), &[]);

	let mut ball_pos = (0, 0);
	let mut paddle_pos = (0, 0);
	let mut score = 0;

	loop {
		let did_halt = cpu.run();

		let tile_buffer = cpu.output_buffer.iter().cloned().collect_vec();
		for tile in tile_buffer.chunks(3) {
			let pos = (tile[0], tile[1]);
			let tile_id = tile[2];

			if pos == (-1, 0) { // Score
				score = tile_id;
			}
			else if tile_id == 3 { // paddle
				paddle_pos = pos;
			}
			else if tile_id == 4 { // ball
				ball_pos = pos;
			}
		}
		cpu.output_buffer.clear();

		if did_halt {
			break;
		}

		// Move the paddle to always be under the ball
		cpu.input_buffer.push_back(if paddle_pos.0 > ball_pos.0 { -1 } else if paddle_pos.0 < ball_pos.0 { 1 } else { 0 });
	}

	println!("Part 2: {}", score);
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	part1(data.clone());
	part2(data.clone());
}