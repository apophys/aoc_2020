mod map;

use std::env;
use std::fs;

use crate::map::Map;

#[derive(Debug, Copy, Clone)]
struct SlopeTraversal {
    right: usize,
    down: usize,
}

fn count_toboggan_trees(map: &Map, rules: SlopeTraversal) -> usize {
    let mut slope_index = 0;
    let mut tree_count = 0;

    for (i, row) in map.into_iter().enumerate() {
        if i % rules.down == 0 {
            if row.is_tree_at(slope_index) {
                tree_count += 1;
            }
            slope_index += rules.right;
        }
    }
    tree_count
}

fn main() -> std::io::Result<()> {
    if let Some(filename) = env::args().nth(1) {
        let file_content = fs::read_to_string(&filename)?;

        let rule_list = [
            SlopeTraversal { right: 1, down: 1 },
            SlopeTraversal { right: 3, down: 1 },
            SlopeTraversal { right: 5, down: 1 },
            SlopeTraversal { right: 7, down: 1 },
            SlopeTraversal { right: 1, down: 2 },
        ];

        match file_content.parse() {
            Ok(toboggan_map) => {
                println!(
                    "Task 1: {}",
                    count_toboggan_trees(&toboggan_map, SlopeTraversal { right: 3, down: 1 })
                );

                let task2_result = rule_list.iter().fold(1, |acc, rule| {
                    acc * count_toboggan_trees(&toboggan_map, *rule)
                });
                println!("Task 2: {}", task2_result);
            }
            Err(error) => {
                eprintln!("{}", error);
                eprintln!("Couldn't parse the toboggan map from file {}", filename);
                std::process::exit(1)
            }
        }
        Ok(())
    } else {
        eprintln!("Pass input file as an argument");
        std::process::exit(1)
    }
}
