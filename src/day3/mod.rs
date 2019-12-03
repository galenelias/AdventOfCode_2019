use itertools::Itertools;
use std::collections::HashMap;

// Create a map of points -> steps for every point in the wire path
fn map_wire(dirs: &Vec<&str>) -> HashMap<(i32, i32), usize> {
	let mut x = 0;
	let mut y = 0;
	let mut steps = 0;
	let mut result = HashMap::new();

	for dir in dirs {
		let d = &dir[0..1];
		let dist = dir[1..].parse::<usize>().unwrap();

		let (dx, dy) = match d {
			"L" => (-1, 0),
			"R" => ( 1, 0), 
			"D" => ( 0,-1),
			"U" => ( 0, 1),
			_ => unreachable!("Unexpected dir: {}", d),
		};

		for _ in 0..dist {
			x += dx;
			y += dy;
			steps += 1;
			result.entry((x, y)).or_insert(steps);
		}
	}

	return result;
}


pub fn solve(inputs : Vec<String>) {
	let wire1 = inputs[0].split(",").collect_vec();
	let wire2 = inputs[1].split(",").collect_vec();

	let wire1_pts = map_wire(&wire1);
	let wire2_pts = map_wire(&wire2);

	let mut closest_intersection = i32::max_value();
	let mut lowest_steps_intersection = usize::max_value();

	for (pt1, steps1) in wire1_pts {
		if let Some(steps2) = wire2_pts.get(&pt1) {
			closest_intersection = std::cmp::min(closest_intersection, pt1.0.abs() + pt1.1.abs());
			lowest_steps_intersection = std::cmp::min(lowest_steps_intersection, steps1 + steps2);
		}
	}

	println!("Part 1: {}", closest_intersection);
	println!("Part 2: {}", lowest_steps_intersection);
}