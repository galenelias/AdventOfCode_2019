
use std::ops::Mul;

// Day 2: Math sucks, not even going to try.
// Day 2 implementation ported from https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbqul0c/

const N: i128 = 119_315_717_514_047;

// 2x2 matrix
#[derive(Copy, Clone)]
struct Matrix {
	a: i128,
	b: i128,
	c: i128,
	d: i128,
}

impl Mul for Matrix {
	type Output = Matrix;

	fn mul(self, o: Matrix) -> Matrix {
		Matrix {
			a: ((self.a*o.a + self.b * o.c)%N + N)%N,
			b: ((self.a*o.b + self.b * o.d)%N + N)%N,
			c: ((self.c*o.a + self.d * o.c)%N + N)%N,
			d: ((self.c*o.b + self.d * o.d)%N + N)%N
		}
	}
}

fn pow128(base: i128, exp: i128) -> i128 {
	if exp == 0 {
		return 1;
	}
	let a = pow128(base, exp/2);
	let a = a * a % N;
	if (exp & 1) != 0 {
		(a * base)%N
	} else {
		a
	}
}

fn mod_inverse(b: i128) -> i128 {
	pow128(b, N-2)
}

fn anti_cut(cut: i128) -> Matrix {
	Matrix { a: 1, b: cut, c: 0, d: 1}
}

fn anti_reverse() -> Matrix {
	Matrix { a: -1, b: N-1, c: 0, d: 1}
}

fn anti_increment(num: i128) -> Matrix {
	Matrix { a: mod_inverse(num), b: 0, c: 0, d: 1}
}

fn pow_matrix(mat: &Matrix, exp: i128) -> Matrix {
	if exp == 0 {
		return Matrix { a: 1, b: 0, c: 0, d: 1};
	}
	let mut ans = pow_matrix(mat, exp / 2);
	ans = ans * ans;
	if (exp & 1) != 0 {
		return *mat * ans;
	} else {
		return ans;
	}
}

pub fn solve(inputs : Vec<String>) {
	let mut deck: Vec<usize> = Vec::new();
	const DECK_LEN: usize = 10007;
	deck.reserve(DECK_LEN);
	for i in 0..DECK_LEN {
		deck.push(i);
	}

	let mut matrix = Matrix{a: 1, b: 0, c: 0, d: 1};

	for input in &inputs {
		if input == "deal into new stack" {
			deck.reverse();
			matrix = matrix * anti_reverse();
		} else if input.starts_with("cut") {
			let cut_amt = input.split(' ').skip(1).next().unwrap().parse::<i64>().unwrap();
			if cut_amt > 0 {
				deck.rotate_left(cut_amt as usize);
			} else {
				deck.rotate_right((-cut_amt) as usize);
			}

			matrix = matrix * anti_cut(cut_amt as i128);
		} else if input.starts_with("deal with increment") {
			let increment = input.split(' ').skip(3).next().unwrap().parse::<usize>().unwrap();
			let mut new_deck = vec![0; deck.len()];
			let mut pos = 0;
			for i in 0..deck.len() {
				new_deck[pos] = deck[i];
				pos = (pos + increment) % deck.len();
			}

			std::mem::swap(&mut deck, &mut new_deck);
			matrix = matrix * anti_increment(increment as i128);
		}
	}

	let card2019_pos = deck.iter().enumerate().find(|&(_, c)| c == &2019).unwrap().0;
	println!("Part 1: {}", card2019_pos);

	let exp: i128 = 101_741_582_076_661;
	matrix = pow_matrix(&matrix, exp);
	let part2 = (matrix.a * 2020 + matrix.b) % N;
	println!("Part 2: {}", part2);
}