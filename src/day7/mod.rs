use itertools::Itertools;
use std::collections::VecDeque;
use permutohedron::LexicalPermutation;

struct Cpu {
	ip: usize,
	data: Vec<i64>,
	input_buffer: VecDeque<i64>,
	output_buffer: VecDeque<i64>,
}

impl Cpu {
	fn new(data: Vec<i64>, inputs: &[i64]) -> Cpu {
		let mut result = Cpu { data: data, ip: 0, input_buffer: VecDeque::new(), output_buffer: VecDeque::new()};
		result.input_buffer.extend(inputs);
		return result;
	}

	fn run(&mut self) -> bool {
		while self.data[self.ip] != 99 {
			let op = self.data[self.ip] % 100;
			let addressing_modes = (self.data[self.ip] as usize) / 100;

			let get_param = |param: usize| -> i64 {
				let is_immediate = ((addressing_modes / (10usize.pow(param as u32 - 1))) % 10) == 1;
				if is_immediate {
					self.data[self.ip + param]
				} else {
					self.data[self.data[self.ip + param] as usize]
				}
			};

			match op {
				1 => { // ADD
					let dest = self.data[self.ip + 3] as usize;
					self.data[dest] = get_param(1) + get_param(2);
					self.ip += 4;
				},
				2 => { // MUL
					let dest = self.data[self.ip + 3] as usize;
					self.data[dest] = get_param(1) * get_param(2);
					self.ip += 4;
				},
				3 => { // INPUT
					if self.input_buffer.is_empty() {
						return false;
					}
					let dest = self.data[self.ip + 1] as usize;
					let input = self.input_buffer.pop_front().unwrap();
					self.data[dest] = input;
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
					let dest = self.data[self.ip + 3] as usize;
					let result = if get_param(1) < get_param(2) { 1 } else { 0 };
					self.data[dest] = result;
					self.ip += 4;
				}
				8 => { // equals
					let dest = self.data[self.ip + 3] as usize;
					let result = if get_param(1) == get_param(2) { 1 } else { 0 };
					self.data[dest] = result;
					self.ip += 4;
				}
				_ => unreachable!("Unexpected opcode: {}", self.data[self.ip]),
			}
		}

		return true;
	}
}

fn run_amplifiers(data: &Vec<i64>, mut inputs: Vec<i64>) -> i64 {
	let mut max_result = 0;

	loop {
		let mut cpus = [
			Cpu::new(data.clone(), &[inputs[0], 0]),
			Cpu::new(data.clone(), &[inputs[1]]),
			Cpu::new(data.clone(), &[inputs[2]]),
			Cpu::new(data.clone(), &[inputs[3]]),
			Cpu::new(data.clone(), &[inputs[4]]),
		];

		let mut is_completed = false;
		while !is_completed {
			for i in 0..5 {
				let halted = cpus[i].run();
				if i == 4 && halted {
					is_completed = true;
					break;
				}

				if let Some(output) = cpus[i].output_buffer.pop_front() {
					cpus[(i+1)%5].input_buffer.push_back(output);
				}
			}
		}

		let result = cpus[4].output_buffer.pop_front().unwrap();
		max_result = std::cmp::max(max_result, result);

		if !inputs.next_permutation() {
			break;
		}
	}

	max_result
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	println!("Part 1: {}", run_amplifiers(&data, vec![0, 1, 2, 3, 4]));
	println!("Part 2: {}", run_amplifiers(&data, vec![5, 6, 7, 8, 9]));
}