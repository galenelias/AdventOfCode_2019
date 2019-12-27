use itertools::Itertools;

fn char_to_int(ch: char) -> i32 {
	(ch as u8 - '0' as u8) as i32
}

fn int_to_char(i: i32) -> char {
	(i as u8 + '0' as u8) as char
}

fn calc_digit(input: &Vec<char>, input_offset: usize) -> i32 {
	const PATTERN: [i32; 4] = [0, 1, 0, -1];

	let mut result = 0;
	for i in 0..input.len() {
		let x = ((i + 1) / (input_offset + 1)) % PATTERN.len();
		result += PATTERN[x] * char_to_int(input[i]);
	}

	result = result % 10;
	if result < 0 {
		result *= -1;
	}
	return result;
}


fn fft(chars: Vec<char>) -> Vec<char> {
	let mut result = chars;
	for _ in 0..100 {
		let mut temp = Vec::new();
		for i in 0..result.len() {
			temp.push(int_to_char(calc_digit(&result, i)));
		}
		result = temp
	}
	return result;
}

// Optimized fft which only computes the last half of the number, since this can be done extremely efficiently
fn fft2(mut chars: Vec<char>) -> Vec<char> {
	let chars_len = chars.len();
	for _ in 0..100 {
		let mut sum = 0;
		for i in 0..chars_len / 2 {
			sum = (sum + char_to_int(chars[chars_len - 1 - i])) % 10;
			chars[chars_len - 1 - i] = int_to_char(sum);
		}
	}

	return chars;
}

pub fn solve(inputs : Vec<String>) {
	let input = inputs[0].chars().collect_vec();

	let part1_fft = fft(input.clone());
	println!("Part 1: {}", &part1_fft[0..8].iter().collect::<String>());

	let mut part2_input: Vec<char> = Vec::with_capacity(input.len() * 10_000);
	for _ in 0..10_000 {
		part2_input.extend(&input);
	}
	let part2_fft = fft2(part2_input);
	let part2_offset = input[0..7].iter().collect::<String>().parse::<usize>().unwrap();
	println!("Part 2: {}", part2_fft[part2_offset..part2_offset+8].iter().collect::<String>());
}