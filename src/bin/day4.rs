const DATA: &str = include_str!("day4.txt");

// We are using `Option`here for convenience, so we can use `?` to return easily.
fn is_valid_part2(s: &str) -> Option<()> {
	let mut byr = false;
	let mut iyr = false;
	let mut eyr = false;
	let mut hgt = false;
	let mut hcl = false;
	let mut ecl = false;
	let mut pid = false;

	for f in s.split_whitespace() {
		let mut left_right = f.splitn(2, ':');
		let field_name = left_right.next()?;
		let val = left_right.next()?;

		match field_name {
			"byr" | "iyr" | "eyr" | "pid" => {
				let n = val.parse::<usize>().ok()?;
				match field_name {
					"byr" => {
						if !(1920..=2002).contains(&n) {
							return None;
						}
						byr = true;
					}
					"iyr" => {
						if !(2010..=2020).contains(&n) {
							return None;
						}
						iyr = true;
					}
					"eyr" => {
						if !(2020..=2030).contains(&n) {
							return None;
						}
						eyr = true;
					}
					"pid" => {
						if val.len() != 9 {
							return None;
						}
						pid = true;
					}
					_ => unreachable!(),
				};
			}
			"cid" => (),
			"hgt" => {
				if let Some(x) = val.strip_suffix("cm") {
					let n = x.parse::<usize>().ok()?;
					if !(150..=193).contains(&n) {
						return None;
					}
					hgt = true;
				} else if let Some(x) = val.strip_suffix("in") {
					let n = x.parse::<usize>().ok()?;
					if !(59..=76).contains(&n) {
						return None;
					}
					hgt = true;
				} else {
					return None;
				}
			}
			"ecl" => {
				match val {
					"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
					_ => return None,
				};
				ecl = true;
			}
			"hcl" if val.len() == 7 => {
				if !val.starts_with('#') {
					return None;
				}
				if !val[1..].chars().all(|c| c.is_digit(16)) {
					return None;
				}
				hcl = true;
			}
			_ => return None,
		};
	}

	if byr && iyr && eyr && hgt && hcl && ecl && pid {
		Some(())
	} else {
		None
	}
}

fn is_valid_part1(s: &str) -> bool {
	let mut byr = false;
	let mut iyr = false;
	let mut eyr = false;
	let mut hgt = false;
	let mut hcl = false;
	let mut ecl = false;
	let mut pid = false;
	for f in s.split_whitespace() {
		match f.split(':').next().unwrap() {
			"byr" => byr = true,
			"iyr" => iyr = true,
			"eyr" => eyr = true,
			"hgt" => hgt = true,
			"hcl" => hcl = true,
			"ecl" => ecl = true,
			"pid" => pid = true,
			_ => (),
		};
	}

	byr && iyr && eyr && hgt && hcl && ecl && pid
}

fn main() {
	let mut p1 = 0;
	let mut p2 = 0;
	for s in DATA.split("\n\n") {
		if is_valid_part1(s) {
			p1 += 1;
		}
		if is_valid_part2(s).is_some() {
			p2 += 1;
		}
	}

	println!("day4-part1 = {}\nday4-part2 = {}", p1, p2);
}
