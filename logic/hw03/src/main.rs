fn main() {
	let args: Vec<String> = std::env::args().collect();
	let path = &args[1];
	let input = std::fs::read_to_string(path).unwrap();
	let input = input.trim();

	let graph = Graph::from(input);
	let result = graph.to_dimacs_cnf();

	print!("{result}");
	println!();
}

type Number = u32;

#[derive(Debug)]
struct Graph {
	node_count: Number,
	_edge_count: Number,
	edges: Vec<Edge>,
}

#[derive(Debug, PartialEq)]
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
		let _edge_count = parse_str_opt(first_line.next());

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
			_edge_count,
			edges,
		}
	}
}

impl Graph {
	pub fn to_formula(&self) -> Vec<Disjunction> {
		[
			self.in_min_1_group(),
			self.in_max_1_group(),
			self.know_each_other(),
			self.distinct(),
		]
		.concat()
	}

	pub fn to_dimacs_cnf(&self) -> String {
		let formula = self.to_formula();

		// headers
		let var_count = 1000000.to_string();
		let clause_count = formula.len().to_string();
		let mut string = format!("p cnf {var_count} {clause_count}\n");

		for disjunction in formula {
			for inclusion in disjunction.0 {
				let Inclusion { x, y, z, negation } = inclusion;

				if negation {
					string.push('-');
				}

				// assuming nodes < 100 for positional formatting: xxyyzz
				let x = x * 10000;
				let y = y * 100;
				let inclusion_sum = x + y + z;

				string += &format!("{:06} ", inclusion_sum);
			}

			string += "0\n";
		}

		string
	}

	fn in_min_1_group(&self) -> Vec<Disjunction> {
		let mut disjunctions = vec![];

		for x in 1..=self.node_count {
			let mut inclusions = vec![];

			for y in 1..=self.node_count {
				for z in (y + 1)..=self.node_count {
					if pairwise_unequal(x, y, z) {
						inclusions.push(Inclusion::new(x, y, z));
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
				for z1 in (y1 + 1)..=self.node_count {
					for y2 in y1..=self.node_count {
						for z2 in (y2 + 1)..=self.node_count {
							if y1 != y2
								&& z1 != z2 && pairwise_unequal(x, y1, z1)
								&& pairwise_unequal(x, y2, z2)
							{
								let a1 = Inclusion::new(x, y1, z1).negated();
								let a2 = Inclusion::new(x, y2, z2).negated();

								disjunctions.push(Disjunction(vec![a1, a2]));
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

		for x in 1..=self.node_count {
			for y in (x + 1)..=self.node_count {
				if !self.edges.contains(&Edge(x, y)) && !self.edges.contains(&Edge(y, x)) {
					for z in 1..=self.node_count {
						if pairwise_unequal(x, y, z) {
							disjunctions.push(Disjunction(vec![Inclusion::new(x, y, z).negated()]));
						}
					}
				}
			}
		}

		disjunctions
	}

	fn distinct(&self) -> Vec<Disjunction> {
		let mut clauses = vec![];

		for x in 1..=self.node_count {
			for y in (x + 1)..=self.node_count {
				for z in (y + 1)..=self.node_count {
					if pairwise_unequal(x, y, z) {
						let clause = [
							Self::xnor((x, y, z), (x, z, y)),
							Self::xnor((x, z, y), (y, x, z)),
							Self::xnor((y, x, z), (y, z, x)),
							Self::xnor((y, z, x), (z, x, y)),
							Self::xnor((z, x, y), (z, y, x)),
						]
						.concat();

						clauses.push(clause);
					}
				}
			}
		}
		clauses.concat()
	}

	fn xnor(inc1: (Number, Number, Number), inc2: (Number, Number, Number)) -> Vec<Disjunction> {
		let (x1, y1, z1) = inc1;
		let (x2, y2, z2) = inc2;

		vec![
			Disjunction(vec![
				Inclusion::new(x1, y1, z1),
				Inclusion::new(x2, y2, z2).negated(),
			]),
			Disjunction(vec![
				Inclusion::new(x1, y1, z1).negated(),
				Inclusion::new(x2, y2, z2),
			]),
		]
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

fn pairwise_unequal(x: Number, y: Number, z: Number) -> bool {
	x != y && y != z && x != z
}

impl Inclusion {
	#[rustfmt::skip]
	pub fn new(x: Number, y: Number, z: Number) -> Self {
		Self {x, y, z, negation: false }
	}

	#[rustfmt::skip]
	pub fn negated(self) -> Self {
		Self { negation: true, ..self  }
	}
}

#[derive(Debug, Clone)]
struct Disjunction(Vec<Inclusion>);
