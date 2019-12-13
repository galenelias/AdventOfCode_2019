use itertools::Itertools;
use regex::Regex;
use num::integer::lcm;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Moon {
	pos: [i64; 3],
	vel: [i64; 3],
}

fn move_moons(mut moons: Vec<Moon>) -> Vec<Moon> {
	for i in 0..moons.len() {
		for j in (i+1)..moons.len() {
			for k in 0..3 {
				if moons[i].pos[k] < moons[j].pos[k] {
					moons[i].vel[k] += 1;
					moons[j].vel[k] -= 1;
				} else if moons[i].pos[k] > moons[j].pos[k] {
					moons[i].vel[k] -= 1;
					moons[j].vel[k] += 1;
				}
			}
		}
	}

	for moon in &mut moons {
		for k in 0..3 {
			moon.pos[k] += moon.vel[k];
		}
	}

	return moons;
}

pub fn solve(inputs : Vec<String>) {
	let re_input = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

	let mut moons = inputs.iter().map(|line| {
		let caps = re_input.captures(&line).unwrap();
		Moon {
			pos: [caps[1].parse::<i64>().unwrap(),caps[2].parse::<i64>().unwrap(),caps[3].parse::<i64>().unwrap()],
			vel: [0, 0, 0],
		}
	}).collect_vec();

	let moons_orig = moons.clone();

	for _ in 0..1000 {
		moons = move_moons(moons);
	}
	let total_energy = moons.iter().map(|moon|
		moon.pos.iter().map(|v| v.abs()).sum::<i64>() * moon.vel.iter().map(|v| v.abs()).sum::<i64>()
	).sum::<i64>();
	println!("Part 1: {}", total_energy);

	let mut moons = moons_orig.clone();
	let mut multipliers = [0; 3];
	'outer: for iteration in 0usize.. {
		moons = move_moons(moons);

		for i in 0..3 {
			if moons.iter().zip(moons_orig.iter()).all(|(m1, m2)| m1.pos[i] == m2.pos[i] && m1.vel[i] == m2.vel[i]) {
				if multipliers[i] == 0 {
					multipliers[i] = iteration + 1;
					if multipliers.iter().all(|i| i != &0) {
						break 'outer;
					}
				}
			}
		}
	}

	let lcm = lcm(multipliers[0], lcm(multipliers[1], multipliers[2]));
	println!("Part 2: {}", lcm);
}