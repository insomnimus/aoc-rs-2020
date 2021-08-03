const N_ROWS: usize = 93;
const N_COLS: usize = 95;
const DELTAS: [(isize, isize); 8] = [
	(-1, -1),
	(-1, 0),
	(-1, 1),
	(0, -1),
	(0, 1),
	(1, -1),
	(1, 0),
	(1, 1),
];

const DATA: &str = include_str!("day11.txt");

#[derive(Copy, Clone, PartialEq)]
enum Cell {
	Empty,
	Floor,
	Taken,
}

#[derive(Copy, Clone)]
struct Row([Cell; N_COLS]);
#[derive(Copy, Clone)]
struct Grid([Row; N_ROWS]);

impl Cell {
	fn new(c: char) -> Option<Self> {
		match c {
			'L' => Some(Self::Empty),
			'.' => Some(Self::Floor),
			'#' => Some(Self::Taken),
			_ => None,
		}
	}

	fn is_taken(&self) -> bool {
		*self == Self::Taken
	}
}

impl Default for Row {
	fn default() -> Self {
		Self([Cell::Floor; 95])
	}
}

impl Row {
	fn parse(s: &str) -> Self {
		let mut buf = [Cell::Floor; 95];
		for (i, c) in s.chars().enumerate() {
			buf[i] = Cell::new(c).unwrap_or_else(|| panic!("invalid char: {}", c));
		}
		Self(buf)
	}
}
impl Grid {
	fn parse(s: &str) -> Self {
		let mut buf = [Row::default(); 93];
		for (i, r) in s.lines().map(Row::parse).enumerate() {
			buf[i] = r;
		}
		Self(buf)
	}

	fn next_frame(&mut self) -> bool {
		let mut has_changed = false;
		let prev = *self;

		for n_row in 0..N_ROWS {
			for n_col in 0..N_COLS {
				let (changed, new_state) = prev.calc_cell_next(n_row, n_col);
				if changed {
					has_changed = true;
					self.0[n_row].0[n_col] = new_state;
				}
			}
		}
		has_changed
	}

	fn calc_cell_next(&self, n_row: usize, n_col: usize) -> (bool, Cell) {
		let c = self.get_unchecked(n_row, n_col);

		match c {
			Cell::Empty
				if self
					.adjacent(n_row, n_col)
					.iter()
					.all(|cell| !cell.is_taken()) =>
			{
				(true, Cell::Taken)
			}
			Cell::Taken
				if self
					.adjacent(n_row, n_col)
					.iter()
					.filter(|c| c.is_taken())
					.count() >= 4 =>
			{
				(true, Cell::Empty)
			}
			_ => (false, c),
		}
	}

	fn adjacent(&self, n_row: usize, n_col: usize) -> Vec<Cell> {
		DELTAS
			.iter()
			.filter_map(|(delta_row, delta_col)| {
				let r = n_row as isize + delta_row;
				if !(r >= 0 && r < N_ROWS as isize) {
					return None;
				}
				let c = n_col as isize + delta_col;
				if !(c >= 0 && c < N_COLS as isize) {
					None
				} else {
					Some((r as usize, c as usize))
				}
			})
			.map(|(r, c)| self.get_unchecked(r, c))
			.collect()
	}

	fn part1(&mut self) -> usize {
		while self.next_frame() {}

		self.0
			.iter()
			.map(|row| row.0.iter().filter(|c| c.is_taken()).count())
			.sum()
	}

	fn get_unchecked(&self, n_row: usize, n_col: usize) -> Cell {
		self.0[n_row].0[n_col]
	}
}

fn main() {
	let mut grid = Grid::parse(DATA);

	let p1 = grid.part1();

	println!("day11-part1 = {}", p1);
}
