use itertools::Itertools;
use intcode::Cpu;
use std::collections::{VecDeque, HashSet, HashMap};

type Pos = (i32, i32);

const CMD_NORTH: i64 = 1;
const CMD_SOUTH: i64 = 2;
const CMD_WEST: i64 = 3;
const CMD_EAST: i64 = 4;

const TILE_UNKNOWN: i64 = -1;
const TILE_WALL: i64 = 0;
const TILE_OPEN: i64 = 1;
const TILE_OXYGEN: i64 = 2;

fn opposite_cmd(cmd: i64) -> i64 {
	match cmd {
		CMD_NORTH => CMD_SOUTH,
		CMD_SOUTH => CMD_NORTH,
		CMD_WEST => CMD_EAST,
		CMD_EAST => CMD_WEST,
		_ => unreachable!(),
	}
}

fn _print_grid(map: &HashMap<Pos, i64>) {
	let min_y = map.keys().min_by_key(|pos| pos.0).unwrap().0;
	let max_y = map.keys().max_by_key(|pos| pos.0).unwrap().0;
	let min_x = map.keys().min_by_key(|pos| pos.1).unwrap().1;
	let max_x = map.keys().max_by_key(|pos| pos.1).unwrap().1;

	println!(" [{} - {}]x[{} - {}]", min_y, max_y, min_x, max_x);
	for y in min_y..=max_y {
		for x in min_x..=max_x {
			let tile = map.get(&(y, x)).unwrap_or(&TILE_UNKNOWN);
			print!("{}", match tile {
				&TILE_UNKNOWN => ' ',
				&TILE_WALL => '#',
				&TILE_OPEN => '.',
				&TILE_OXYGEN => 'O',
				_ => unreachable!(),
			})
		}
		println!("");
	}
}

fn explore(cpu: &mut Cpu, map: &mut HashMap<Pos, i64>, pos: &Pos) {
	let mut try_move = |cmd, new_pos| {
		if !map.contains_key(&new_pos) {
			cpu.input_buffer.push_back(cmd);
			cpu.run();
			let result = cpu.output_buffer.pop_front().unwrap();
			map.insert(new_pos, result);
			if result != 0 { // not a wall
				explore(cpu, map, &new_pos);
				cpu.input_buffer.push_back(opposite_cmd(cmd));
				cpu.run();
				cpu.output_buffer.pop_front();
			}
		}
	};

	try_move(CMD_NORTH, (pos.0 - 1, pos.1));
	try_move(CMD_WEST, (pos.0, pos.1 - 1));
	try_move(CMD_SOUTH, (pos.0 + 1, pos.1));
	try_move(CMD_EAST, (pos.0, pos.1 + 1));
}

fn bfs(map: &HashMap<Pos, i64>, start_pos: &Pos, target_pos: &Pos) -> usize {
	let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
	queue.push_back((start_pos.clone(), 0));
	let mut visited = HashSet::new();
	let mut max_steps = 0;

	while !queue.is_empty() {
		let (pos, steps) = queue.pop_front().unwrap();
		if !visited.insert(pos) {
			continue;
		} else if map.get(&pos).unwrap_or(&TILE_UNKNOWN) == &TILE_WALL {
			continue;
		}

		if &pos == target_pos {
			return steps;
		}

		max_steps = std::cmp::max(max_steps, steps);

		queue.push_back(((pos.0 - 1, pos.1), steps + 1));
		queue.push_back(((pos.0 + 1, pos.1), steps + 1));
		queue.push_back(((pos.0, pos.1 - 1), steps + 1));
		queue.push_back(((pos.0, pos.1 + 1), steps + 1));
	}

	return max_steps;
}

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	let mut cpu = Cpu::new(data.clone(), &[]);
	cpu.run();

	let mut map = HashMap::new();
	map.insert((0, 0), 1);
	explore(&mut cpu, &mut map, &(0, 0));

	let oxygen_pos = map.iter().find(|&(_, v)| v == &TILE_OXYGEN).unwrap().0;

	println!("Part 1: {}", bfs(&map, &(0, 0), oxygen_pos));
	println!("Part 2: {}", bfs(&map, oxygen_pos, &(999,999)));
}