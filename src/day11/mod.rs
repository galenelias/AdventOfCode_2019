use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

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

enum CompassDirections {
	North, East, South, West
}

fn paint_hull(mut cpu: Cpu, part2: bool) {
	let mut hull = HashMap::new();
	let mut x = 0;
	let mut y = 0;
	let mut dir = CompassDirections::North;

	if part2 {
		hull.insert((0, 0), 1);
	}

	loop {
		cpu.input_buffer.push_back(*hull.get(&(x, y)).unwrap_or(&0));
		let did_halt = cpu.run();
		if did_halt {
			break;
		}

		hull.insert((x, y), cpu.output_buffer.pop_front().unwrap());

		let turn = cpu.output_buffer.pop_front().unwrap();
		let current_dir = dir;
		dir = if turn == 0 {
			match current_dir {
				CompassDirections::North => CompassDirections::West,
				CompassDirections::West => CompassDirections::South,
				CompassDirections::South => CompassDirections::East,
				CompassDirections::East => CompassDirections::North,
			}
		} else {
			match current_dir {
				CompassDirections::North => CompassDirections::East,
				CompassDirections::East => CompassDirections::South,
				CompassDirections::South => CompassDirections::West,
				CompassDirections::West => CompassDirections::North,
			}
		};

		match dir {
			CompassDirections::North => y += 1,
			CompassDirections::East => x += 1,
			CompassDirections::South => y -= 1,
			CompassDirections::West => x -= 1,
		}
	}

	if !part2 {
		println!("Part 1: {}", hull.len());
	} else {
		println!("Part 2:");
		let min_x = hull.keys().min_by_key(|coord| coord.0).unwrap().0.clone();
		let max_x = hull.keys().max_by_key(|coord| coord.0).unwrap().0.clone();
		let min_y = hull.keys().min_by_key(|coord| coord.1).unwrap().1.clone();
		let max_y = hull.keys().max_by_key(|coord| coord.1).unwrap().1.clone();

		for y in (min_y..=max_y).rev() {
			for x in min_x..=max_x {
				let color = hull.get(&(x, y)).unwrap_or(&0);
				print!("{}", if color == &1 { '#' } else { ' ' });
			}
			println!("")
		}
	}
}


pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	paint_hull(Cpu::new(data.clone(), &[]), false);
	paint_hull(Cpu::new(data.clone(), &[]), true);
}