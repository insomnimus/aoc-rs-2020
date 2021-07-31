const DATA: &str = include_str!("day1.txt");

fn calc_part1(nums: &[usize]) -> usize {
	find_pair(2020, nums)
		.map(|(n1, n2)| n1 * n2)
		.expect("day1-part1: failed to find an answer")
}

fn calc_part2(nums: &[usize]) -> usize {
	for (i, n1) in nums.iter().enumerate() {
		if let Some((n2, n3)) = find_pair(2020 - *n1, &nums[i + 1..]) {
			return *n1 * n2 * n3;
		}
	}
	panic!("day1-part2: failed to find an answer");
}

fn find_pair(sum: usize, nums: &[usize]) -> Option<(usize, usize)> {
	for (i, n1) in nums.iter().enumerate() {
		if let Some(n2) = nums.iter().skip(i).find(|x| *x + *n1 == sum) {
			return Some((*n1, *n2));
		}
	}
	None
}

fn main() {
	let nums: Vec<_> = DATA
		.split('\n')
		.map(|s| {
			s.parse::<usize>()
				.expect("the file contains non-usize lines")
		})
		.collect();

	println!(
		"day1-part1 = {}\nday1-part2 = {}",
		calc_part1(&nums),
		calc_part2(&nums),
	)
}
