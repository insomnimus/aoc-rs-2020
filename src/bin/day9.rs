const DATA: &str = include_str!("day9.txt");

struct Cipher(Vec<usize>);

impl Cipher {
	fn part1(&self) -> (usize, usize) {
		self.0
			.iter()
			.copied()
			.enumerate()
			.skip(25)
			.find(|(i, n)| {
				//println!("i: {}\nn: {}", i, n);
				let prev = &self.0[*i - 25..*i];

				!prev[..prev.len() - 1]
					.iter()
					.copied()
					.enumerate()
					.any(|(j, p)| *n > p && prev[j + 1..].contains(&(n - p)))
			})
			.unwrap()
	}

	fn part2(&self, target_index: usize, target: usize) -> usize {
		let left = &self.0[..target_index];

		(0..left.len())
			.map(|i| {
				let mut acc = 0;
				(
					i,
					i + left
						.iter()
						.skip(i)
						.take_while(|n| {
							if acc >= target {
								false
							} else {
								acc += *n;
								true
							}
						})
						.count(),
					acc,
				)
			})
			.find(|(_, _, acc)| *acc == target)
			.map(|(start, end, _)| {
				left[start..end].iter().min().unwrap() + left[start..end].iter().max().unwrap()
			})
			.expect("day9-part2: failed to solve")
	}
}

fn main() {
	let cipher = Cipher(DATA.lines().map(|s| s.parse::<usize>().unwrap()).collect());

	let (index, p1) = cipher.part1();
	let p2 = cipher.part2(index, p1);

	println!("day9-part1 = {}\nday9-part2 = {}", p1, p2);
}
