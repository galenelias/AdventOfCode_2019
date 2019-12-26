use itertools::Itertools;
use intcode::Cpu;
use std::io::{stdin};

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].split(",").map(|w| w.parse::<i64>().unwrap()).collect_vec();
	let mut cpu = Cpu::new(data.clone(), &[]);

	loop {
		let did_halt = cpu.run();
		let output = cpu.output_buffer.iter().map(|&i| i as u8 as char).collect::<String>();
		cpu.output_buffer.clear();
		println!("{}", output);

		if did_halt {
			println!("Program terminated!");
			break;
		}

		let mut s=String::new();
		stdin().read_line(&mut s).expect("Did not enter a correct string");

		if s.len() == 0 {
			break;
		}
		cpu.input_buffer.extend(s.chars().map(|c| c as u8 as i64));
	}

}