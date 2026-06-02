use std::fs;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let path = &args[1];
	let input = fs::read_to_string(path).unwrap();
	let input = input.trim();

	let graph = Graph::from(input);

	dbg!(graph);
}

type Number = u16;

#[derive(Debug)]
struct Graph {
	node_count: Number,
	edge_count: Number,
	edges: Vec<Edge>,
}

#[derive(Debug)]
struct Edge(Number, Number);

fn parse_str_opt(string: Option<&str>) -> Number {
	string.unwrap().parse().unwrap()
}

impl From<&str> for Graph {
	fn from(text: &str) -> Self {
		let mut lines = text.lines();

		let mut first_line = lines.next().unwrap().split(' ');

		let _p = first_line.next();
		let _edge = first_line.next();
		let node_count = parse_str_opt(first_line.next());
		let edge_count = parse_str_opt(first_line.next());

		let edges = lines
			.map(|line| {
				let mut words = line.split(' ');

				let _k = words.next();
				let from = parse_str_opt(words.next());
				let to = parse_str_opt(words.next());

				Edge(from, to)
			})
			.collect();

		Self {
			node_count,
			edge_count,
			edges,
		}
	}
}

impl Graph {
	pub fn to_formula(&self) -> Conjunction {
		Conjunction(
			[
				self.in_min_1_group(),
				self.in_max_1_group(),
				self.know_each_other(),
				self.distinct(),
			]
			.concat(),
		)
	}

	fn in_min_1_group(&self) -> Vec<Disjunction> {
		let mut disjunctions = vec![];

		for x in 0..self.node_count {
			let mut inclusions = vec![];

			for y in 0..self.node_count {
				for z in 0..self.node_count {
					if y < z {
						inclusions.push(Inclusion::new(x, y, z));
					}
				}
			}

			disjunctions.push(Disjunction(inclusions));
		}

		disjunctions
	}

	fn in_max_1_group(&self) -> Vec<Disjunction> {
		todo!()
	}

	fn know_each_other(&self) -> Vec<Disjunction> {
		todo!()
	}

	fn distinct(&self) -> Vec<Disjunction> {
		todo!()
	}
}

#[derive(Debug, Clone)]
/// inclusion in group
struct Inclusion {
	x: Number,
	y: Number,
	z: Number,
	negation: bool,
}

impl Inclusion {
	#[rustfmt::skip]
	pub fn new(x: Number, y: Number, z: Number) -> Self {
		Self { x, y, z, negation: false, }
	}

	#[rustfmt::skip]
	pub fn negated(x: Number, y: Number, z: Number) -> Self {
		Self { x, y, z, negation: true,
		}
	}
}

#[derive(Debug, Clone)]
struct Disjunction(Vec<Inclusion>);

#[derive(Debug, Clone)]
struct Conjunction(Vec<Disjunction>);
