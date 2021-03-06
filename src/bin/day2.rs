const DATA: &str = include_str!("day2.txt");

struct Password<'a> {
	min: usize,
	max: usize,
	letter: char,
	text: &'a str,
}

impl<'a> Password<'a> {
	fn parse(s: &'a str) -> Self {
		let mut fields = s.split_ascii_whitespace();
		let mut min_max = fields.next().unwrap().split('-');
		let min = min_max.next().unwrap().parse::<usize>().unwrap();
		let max = min_max.next().unwrap().parse::<usize>().unwrap();
		let letter = fields.next().unwrap().chars().next().unwrap();
		let text = fields.next().unwrap();
		Self {
			min,
			max,
			letter,
			text,
		}
	}

	fn is_valid_part1(&self) -> bool {
		let n = self.text.chars().filter(|c| *c == self.letter).count();
		self.min as usize <= n && n <= self.max as usize
	}

	fn is_valid_part2(&self) -> bool {
		let chars: Vec<_> = self.text.chars().collect();
		let left = chars.get(self.min - 1);
		let right = chars.get(self.max - 1);
		match (left, right) {
			(None, None) => false,
			(Some(c), None) => *c == self.letter,
			(None, Some(c)) => *c == self.letter,
			(Some(x), Some(y)) => x != y && (*x == self.letter || *y == self.letter),
		}
	}
}

fn main() {
	let mut part1 = 0_usize;
	let mut part2 = 0_usize;
	for p in DATA.lines().map(Password::parse) {
		if p.is_valid_part1() {
			part1 += 1;
		}
		if p.is_valid_part2() {
			part2 += 1;
		}
	}

	println!("day2-part1 = {}\nday2-part2 = {}", part1, part2);
}
