use itertools::Itertools;

fn is_valid_pass_1(pass: &String) -> bool {
	let chars = pass.chars().collect_vec();
	return chars.windows(2).all(|pair| pair[0] <= pair[1])
		&& chars.windows(2).any(|pair| pair[0] == pair[1]);
}

fn is_valid_pass_2(pass: &String) -> bool {
	let chars = pass.chars().collect_vec();

	if !chars.windows(2).all(|pair| pair[0] <= pair[1]) {
		return false;
	}

	for i in 0..(chars.len()-1) {
		if chars[i] == chars[i+1] && (i == chars.len() - 2 || chars[i] != chars[i+2]) && (i == 0 || chars[i] != chars[i-1]) {
			return true;
		}
	}

	return false;
}

pub fn solve(inputs : Vec<String>) {
	let inputs = inputs[0].split("-").map(|w| w.parse::<i32>().unwrap()).collect_vec();

	println!("Part 1: {}", (inputs[0]..inputs[1]).map(|i| i.to_string()).filter(is_valid_pass_1).count());
	println!("Part 2: {}", (inputs[0]..inputs[1]).map(|i| i.to_string()).filter(is_valid_pass_2).count());
}