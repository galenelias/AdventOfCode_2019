use itertools::Itertools;

fn simulate(mut data: Vec<i32>, input_value: i32) -> Vec<i32> {
	let mut ip = 0;
	let mut output = Vec::new();

	while data[ip] != 99 {
		let op = data[ip] % 100;
		let addressing_modes = (data[ip] as usize) / 100;

		let get_param = |param: usize| -> i32 {
			let is_immediate = ((addressing_modes / (10usize.pow(param as u32 - 1))) % 10) == 1;
			if is_immediate {
				data[ip + param]
			} else {
				data[data[ip + param] as usize]
			}
		};

		match op {
			1 => {
				let dest = data[ip + 3] as usize;
				data[dest] = get_param(1) + get_param(2);
				ip += 4;
			},
			2 => {
				let dest = data[ip + 3] as usize;
				data[dest] = get_param(1) * get_param(2);
				ip += 4;
			},
			3 => {
				let dest = data[ip + 1] as usize;
				data[dest] = input_value;
				ip += 2;
			},
			4 => {
				output.push(get_param(1));
				ip += 2;
			}
			5 => { // jump if true
				if get_param(1) != 0 {
					ip = get_param(2) as usize;
				} else {
					ip += 3;
				}
			}
			6 => { // jump if false
				if get_param(1) == 0 {
					ip = get_param(2) as usize;
				} else {
					ip += 3;
				}
			}
			7 => { // less than
				let dest = data[ip + 3] as usize;
				let result = if get_param(1) < get_param(2) { 1 } else { 0 };
				data[dest] = result;
				ip += 4;
			}
			8 => { // equals
				let dest = data[ip + 3] as usize;
				let result = if get_param(1) == get_param(2) { 1 } else { 0 };
				data[dest] = result;
				ip += 4;
			}
			_ => unreachable!("Unexpected opcode: {}", data[ip]),
		}
	}

	return output;
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i32>().unwrap()).collect_vec();

	let part1 = simulate(data.clone(), 1);
	println!("Part 1: {}", part1.last().unwrap());

	let part2 = simulate(data.clone(), 5);
	println!("Part 2: {}", part2.last().unwrap());
}