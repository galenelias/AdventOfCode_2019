use itertools::Itertools;

const LAYER_HEIGHT: usize = 6;
const LAYER_WIDTH: usize = 25;

pub fn solve(inputs : Vec<String>) {
	let data = inputs[0].chars().map(|ch| ch.to_digit(10).unwrap()).collect_vec();
	let layers = data.chunks(LAYER_WIDTH * LAYER_HEIGHT).collect_vec();

	let pixel_counts = layers.iter().map(|layer| {
		(layer.iter().filter(|&pixel| pixel == &0).count(),
		 layer.iter().filter(|&pixel| pixel == &1).count(),
		 layer.iter().filter(|&pixel| pixel == &2).count())
	}).collect_vec();

	let min_zero_layer = pixel_counts.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();
	println!("Part 1: {:?}", min_zero_layer.1 * min_zero_layer.2);

	println!("Part 2:");
	for row in 0..LAYER_HEIGHT {
		for col in 0..LAYER_WIDTH {
			for layer in &layers {
				match layer[row * LAYER_WIDTH + col] {
					0 => {print!(" "); break;},
					1 => {print!("X"); break;},
					_ => {},
				};
			}
		}
		println!("");
	}
}