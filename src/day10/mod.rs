use itertools::Itertools;
use std::collections::HashSet; 

fn build_asteroids(inputs : &Vec<Vec<char>>) -> HashSet<(i64,i64)> {
	let mut result = HashSet::new();
	for row in 0..inputs.len() {
		for col in 0..inputs[row].len() {
			if inputs[row][col] == '#' {
				result.insert((row as i64, col as i64));
			}
		}
	}
	return result;
}

fn can_see(a: &(i64, i64), b: &(i64, i64), asteroids: &HashSet<(i64,i64)>) -> bool {
	if a == b {
		return false;
	}

	for c in asteroids {
		if c == a || c == b {
			continue;
		}

		// Compute the area of the triangle to determine if the points are collinear
		let area = a.0 * (b.1 - c.1) +  
			b.0 * (c.1 - a.1) +  
			c.0 * (a.1 - b.1);

		// Check if asteroid 'c' is in between a and b
		if area == 0 && ((c.0 >= a.0 && c.0 <= b.0) || (c.0 >= b.0 && c.0 <= a.0)) && ((c.1 >= a.1 && c.1 <= b.1) || (c.1 >= b.1 && c.1 <= a.1)) {
			return false;
		}
	}

	true
}

pub fn solve(inputs : Vec<String>) {
	let inputs = inputs.iter().map(|s| s.chars().collect_vec()).collect_vec();
	let asteroids = build_asteroids(&inputs);

	let base = asteroids.iter().max_by_key(|a| {
		asteroids.iter().filter(|b| can_see(a, b, &asteroids)).count()
	}).unwrap();

	let get_angle = |a: &(i64, i64)| {
		let result = ((a.1 - base.1) as f64).atan2(-(a.0 - base.0) as f64);
		if result < 0.0f64 {
			result + std::f64::consts::PI * 2.0f64
		} else {
			result
		}
	};

	let mut visible_asteroids = asteroids.iter().filter(|b| can_see(base, b, &asteroids)).collect_vec();
	visible_asteroids.sort_by(|a, b| get_angle(a).partial_cmp(&get_angle(b)).unwrap());

	println!("Part 1: {}", visible_asteroids.len());
	println!("Part 2: {}", visible_asteroids[199].1 * 100 + visible_asteroids[199].0);
}