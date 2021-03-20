mod passport;

use std::env;
use std::fs;

use passport::Passport;

fn main() -> std::io::Result<()> {
    if let Some(filename) = env::args().nth(1) {
        let content = fs::read_to_string(&filename)?;

        let passports: Result<Vec<_>, _> = content
            .split("\n\n")
            .map(Passport::new_from_slice)
            .collect();

        if let Ok(passports) = passports {
            let result1 = passports.iter().filter(|x| x.is_valid()).count();
            println!("There is {} valid passwords", result1);

            let result2 = passports.iter().filter(|x| x.is_strictly_valid()).count();
            println!("There is {} strictly valid passwords", result2);
        } else {
            eprintln!("Couldn't parse the input data.");
            std::process::exit(1);
        }
    }
    Ok(())
}
