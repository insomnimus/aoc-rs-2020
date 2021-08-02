use std::collections::HashMap;

const DATA: &str = include_str!("day7.txt");

#[derive(PartialEq, PartialOrd, Eq, Hash)]
struct Bag<'a> {
	kind: &'a str,
	colour: &'a str,
}

impl<'a> Bag<'a> {
	const fn new(kind: &'a str, colour: &'a str) -> Self {
		Self { kind, colour }
	}
}

struct Rule<'a> {
	bag: Bag<'a>,
	contains: HashMap<Bag<'a>, u8>,
}

impl<'a> Rule<'a> {
	fn parse(s: &'a str) -> Self {
		let s = match s.strip_suffix('.') {
			Some(new) => new,
			None => s,
		};

		let mut left_right = s.splitn(2, " contain ");
		let mut kind_colour = left_right.next().unwrap().split_whitespace();
		let kind = kind_colour.next().unwrap();
		let colour = kind_colour.next().unwrap();
		let bag = Bag::new(kind, colour);
		let right = left_right.next().unwrap();
		let mut contains = HashMap::new();

		for s in right.split(',') {
			let s = s.trim();
			if s == "no other bags" {
				continue;
			}
			let mut fields = s.split_whitespace();
			let n = fields.next().unwrap().parse::<u8>().unwrap_or_else(|e| {
				panic!("invalid number: {}\n in {}", e, s);
			});
			let kind = fields.next().unwrap();
			let colour = fields.next().unwrap();
			contains.insert(Bag::new(kind, colour), n);
		}

		Self { bag, contains }
	}

	fn can_hold(&self, bag: &Bag<'_>, rules: &[Rule<'_>]) -> bool {
		if self.contains.contains_key(bag) {
			return true;
		}
		for key in self.contains.keys() {
			if let Some(rule) = rules.iter().find(|r| r.bag.eq(key)) {
				if rule.can_hold(bag, rules) {
					return true;
				}
			}
		}

		false
	}

	fn contains_total(&self, rules: &[Rule<'_>]) -> usize {
		self.contains
			.iter()
			.map(|(bag, amount)| {
				rules
					.iter()
					.find(|r| r.bag.eq(bag))
					.map(|r| r.contains_total(rules))
					.unwrap_or(0) * *amount as usize
					+ *amount as usize
			})
			.sum()
	}
}

fn main() {
	const TARGET: Bag<'static> = Bag::new("shiny", "gold");

	let rules: Vec<_> = DATA.lines().map(Rule::parse).collect();

	let part1 = rules.iter().filter(|r| r.can_hold(&TARGET, &rules)).count();

	let part2 = rules
		.iter()
		.find(|r| r.bag.eq(&TARGET))
		.map(|r| r.contains_total(&rules))
		.unwrap_or(0_usize);

	println!("day7-part1 = {}\nday7-part2 = {}", part1, part2);
}
