mod map;


use std::fs::File;
use std::env;
use std::io::prelude::*;

use crate::map::Map;


fn count_toboggan_trees(map: Map) -> usize {

	let mut slope_index = 0;
	let mut tree_count = 0;

	for row in map.into_iter() {
		if row.is_tree_at(slope_index) {
			tree_count += 1;
		}
		slope_index += 3;
	}

	tree_count
}


fn main() -> std::io::Result<()> {
	if let Some(filename) = env::args().nth(1) {
		let mut file = File::open(&filename)?;
		let mut file_content = String::new();

		file.read_to_string(&mut file_content)?;

		match file_content.parse::<Map>() {
			Ok(toboggan_map) => println!("{}", count_toboggan_trees(toboggan_map)),
			Err(error) => {
				eprintln!("{}", error);
				eprintln!("Couldn't parse the toboggan map from file {}", filename);
				std::process::exit(1)
			}
		}
		Ok(())
	}
	else {
		eprintln!("Pass input file as an argument");
		std::process::exit(1)
	}
}
