use itertools::Itertools;
use intcode::Cpu;

fn run_cpu(data: &Vec<i64>, input: i64) -> i64 {
	let mut cpu = Cpu::new(data.clone(), &[input]);
	cpu.run();
	cpu.output_buffer.pop_front().unwrap()
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	println!("Part 1: {}", run_cpu(&data, 1));
	println!("Part 2: {}", run_cpu(&data, 2));
}