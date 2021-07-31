const DATA: &str = include_str!("day3.txt");

struct LineBuf([char; 31]);

impl LineBuf {
	fn parse(s: &str) -> Self {
		let mut buf: [char; 31] = ['.'; 31];
		for (i, c) in s.chars().enumerate().take(31) {
			buf[i] = c;
		}
		Self(buf)
	}

	fn nth(&self, n: usize) -> char {
		self.0[n % 31]
	}
}

struct Slope {
	right: usize,
	down: usize,
}

impl Slope {
	fn new(right: usize, down: usize) -> Self {
		Self { down, right }
	}

	fn n_trees(&self, map: &[LineBuf]) -> usize {
		let mut n = 0_usize;
		let mut x = 0_usize;
		for (i, ln) in map.iter().enumerate() {
			if i % self.down == 0 {
				if ln.nth(x) == '#' {
					n += 1;
				}
				x += self.right;
			}
		}
		n
	}
}

fn main() {
	let part1 = Slope::new(3, 1);
	let part2_1 = Slope::new(1, 1);
	let part2_3 = Slope::new(5, 1);
	let part2_4 = Slope::new(7, 1);
	let part2_5 = Slope::new(1, 2);

	let map: Vec<_> = DATA.lines().map(LineBuf::parse).collect();
	let p1 = part1.n_trees(&map);
	let p2 = p1
		* part2_1.n_trees(&map)
		* part2_3.n_trees(&map)
		* part2_4.n_trees(&map)
		* part2_5.n_trees(&map);

	println!("day3-part1 = {}\nday3-part2 = {}", p1, p2);
}
