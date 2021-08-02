const DATA: &str = include_str!("day8.txt");

enum Op {
	Acc(isize),
	Jmp(isize),
	Nop(isize),
}

struct Code {
	op: Op,
	executed: bool,
}

impl Op {
	fn parse(s: &str) -> Self {
		let mut fields = s.splitn(2, ' ');
		let op = fields.next().unwrap();
		let n = fields.next().unwrap().parse::<isize>().unwrap();

		match op {
			"acc" => Self::Acc(n),
			"jmp" => Self::Jmp(n),
			"nop" => Self::Nop(n),
			_ => panic!("unknown instruction: {}", s),
		}
	}

	fn swap(&mut self) {
		match self {
			Self::Jmp(n) => *self = Self::Nop(*n),
			Self::Nop(n) => *self = Self::Jmp(*n),
			_ => (),
		};
	}
}

impl Code {
	fn new(op: Op) -> Self {
		Self {
			op,
			executed: false,
		}
	}

	fn swap(&mut self) {
		self.op.swap();
	}
}

struct Machine {
	addr: isize,
	acc: isize,
	lines: Vec<Code>,
}

impl Machine {
	fn new(instructions: &str) -> Self {
		let lines: Vec<_> = instructions
			.lines()
			.map(|s| Code::new(Op::parse(s)))
			.collect();
		Self {
			addr: 0,
			acc: 0,
			lines,
		}
	}

	fn haults(&mut self) -> bool {
		while let Some(code) = self.lines.get_mut(self.addr as usize) {
			if code.executed {
				return false;
			}
			code.executed = true;
			match code.op {
				Op::Nop(_) => self.addr += 1,
				Op::Jmp(n) => self.addr += n,
				Op::Acc(n) => {
					self.addr += 1;
					self.acc += n;
				}
			}
		}
		true
	}

	fn fix(&mut self) {
		let mut i = 0;
		let mut i_prev = 0;

		while !self.haults() {
			self.reset();
			self.lines[i].swap();
			self.lines[i_prev].swap();
			i_prev = i;
			i += 1;
			if i == self.lines.len() {
				panic!("couldn't solve day 8");
			}
		}
	}

	fn reset(&mut self) {
		self.acc = 0;
		self.addr = 0;
		for mut code in self.lines.iter_mut() {
			code.executed = false;
		}
	}
}

fn main() {
	let mut machine = Machine::new(DATA);
	machine.haults();
	let p1 = machine.acc;

	machine.reset();
	machine.fix();
	let p2 = machine.acc;

	println!("day8-part1 = {}\nday8-part2 = {}", p1, p2);
}
