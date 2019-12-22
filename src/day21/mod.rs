use itertools::Itertools;
use intcode::Cpu;

fn run_ascii_code_program(data: &Vec<i64>, input: &str) -> i64 {
	let mut cpu = Cpu::new(data.clone(), &[]);
	cpu.input_buffer.extend(input.chars().map(|c| c as i64));
	cpu.input_buffer.push_back(10); //newline
	cpu.run();

	if cpu.output_buffer.back().unwrap() > &128 {
		return cpu.output_buffer.pop_back().unwrap();
	} else {
		let output = cpu.output_buffer.iter().filter(|&i| i < &128).map(|&i| (i as u8) as char).collect::<String>();
		println!("Output: {}", output);
		return 0;
	}
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	let part1 = run_ascii_code_program(&data, 
		"NOT A T
		NOT T J
		AND B J
		AND C J
		NOT J J
		AND D J
		WALK");
	println!("Part 1: {}", part1);

	let part2 = run_ascii_code_program(&data,
		"NOT A T
		NOT T J
		AND B J
		AND C J
		NOT J J
		AND D J
		NOT E T
		NOT T T
		OR  H T
		AND T J
		RUN");
	println!("Part 2: {}", part2);
}