mod boardpass;

use std::env;
use std::fs;

use boardpass::BoardingPass;

fn main() -> std::io::Result<()> {
    if let Some(filename) = env::args().nth(1) {
        let content = fs::read_to_string(&filename)?;

        let boarding_passes: Vec<_> = content.split_whitespace().map(BoardingPass::new).collect();

        if let Some(max_id) = boarding_passes.iter().map(BoardingPass::seat_id).max() {
            println!("Max id: {}", max_id);
        }

        let mut ids: Vec<_> = boarding_passes.iter().map(BoardingPass::seat_id).collect();
        ids.sort_unstable();

        let mut index = 1;
        while index < ids.len() {
            let last = ids[index - 1];
            let current = ids[index];

            if current - last == 2 {
                println!("Gap is on {}", last + 1);
                break;
            }
            index += 1;
        }
    }
    Ok(())
}
