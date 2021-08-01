const DATA: &str = include_str!("day5.txt");
const ROWS: [u8; 128] = gen_rows();
const COLS: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

const fn gen_rows() -> [u8; 128] {
	let mut arr = [0_u8; 128];
	let mut i = 0_u8;
	while i <= 127_u8 {
		arr[i as usize] = i;
		i += 1;
	}
	arr
}

struct Seat {
	row: u8,
	col: u8,
}

impl Seat {
	fn parse(s: &str) -> Self {
		let mut rows = &ROWS[..];
		let mut cols = &COLS[..];
		for c in s[..7].as_bytes() {
			if *c == b'F' {
				rows = &rows[..rows.len() / 2];
			} else if *c == b'B' {
				rows = &rows[rows.len() / 2..];
			} else {
				panic!("unexpected char in first 7 letters");
			}
		}

		for c in s[7..10].as_bytes() {
			if *c == b'L' {
				cols = &cols[..cols.len() / 2];
			} else if *c == b'R' {
				cols = &cols[cols.len() / 2..];
			} else {
				panic!("unexpected char in letters 7-10");
			}
		}

		Self {
			row: rows[0],
			col: cols[0],
		}
	}

	fn id(&self) -> usize {
		self.row as usize * 8 + self.col as usize
	}
}

fn main() {
	let mut max = 0_usize;
	let mut min = usize::MAX;
	let mut total = 0_usize;
	let seats: Vec<_> = DATA.lines().map(Seat::parse).collect();

	for n in seats.iter().map(Seat::id) {
		if n < min {
			min = n;
		}
		if n > max {
			max = n;
		}
		total += n;
	}

	let expected: usize = (min..=max).sum();
	let id = expected - total;
	println!("day5-part1 = {}\nday5-part2 = {}", max, id);
}
