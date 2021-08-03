//#![feature(const_fn_fn_ptr_basics)]

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

type Neighbours = [Option<(usize, usize)>; 8];

#[derive(Copy, Clone, PartialEq)]
enum State {
	Empty,
	Floor,
	Taken,
}

#[derive(Copy, Clone)]
struct Cell {
	state: State,
	neighbours: Neighbours,
}

#[derive(Clone, Copy)]
struct Row([Cell; N_COLS]);
#[derive(Clone, Copy)]
struct Grid([Row; N_ROWS]);

impl State {
	fn new(c: u8) -> Self {
		match c {
			b'L' => Self::Empty,
			b'.' => Self::Floor,
			b'#' => Self::Taken,
			_ => panic!("invalid char for state: {}", c),
		}
	}

	fn is_taken(&self) -> bool {
		*self == Self::Taken
	}
}

impl Cell {
	fn new(n_row: usize, n_col: usize, state: State) -> Self {
		let mut neighbours = [None; 8];
		let mut i = 0;
		while i < DELTAS.len() {
			let (delta_row, delta_col) = DELTAS[i];
			let r = n_row as isize + delta_row;
			if r >= 0 && r < N_ROWS as isize {
				let c = n_col as isize + delta_col;
				if c >= 0 && c < N_COLS as isize {
					neighbours[i] = Some((r as usize, c as usize));
				}
			}
			i += 1;
		}

		Self { state, neighbours }
	}
}

impl Default for Row {
	fn default() -> Self {
		Self([Cell::default(); N_COLS])
	}
}

impl Default for Cell {
	fn default() -> Self {
		Self {
			state: State::Floor,
			neighbours: [None; 8],
		}
	}
}

impl Grid {
	fn parse(s: &str) -> Self {
		let mut buf = [Row::default(); N_ROWS];

		for (n_row, row) in s.lines().enumerate() {
			for (n_col, c) in row.as_bytes().iter().enumerate() {
				buf[n_row].0[n_col] = Cell::new(n_row, n_col, State::new(*c));
			}
		}

		Self(buf)
	}

	fn next_frame_p1(&mut self) -> bool {
		let mut has_changed = false;
		let prev = *self;

		for n_row in 0..N_ROWS {
			for n_col in 0..N_COLS {
				let (changed, new_state) = prev.calc_cell_p1(n_row, n_col);
				if changed {
					has_changed = true;
					self.0[n_row].0[n_col].state = new_state;
				}
			}
		}

		has_changed
	}

	fn calc_cell_p1(&self, n_row: usize, n_col: usize) -> (bool, State) {
		let c = self.get_unchecked(n_row, n_col);

		match c.state {
			State::Empty if self.states(&c.neighbours).iter().all(|s| !s.is_taken()) => {
				(true, State::Taken)
			}
			State::Taken
				if self
					.states(&c.neighbours)
					.iter()
					.filter(|s| s.is_taken())
					.count() >= 4 =>
			{
				(true, State::Empty)
			}
			_ => (false, c.state),
		}
	}

	fn states(&self, indices: &[Option<(usize, usize)>]) -> Vec<State> {
		indices
			.iter()
			.filter_map(|o| o.map(|(n_row, n_col)| self.get_unchecked(n_row, n_col).state))
			.collect()
	}

	fn get_unchecked(&self, n_row: usize, n_col: usize) -> Cell {
		self.0[n_row].0[n_col]
	}

	fn part1(&mut self) -> usize {
		while self.next_frame_p1() {}

		self.0
			.iter()
			.map(|row| row.0.iter().filter(|c| c.state.is_taken()).count())
			.sum()
	}
}

fn main() {
	let mut grid = Grid::parse(DATA);

	let p1 = grid.part1();

	println!("day11-part1 = {}", p1);
}
