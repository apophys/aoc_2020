
use std::error::Error;

use std::fs::File;
use std::env;
use std::io::prelude::*;

type AOCResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

fn load_numbers(filename: &str) -> AOCResult<Vec<i32>> {
	let mut file = File::open(filename)?;

	let mut content = String::new();
	file.read_to_string(&mut content)?;

	let mut numbers = Vec::new();

	for line in content.lines() {
		numbers.push(line.parse()?)
	}

	Ok(numbers)
}

fn compute_nice_pair(numbers_list: &[i32], expected_sum: i32) -> Option<(i32, i32)> {
	let mut counter: usize = 0;

	while counter < numbers_list.len() {
		let current = numbers_list[counter];
		for i in &numbers_list[counter..] {
			if current + i == expected_sum {
				eprintln!("Found pair adding up to {}: {} + {}", expected_sum, i, current);
				return Some((current, *i))
			}
		}
		counter+=1;
	}
	None

}

fn compute_nice_triplet(numbers_list: &[i32], expected_sum: i32) -> Option<(i32, i32, i32)> {
	let mut counter: usize = 0;
	while counter < numbers_list.len() {
		let current = numbers_list[counter];

		let mut inner_counter: usize = 0;
		let outer_slice = &numbers_list[counter..];
		while inner_counter < outer_slice.len() {
			let inner_value = outer_slice[inner_counter];
			for i in &outer_slice[inner_counter..] {
				if current + inner_value + i == expected_sum {
					eprintln!("Found three values adding up to {}: {} + {} + {}", expected_sum, i, inner_value, current);
					return Some((current, inner_value, *i))
				}

			}
			inner_counter += 1;
		}
		counter += 1;
	}
	None
}

fn main() -> AOCResult<()>{
	if let Some(filename) = env::args().nth(1) {
		let numbers = load_numbers(&filename)?;

		if let Some(pair) = compute_nice_pair(&numbers, 2020) {
			eprintln!("Result for AoC task 1: {} ", pair.0 * pair.1);
		}

		if let Some(triplet) = compute_nice_triplet(&numbers, 2020) {
			eprintln!("Result for AoC task 1 part 2: {} ", triplet.0 * triplet.1 * triplet.2);
		}

		Ok(())
	}
	else {
		eprintln!("Pass input file as an argument.");
		Ok(())
	}
}
