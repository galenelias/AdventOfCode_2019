use itertools::Itertools;
use intcode::Cpu;

fn simulate(data: Vec<i64>, input_value: i64) -> i64 {
	let mut cpu = Cpu::new(data, &[input_value]);
	cpu.run();
	cpu.output_buffer.pop_back().unwrap()
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	let part1 = simulate(data.clone(), 1);
	println!("Part 1: {}", part1);

	let part2 = simulate(data.clone(), 5);
	println!("Part 2: {}", part2);
}