use std::fs;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let path = &args[1];
	let input = fs::read_to_string(path).unwrap();
	let input = input.trim();

	let graph = Graph::from(input);

	println!("{:#?}", &graph);
	println!();
	graph.to_formula();
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
				dbg!(self.in_min_1_group()),
				dbg!(self.in_max_1_group()),
				dbg!(self.know_each_other()),
				dbg!(self.distinct()),
			]
			.concat(),
		)
	}

	fn in_min_1_group(&self) -> Vec<Disjunction> {
		let mut disjunctions = vec![];

		for x in 1..=self.node_count {
			let mut inclusions = vec![];

			for y in 1..=self.node_count {
				for z in 1..=self.node_count {
					if let Some(inclusion) = Inclusion::new(x, y, z)
						&& y < z
					{
						inclusions.push(inclusion);
					}
				}
			}

			disjunctions.push(Disjunction(inclusions));
		}

		disjunctions
	}

	fn in_max_1_group(&self) -> Vec<Disjunction> {
		let mut disjunctions = vec![];

		for x in 1..=self.node_count {
			for y1 in 1..=self.node_count {
				for z1 in 1..=self.node_count {
					if y1 < z1 {
						for y2 in 1..=self.node_count {
							for z2 in 1..=self.node_count {
								if y1 <= y2
									&& y2 < z2 && y1 != y2 && z1 != z2
									&& let (Some(a1), Some(a2)) =
										(Inclusion::new(x, y1, z1), Inclusion::new(x, y2, z2))
								{
									disjunctions
										.push(Disjunction(vec![a1.negated(), a2.negated()]));
								}
							}
						}
					}
				}
			}
		}

		disjunctions
	}

	fn know_each_other(&self) -> Vec<Disjunction> {
		let mut disjunctions = vec![];
		disjunctions
	}

	fn distinct(&self) -> Vec<Disjunction> {
		let mut disjunctions = vec![];
		disjunctions
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
	/// checks for pairwise inequality
	pub fn new(x: Number, y: Number, z: Number) -> Option<Self> {
		if x != y && y != z && x!=z  {
			Some(Self { x, y, z, negation: false })
		} else {
			None
		}
	}

	#[rustfmt::skip]
	pub fn negated(self) -> Self {
		Self { negation: true, ..self  }
	}
}

#[derive(Debug, Clone)]
struct Disjunction(Vec<Inclusion>);

#[derive(Debug, Clone)]
struct Conjunction(Vec<Disjunction>);
