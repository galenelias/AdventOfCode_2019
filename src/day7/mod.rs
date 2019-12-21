use itertools::Itertools;
use permutohedron::LexicalPermutation;
use intcode::Cpu;

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