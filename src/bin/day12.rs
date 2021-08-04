const DATA: &str = include_str!("day12.txt");

#[derive(Copy, Clone)]
enum Action {
	North(isize),
	South(isize),
	East(isize),
	West(isize),
	Forward(isize),
	Left(isize),
	Right(isize),
}

impl Action {
	fn parse(s: &str) -> Self {
		let val = s[1..].parse::<isize>().unwrap();
		match s.chars().next().unwrap() {
			'N' => Self::North(val),
			'S' => Self::South(val),
			'E' => Self::East(val),
			'W' => Self::West(val),
			'F' => Self::Forward(val),
			'L' => Self::Left(val),
			'R' => Self::Right(val),
			ch => panic!("illegal char {}", ch),
		}
	}
}

struct WayPoint {
	x: isize,
	y: isize,
}

impl WayPoint {
	fn rotate(&mut self, mut degrees: isize) {
		while degrees < 0 {
			degrees += 360;
		}
		let x = self.x;
		let y = self.y;
		match degrees {
			0 => (),
			90 => {
				self.y = x;
				self.x = -y;
			}
			180 => {
				self.x = -x;
				self.y = -y;
			}
			270 => {
				self.x = y;
				self.y = -x;
			}
			deg => panic!("invalid degree {}", deg),
		};
	}
}

struct Ship {
	x: isize,
	y: isize,
	// for part 1
	direction: isize,
	// for pat 2
	waypoint: WayPoint,
}

impl Ship {
	fn new() -> Self {
		Self {
			x: 0,
			y: 0,
			direction: 0,
			waypoint: WayPoint { x: 10, y: 1 },
		}
	}

	fn take_action_p1(&mut self, a: Action) {
		match a {
			Action::Forward(n) => self.move_forward_p1(n),
			Action::Left(n) => self.turn(n),
			Action::Right(n) => self.turn(-n),
			Action::North(n) => self.y += n,
			Action::South(n) => self.y -= n,
			Action::East(n) => self.x += n,
			Action::West(n) => self.x -= n,
		};
	}

	fn take_action_p2(&mut self, a: Action) {
		match a {
			Action::Forward(n) => self.move_forward_p2(n),
			Action::Left(n) => self.waypoint.rotate(n),
			Action::Right(n) => self.waypoint.rotate(-n),
			Action::North(n) => self.waypoint.y += n,
			Action::South(n) => self.waypoint.y -= n,
			Action::East(n) => self.waypoint.x += n,
			Action::West(n) => self.waypoint.x -= n,
		};
	}

	fn turn(&mut self, degrees: isize) {
		self.direction = (self.direction + degrees) % 360;
		while self.direction < 0 {
			self.direction += 360;
		}
	}

	fn move_forward_p1(&mut self, n: isize) {
		match self.direction {
			0 => self.x += n,
			90 => self.y += n,
			180 => self.x -= n,
			270 => self.y -= n,
			_ => panic!("invalid direction: {}", self.direction),
		};
	}

	fn move_forward_p2(&mut self, times: isize) {
		self.x += self.waypoint.x * times;
		self.y += self.waypoint.y * times;
	}
}

fn main() {
	let mut ship1 = Ship::new();
	let mut ship2 = Ship::new();
	for a in DATA.lines().map(Action::parse) {
		ship1.take_action_p1(a);
		ship2.take_action_p2(a);
	}

	let p1 = ship1.x.abs() + ship1.y.abs();
	let p2 = ship2.x.abs() + ship2.y.abs();
	println!("day12-part1 = {}\nday12-part2 = {}", p1, p2);
}
