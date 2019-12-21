use itertools::Itertools;
use std::collections::{HashMap};
use intcode::Cpu;

enum CompassDirections {
	North, East, South, West
}

fn paint_hull(mut cpu: Cpu, part2: bool) {
	let mut hull = HashMap::new();
	let mut x = 0;
	let mut y = 0;
	let mut dir = CompassDirections::North;

	if part2 {
		hull.insert((0, 0), 1);
	}

	loop {
		cpu.input_buffer.push_back(*hull.get(&(x, y)).unwrap_or(&0));
		let did_halt = cpu.run();
		if did_halt {
			break;
		}

		hull.insert((x, y), cpu.output_buffer.pop_front().unwrap());

		let turn = cpu.output_buffer.pop_front().unwrap();
		let current_dir = dir;
		dir = if turn == 0 {
			match current_dir {
				CompassDirections::North => CompassDirections::West,
				CompassDirections::West => CompassDirections::South,
				CompassDirections::South => CompassDirections::East,
				CompassDirections::East => CompassDirections::North,
			}
		} else {
			match current_dir {
				CompassDirections::North => CompassDirections::East,
				CompassDirections::East => CompassDirections::South,
				CompassDirections::South => CompassDirections::West,
				CompassDirections::West => CompassDirections::North,
			}
		};

		match dir {
			CompassDirections::North => y += 1,
			CompassDirections::East => x += 1,
			CompassDirections::South => y -= 1,
			CompassDirections::West => x -= 1,
		}
	}

	if !part2 {
		println!("Part 1: {}", hull.len());
	} else {
		println!("Part 2:");
		let min_x = hull.keys().min_by_key(|coord| coord.0).unwrap().0.clone();
		let max_x = hull.keys().max_by_key(|coord| coord.0).unwrap().0.clone();
		let min_y = hull.keys().min_by_key(|coord| coord.1).unwrap().1.clone();
		let max_y = hull.keys().max_by_key(|coord| coord.1).unwrap().1.clone();

		for y in (min_y..=max_y).rev() {
			for x in min_x..=max_x {
				let color = hull.get(&(x, y)).unwrap_or(&0);
				print!("{}", if color == &1 { '#' } else { ' ' });
			}
			println!("")
		}
	}
}


pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	paint_hull(Cpu::new(data.clone(), &[]), false);
	paint_hull(Cpu::new(data.clone(), &[]), true);
}