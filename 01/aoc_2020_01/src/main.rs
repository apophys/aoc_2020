// devel time yay
#![allow(unused_imports,dead_code,unused_variables,unused_mut)]

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

fn compute_nice_pair(numbers_list: Vec<i32>, expected_sum: i32) -> Option<(i32, i32)> {
	let mut counter: usize = 0;

	while counter < numbers_list.len() {
		let current = numbers_list[counter];
		for i in &numbers_list[counter..] {
			if current + i == expected_sum {
				eprintln!("Found pair summing up to {}: {} + {}", expected_sum, i, current);
				return Some((current, *i))
			}
		}
		counter+=1;
	}
	None

}

fn main() -> AOCResult<()>{
	if let Some(filename) = env::args().skip(1).next() {
		let numbers = load_numbers(&filename)?;

		if let Some(pair) = compute_nice_pair(numbers, 2020) {
			eprintln!("Result for AoC task 1: {} ", pair.0 * pair.1);
		}

		Ok(())
	}
	else {
		eprintln!("Pass input file as an argument.");
		Ok(())
	}
}
