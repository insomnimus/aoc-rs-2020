const DATA: &str = include_str!("day6.txt");

fn n_part1(groups: &[&str]) -> usize {
	let mut buf = [false; 29];

	for g in groups {
		for b in g.as_bytes() {
			buf[*b as usize - 97] = true;
		}
	}

	buf.iter().filter(|b| **b).count()
}

fn n_part2(groups: &[&str]) -> usize {
	let mut buf = [0_u8; 29];
	for g in groups {
		for b in g.as_bytes() {
			buf[*b as usize - 97] += 1;
		}
	}

	buf.iter().filter(|n| **n as usize == groups.len()).count()
}

fn main() {
	let mut part1 = 0_usize;
	let mut part2 = 0_usize;
	for s in DATA.split("\n\n") {
		let groups: Vec<_> = s.split_whitespace().collect();
		part1 += n_part1(&groups);
		part2 += n_part2(&groups);
	}

	println!("day6-part1 = {}\nday6-part2 = {}", part1, part2);
}
