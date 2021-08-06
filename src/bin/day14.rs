use std::collections::HashMap;

const DATA: &str = include_str!("day14.txt");

#[derive(Copy, Clone)]
enum Op {
	Mask(Mask),
	Mem(u64, u64),
}

#[derive(Copy, Clone)]
struct Mask {
	zero: u64,
	one: u64,
	x: u64,
}

struct Machine {
	mask: Mask,
	memory: HashMap<u64, u64>,
}

impl Op {
	fn parse(s: &str) -> Self {
		if s.starts_with("mem") {
			let mut left_right = s.splitn(22, "] = ");
			let addr = left_right.next().unwrap()[4..].parse::<u64>().unwrap();

			let val = left_right.next().unwrap().parse::<u64>().unwrap();

			Self::Mem(addr, val)
		} else {
			Self::Mask(s[7..].chars().fold(
				Mask {
					zero: 0,
					one: 0,
					x: 0,
				},
				|mut acc, c| {
					assert!(
						c == '0' || c == '1' || c == 'X',
						"illegal character in mask"
					);
					acc.zero = (acc.zero << 1) | (c == '0') as u64;
					acc.one = (acc.one << 1) | (c == '1') as u64;
					acc.x = (acc.x << 1) | (c == 'X') as u64;
					acc
				},
			))
		}
	}
}

impl Machine {
	fn new() -> Self {
		Self {
			mask: Mask {
				zero: 0,
				one: 0,
				x: 0,
			},
			memory: HashMap::new(),
		}
	}

	fn exec(&mut self, op: &Op) {
		match op {
			Op::Mask(m) => self.mask = *m,
			Op::Mem(addr, val) => {
				let val = (val & self.mask.x & !self.mask.zero) | self.mask.one;
				self.memory.insert(*addr, val);
			}
		}
	}
}

fn main() {
	let ops: Vec<_> = DATA.lines().map(Op::parse).collect();
	let mut machine = Machine::new();

	for op in &ops {
		machine.exec(op);
	}

	let p1: u64 = machine.memory.values().copied().sum();

	println!("day14-part1 = {}", p1);
}
