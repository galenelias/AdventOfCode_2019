use itertools::Itertools;

fn fuel_for(weight: &i32) -> i32 {
	(weight / 3) - 2
}

fn total_fuel_iter(weight: &i32) -> i32 {
	// Create an iterator sequence of the fuel for a given weight, terminating when it hits zero
	std::iter::successors(Some(fuel_for(weight)), |&input| -> Option<i32> {
		let fuel = fuel_for(&input);
		if fuel <= 0 {
			None
		} else {
			Some(fuel)
		}
	}).sum::<i32>()
}

fn total_fuel_recursive(weight: &i32) -> i32 {
	let fuel = fuel_for(weight);
	if fuel > 0 {
		fuel + total_fuel_recursive(&fuel)
	} else {
		0
	}
}

pub fn solve(inputs : Vec<String>) {
	let inputs = inputs.iter().map(|line| line.parse::<i32>().unwrap()).collect_vec();

	let part1 = inputs.iter().map(fuel_for).sum::<i32>();
	let part2_it = inputs.iter().map(total_fuel_iter).sum::<i32>();
	let part2_rec = inputs.iter().map(total_fuel_recursive).sum::<i32>();

	println!("Part 1: {}", part1);
	println!("Part 2 (iterator):  {}", part2_it);
	println!("Part 2 (recursive): {}", part2_rec);
}