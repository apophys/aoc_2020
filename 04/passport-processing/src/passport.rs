use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Passport<'a>(HashMap<&'a str, &'a str>);

#[derive(Debug)]
pub enum PassportParseError {
    Invalid,
}

impl fmt::Display for PassportParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, ""),
        }
    }
}

impl Error for PassportParseError {}

type PassportResult<T> = Result<T, PassportParseError>;

fn valid_range(value: &str, min: i32, max: i32) -> bool {
    value.parse::<i32>().map_or(false, |value| value >= min && value <= max)
}

impl<'a> Passport<'a> {
    pub fn new_from_slice(source: &'a str) -> PassportResult<Passport<'a>> {
        let mut hashmap = HashMap::new();

        for pair in source.split_whitespace() {
            if let Some((key, value)) = pair.split_once(":") {
                hashmap.insert(key, value);
            } else {
                return Err(PassportParseError::Invalid);
            }
        }
        Ok(Passport(hashmap))
    }

    pub fn is_valid(&self) -> bool {
        self.0.len() == 8 || self.0.len() == 7 && !self.0.contains_key("cid")
    }

    pub fn is_strictly_valid(&self) -> bool {
        if self.is_valid() {
            let validators = [
                self.is_valid_byr(),
                self.is_valid_iyr(),
                self.is_valid_eyr(),
                self.is_valid_hgt(),
                self.is_valid_hcl(),
                self.is_valid_ecl(),
                self.is_valid_pid(),
            ];

            validators.iter().all(|x| *x)
        } else {
            false
        }
    }

    fn valid_year_range(&self, key: &str, min: i32, max: i32) -> bool {
        if let Some(ref field) = self.0.get(key) {
            valid_range(field, min, max)
        } else {
            false
        }
    }

    fn is_valid_byr(&self) -> bool {
        self.valid_year_range("byr", 1920, 2002)
    }

    fn is_valid_iyr(&self) -> bool {
        self.valid_year_range("iyr", 2010, 2020)
    }

    fn is_valid_eyr(&self) -> bool {
        self.valid_year_range("eyr", 2020, 2030)
    }

    fn is_valid_hgt(&self) -> bool {
        if let Some(hgt) = self.0.get("hgt") {
            match hgt.split_at(hgt.len() - 2) {
                (value, "cm") => valid_range(value, 150, 193),
                (value, "in") => valid_range(value, 59, 76),
                (_, _) => false,
            }
        } else {
            false
        }
    }

    fn is_valid_hcl(&self) -> bool {
        if let Some(hcl) = self.0.get("hcl") {
            match hcl.split_at(1) {
                ("#", digits) => {
                    let numeric_digits: Vec<_> =
                        digits.chars().filter(|x| x.is_digit(16)).collect();

                    numeric_digits.len() == 6
                }
                (_, _) => false,
            }
        } else {
            false
        }
    }

    fn is_valid_ecl(&self) -> bool {
        if let Some(ecl) = self.0.get("ecl") {
            matches!(*ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
        } else {
            false
        }
    }

    fn is_valid_pid(&self) -> bool {
        if let Some(pid) = self.0.get("pid") {
            let digits: Vec<_> = pid.chars().filter(|x| x.is_digit(10)).collect();

            pid.len() == 9 && digits.len() == 9
        } else {
            false
        }
    }
}
