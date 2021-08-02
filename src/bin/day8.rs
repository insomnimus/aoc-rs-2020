const DATA: &str = include_str!("day8.txt");

#[derive(PartialEq, Clone, Copy)]
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
}

impl Code {
	fn new(op: Op) -> Self {
		Self {
			op,
			executed: false,
		}
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

	fn has_infinite_loop(&mut self) -> bool {
		while let Some(code) = self.lines.get_mut(self.addr as usize) {
			if code.executed {
				return true;
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
		false
	}
}

fn main() {
	let mut machine = Machine::new(DATA);
	machine.has_infinite_loop();
	let p1 = machine.acc;

	println!("day8-part1 = {}", p1);
}
