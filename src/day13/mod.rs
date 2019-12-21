use itertools::Itertools;
use intcode::Cpu;

fn part1(data: Vec<i64>) {
	let mut cpu = Cpu::new(data.clone(), &[]);
	cpu.run();

	let blocks = cpu.output_buffer.iter().collect_vec().chunks(3).filter(|tile| tile[2] == &2).count();
	println!("Part 1: {}", blocks);
}

fn part2(mut data: Vec<i64>) {
	data[0] = 2; // insert 2 quarters
	let mut cpu = Cpu::new(data.clone(), &[]);

	let mut ball_pos = (0, 0);
	let mut paddle_pos = (0, 0);
	let mut score = 0;

	loop {
		let did_halt = cpu.run();

		let tile_buffer = cpu.output_buffer.iter().cloned().collect_vec();
		for tile in tile_buffer.chunks(3) {
			let pos = (tile[0], tile[1]);
			let tile_id = tile[2];

			if pos == (-1, 0) { // Score
				score = tile_id;
			}
			else if tile_id == 3 { // paddle
				paddle_pos = pos;
			}
			else if tile_id == 4 { // ball
				ball_pos = pos;
			}
		}
		cpu.output_buffer.clear();

		if did_halt {
			break;
		}

		// Move the paddle to always be under the ball
		cpu.input_buffer.push_back(if paddle_pos.0 > ball_pos.0 { -1 } else if paddle_pos.0 < ball_pos.0 { 1 } else { 0 });
	}

	println!("Part 2: {}", score);
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	part1(data.clone());
	part2(data.clone());
}