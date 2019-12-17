use itertools::Itertools;
use std::collections::HashMap;

struct RecipePart {
	chemical: String,
	amount: usize,
}

impl RecipePart {
	fn from_parts(parts: &Vec<&str>) -> RecipePart {
		RecipePart {
			amount: parts[0].parse::<usize>().unwrap(), 
			chemical: parts[1].to_string(),
		}
	}
}

struct Recipe {
	from_parts: Vec<RecipePart>,
	to_part: RecipePart,
}

fn sort_recipes_by_rank(mut recipes: Vec<Recipe>) -> Vec<Recipe> {
	let mut ranks = HashMap::new();
	ranks.insert("ORE".to_string(), 1);

	while ranks.len() < recipes.len() + 1 {
		for recipe in &recipes {
			if recipe.from_parts.iter().all(|part| ranks.contains_key(part.chemical.as_str())) {
				let rank = recipe.from_parts.iter().map(|part| ranks.get(part.chemical.as_str()).unwrap()).max().unwrap().clone();
				ranks.insert(recipe.to_part.chemical.clone(), rank + 1);
			}
		}
	}

	recipes.sort_unstable_by_key(|recipe| -1 * ranks.get(recipe.to_part.chemical.as_str()).unwrap());
	return recipes
}

fn get_ore_for_fuel(recipes: &Vec<Recipe>, fuel_amt: usize) -> usize {
	let mut materials = HashMap::new();
	materials.insert(String::from("FUEL"), fuel_amt);

	for recipe in recipes {
		let mat_amt = materials.get(&recipe.to_part.chemical).unwrap();
		let ratio = (mat_amt + recipe.to_part.amount - 1) / recipe.to_part.amount;

		for from_part in &recipe.from_parts {
			*(materials.entry(from_part.chemical.clone()).or_default()) += from_part.amount * ratio;
		}
	}

	return *materials.get(&String::from("ORE")).unwrap();
}

fn get_fuel_from_ore(recipes: &Vec<Recipe>, ore_amt: usize) -> usize {
	let mut high_fuel = 1;
	while get_ore_for_fuel(recipes, high_fuel) < ore_amt {
		high_fuel *= 10;
	}

	let mut low_fuel = high_fuel / 10;

	while low_fuel < high_fuel {
		let mid = (low_fuel + high_fuel) / 2;
		let ore = get_ore_for_fuel(recipes, mid);
		if ore < ore_amt {
			low_fuel = mid + 1;
		} else {
			high_fuel = mid;
		}
	}

	return low_fuel - 1;
}

pub fn solve(inputs : Vec<String>) {
	let mut recipes = inputs.iter().map( |line| {
		let mut parts = line.split(" => ");
		let from_parts = parts.next().unwrap().split(", ").map(|part| RecipePart::from_parts(&part.split(" ").collect_vec())).collect_vec();
		let to_part = RecipePart::from_parts(&parts.next().unwrap().split(" ").collect_vec());
		Recipe{ from_parts, to_part }
	}).collect_vec();

	recipes = sort_recipes_by_rank(recipes);

	println!("Part 1: {}", get_ore_for_fuel(&recipes, 1));
	println!("Part 2: {}", get_fuel_from_ore(&recipes, 1_000_000_000_000));
}