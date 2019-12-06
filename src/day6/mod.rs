use itertools::Itertools;
use std::collections::{HashSet, HashMap, VecDeque};

fn sub_orbits(graph: &HashMap<String, Vec<String>>, seed: &String) -> usize {
	if let Some(children) = graph.get(seed) {
		// Sub-orbits are all child orbits, plus all sub-orbits of all children (recursively)
		children.len() + children.iter().map(|child| sub_orbits(graph, child)).sum::<usize>()
	} else {
		0
	}
}

fn bfs(graph: &HashMap<String, Vec<String>>, from: &str, to: &str) -> usize {
	let mut q = VecDeque::new();
	let mut visited = HashSet::new();

	q.push_back((from, 0));
	while !q.is_empty() {
		let (node, steps) = q.pop_front().unwrap();

		if !visited.insert(node) {
			continue;
		}

		if node == to {
			return steps;
		}

		for child in &graph[node] {
			q.push_back((child, steps + 1));
		}
	}

	unreachable!("No path found!");
}

pub fn solve(inputs : Vec<String>) {
	let pairs = inputs.iter().map(|input| input.split(")").collect_vec());

	let mut graph: HashMap<String, Vec<String>> = HashMap::new();
	for pair in pairs {
		graph.entry(pair[0].to_string()).or_insert(Vec::new()).push(pair[1].to_string());
	}

	let part1 = graph.keys().map(|key| sub_orbits(&graph, key)).sum::<usize>();
	println!("Part 1: {}", part1);

	let mut neighbors = graph.clone();
	for (key,value) in graph {
		for child in value {
			neighbors.entry(child).or_insert(Vec::new()).push(key.clone());
		}
	}

	println!("Part 2: {}", bfs(&neighbors, "YOU", "SAN") - 2);
}