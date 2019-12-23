use itertools::Itertools;
use intcode::Cpu;

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();

	let mut cpus: Vec<Cpu> = Vec::new();
	for i in 0..50 {
		cpus.push(Cpu::new(data.clone(), &[i]));
	}

	let mut nat_value: Option<(i64, i64)> = None;
	let mut last_nic_zero_delivery = (0, 0);
	let mut part1_completed = false;
	loop {
		for i in 0..50 {

			let are_all_idle = cpus.iter().all(|cpu| cpu.input_buffer.is_empty());
			if are_all_idle && nat_value.is_some() {
				let nat_value = nat_value.unwrap();
				if nat_value == last_nic_zero_delivery {
					println!("Part 2: {}", last_nic_zero_delivery.1);
					return;
				}
				cpus[0].input_buffer.push_back(nat_value.0);
				cpus[0].input_buffer.push_back(nat_value.1);
				last_nic_zero_delivery = nat_value;
			}
			if cpus[i].input_buffer.is_empty() {
				cpus[i].input_buffer.push_back(-1);
			}

			cpus[i].run();

			if !cpus[i].output_buffer.is_empty() {
				let target = cpus[i].output_buffer.pop_front().unwrap() as usize;
				let x_val = cpus[i].output_buffer.pop_front().unwrap();
				let y_val = cpus[i].output_buffer.pop_front().unwrap();
				if target == 255 {
					if !part1_completed {
						println!("Part 1: {}", y_val);
						part1_completed = true;
					}
					nat_value = Some((x_val, y_val));
				} else {
					cpus[target].input_buffer.push_back(x_val);
					cpus[target].input_buffer.push_back(y_val);
				}
			}
		}
	}
}