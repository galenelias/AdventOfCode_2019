use itertools::Itertools;

fn simulate(mut data: Vec<usize>, noun: usize, verb: usize) -> usize {
	data[1] = noun;
	data[2] = verb;

	let mut ip = 0;
	while data[ip] != 99 {
		let dest = data[ip + 3];
		match data[ip] {
			1 => data[dest] = data[data[ip + 1]] + data[data[ip + 2]],
			2 => data[dest] = data[data[ip + 1]] * data[data[ip + 2]],
			_ => unreachable!("Unexpected opcode: {}", data[ip]),
		}
		ip += 4;
	}

	return data[0];
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<usize>().unwrap()).collect_vec();

	println!("Part 1: {:?}", simulate(data.clone(), 12, 2));

	for noun in 0..100 {
		for verb in 0..100 {
			if simulate(data.clone(), noun, verb) == 19690720 {
				println!("Part 2: {}", noun * 100 + verb);
			}
		}
	}
}