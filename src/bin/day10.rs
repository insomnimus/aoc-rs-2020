use std::collections::HashMap;

const DATA: &str = include_str!("day10.txt");

fn part1(sorted: &[usize]) -> usize {
	let acc = match sorted[0] {
		1 => (1, 0),
		3 => (0, 1),
		_ => (0, 0),
	};
	let (ones, threes) = sorted
		.windows(2)
		.fold(acc, |(ones, threes), val| match val[1] - val[0] {
			1 => (ones + 1, threes),
			3 => (ones, threes + 1),
			_ => (ones, threes),
		});
	ones * threes
}

fn part2(sorted: &[usize]) -> usize {
	let mut paths = HashMap::new();
	paths.insert(0_usize, 1_usize); // 1 way to get to 0
	for n in sorted {
		paths.insert(*n, 0);

		for dif in (1..=3).filter(|d| d <= n) {
			if let Some(val) = paths.get(&(*n - dif)).copied() {
				paths.insert(*n, *paths.get(n).unwrap() + val);
			}
		}
	}

	*paths
		.get(sorted.last().unwrap())
		.expect("day10-part2: failed to solve")
}

fn main() {
	let mut nums: Vec<_> = DATA.lines().map(|s| s.parse::<usize>().unwrap()).collect();
	nums.sort_unstable();
	nums.push(nums.last().unwrap() + 3);

	let p1 = part1(&nums);
	let p2 = part2(&nums);

	println!("day10-part1 = {}\nday10-part2 = {}", p1, p2);
}
