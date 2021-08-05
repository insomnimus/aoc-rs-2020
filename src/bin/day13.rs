const DATA: &str = include_str!("day13.txt");

fn part1(current_time: usize, times: &[usize]) -> usize {
	times
		.iter()
		.map(|n| {
			let delta = current_time % *n;
			if delta == 0 {
				(*n, 0)
			} else {
				(*n, *n - delta)
			}
		})
		.reduce(|x, y| if x.1 < y.1 { x } else { y })
		.map(|(id, time)| id * time)
		.unwrap()
}

fn part2(times: &[(usize, usize)]) -> usize {
	times
		.iter()
		.map(|(i, id)| ((id - (i % id)) % id, *id))
		.reduce(|a, b| {
			let mut rem = a.0;
			while rem % b.1 != b.0 {
				rem += a.1;
			}
			(rem, a.1 * b.1)
		})
		.unwrap()
		.0
}

fn main() {
	let mut lines = DATA.lines();
	let current = lines.next().unwrap().parse::<usize>().unwrap();
	let mut times = Vec::new();

	let offset_times: Vec<_> = lines
		.next()
		.unwrap()
		.split(',')
		.enumerate()
		.filter_map(|(i, s)| {
			s.parse::<usize>()
				.map(|n| {
					times.push(n);
					(i, n)
				})
				.ok()
		})
		.collect();

	let p1 = part1(current, &times);
	let p2 = part2(&offset_times);
	println!("day13-part1 = {}\nday13-part2 = {}", p1, p2);
}
