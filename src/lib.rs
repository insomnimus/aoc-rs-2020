use std::{
	error::Error,
	fs::File,
	io::{
		self,
		BufRead,
	},
};

pub fn read_ints(p: &str) -> Result<Vec<i64>, Box<dyn Error>> {
	let f = File::open(p)?;
	io::BufReader::new(f)
		.lines()
		.filter_map(Result::ok)
		.map(|s| s.parse::<i64>())
		.collect::<Result<_, _>>()
		.map_err(|e| e.into())
}
