use std::{fs::read_to_string, path::Path};

use regex::Regex;

use super::{input_folder, Day};

#[derive(Default)]
pub struct Day04 {
    input: Vec<Passport>,
}

// apparently for this problem size a vector is faster than a hashmap :O
type Passport = Vec<(String, String)>;

impl Day04 {
    const BLANK_LINE: &'static str = "\n\n";
    const SEPARATOR: &'static str = ":";

    fn parse_input(s: &str) -> Vec<Passport> {
        s.split(Self::BLANK_LINE)
            .map(Self::parse_passport)
            .collect()
    }

    fn parse_passport(s: &str) -> Passport {
        let mut passport = s
            .split_ascii_whitespace()
            .map(|entry| {
                let pair = entry.split(Self::SEPARATOR).collect::<Vec<_>>();
                (pair[0].to_string(), pair[1].to_string())
            })
            .collect::<Vec<_>>();
        passport.sort();
        passport
    }

    fn count_valid<V>(&self) -> usize
    where
        V: PassportValidator,
    {
        self.input
            .iter()
            .filter(|&passport| V::is_valid(passport))
            .count()
    }
}

impl Day for Day04 {
    fn load_input(&mut self) {
        let path = Path::new(&input_folder()).join("day_04");
        let content = read_to_string(path).expect("Load input failed");
        self.input = Day04::parse_input(&content);
    }

    fn first_challenge(&self) -> String {
        self.count_valid::<OldValidator>().to_string()
    }

    fn second_challenge(&self) -> String {
        self.count_valid::<NewValidator>().to_string()
    }
}

trait PassportValidator {
    fn is_valid(passport: &Passport) -> bool;
}

struct OldValidator;

impl OldValidator {
    const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
}

impl PassportValidator for OldValidator {
    fn is_valid(passport: &Passport) -> bool {
        Self::REQUIRED_FIELDS
            .iter()
            .all(|&key| passport.iter().any(|(pkey, _)| pkey.as_str() == key))
    }
}

struct NewValidator;

impl NewValidator {
    const FIELD_VALIDATORS: [&'static dyn FieldValidator; 7] = [
        &YearValidator {
            key: "byr",
            min: 1920,
            max: 2002,
        },
        &YearValidator {
            key: "iyr",
            min: 2010,
            max: 2020,
        },
        &YearValidator {
            key: "eyr",
            min: 2020,
            max: 2030,
        },
        &HeightValidator,
        &HairValidator,
        &EyeValidator,
        &PassportIdValidator,
    ];
}

impl PassportValidator for NewValidator {
    fn is_valid(passport: &Passport) -> bool {
        Self::FIELD_VALIDATORS.iter().all(|&validator| {
            passport
                .iter()
                .find(|(pkey, _)| pkey.as_str() == validator.key())
                .map(|(_, field)| validator.is_valid(field))
                .unwrap_or_default()
        })
    }
}

trait FieldValidator {
    fn key(&self) -> &str;
    fn is_valid(&self, field: &str) -> bool;
}

struct YearValidator {
    key: &'static str,
    min: i32,
    max: i32,
}

impl FieldValidator for YearValidator {
    fn key(&self) -> &str {
        self.key
    }

    fn is_valid(&self, field: &str) -> bool {
        lazy_static! {
            static ref YEAR_REGEX: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        YEAR_REGEX.is_match(field) && (self.min..=self.max).contains(&field.parse::<i32>().unwrap())
    }
}

struct HeightValidator;

impl FieldValidator for HeightValidator {
    fn key(&self) -> &str {
        "hgt"
    }

    fn is_valid(&self, field: &str) -> bool {
        lazy_static! {
            static ref HEIGHT_REGEX: Regex =
                Regex::new(r"^(?P<num>\d+)(?P<unit>(cm)|(in))$").unwrap();
        }

        HEIGHT_REGEX
            .captures(field)
            .map(|c| {
                let number = c.name("num").map(|m| m.as_str().parse::<i32>().unwrap());
                let unit = c.name("unit").map(|m| m.as_str());
                number.zip(unit)
            })
            .flatten()
            .map(|(number, unit)| {
                let range = match unit {
                    "cm" => 150..=193,
                    "in" => 59..=76,
                    _ => panic!("unknown unit!"),
                };

                range.contains(&number)
            })
            .unwrap_or_default()
    }
}

struct HairValidator;

impl FieldValidator for HairValidator {
    fn key(&self) -> &str {
        "hcl"
    }

    fn is_valid(&self, field: &str) -> bool {
        lazy_static! {
            static ref HAIR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        HAIR_REGEX.is_match(field)
    }
}

struct EyeValidator;

impl FieldValidator for EyeValidator {
    fn key(&self) -> &str {
        "ecl"
    }

    fn is_valid(&self, field: &str) -> bool {
        lazy_static! {
            static ref VALID_COLORS: [&'static str; 7] =
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        }

        VALID_COLORS.contains(&field)
    }
}

struct PassportIdValidator;

impl FieldValidator for PassportIdValidator {
    fn key(&self) -> &str {
        "pid"
    }

    fn is_valid(&self, field: &str) -> bool {
        lazy_static! {
            static ref PASSPORT_ID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        PASSPORT_ID_REGEX.is_match(field)
    }
}

/* tests */

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_first_challenge() {
        let mut day = Day04::default();
        day.input = Day04::parse_input(INPUT);
        assert_eq!(day.first_challenge(), "2");
    }

    #[test]
    fn test_second_challenge() {
        let mut day = Day04::default();

        day.input = Day04::parse_input(
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:20076",
        );
        assert_eq!(day.second_challenge(), "0");

        day.input = Day04::parse_input(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );
        assert_eq!(day.second_challenge(), "4");
    }

    #[test]
    fn test_valid_year() {
        let ref validator = YearValidator {
            key: "year",
            min: 1920,
            max: 2020,
        };
        assert_eq!(validator.is_valid("2002"), true);
        assert_eq!(validator.is_valid("2021"), false);
    }

    #[test]
    fn test_valid_height() {
        let ref validator = HeightValidator;
        assert_eq!(validator.is_valid("60in"), true);
        assert_eq!(validator.is_valid("190cm"), true);
        assert_eq!(validator.is_valid("190in"), false);
        assert_eq!(validator.is_valid("190"), false);
    }

    #[test]
    fn test_valid_hair() {
        let ref validator = HairValidator;
        assert_eq!(validator.is_valid("#123abc"), true);
        assert_eq!(validator.is_valid("#123abz"), false);
        assert_eq!(validator.is_valid("123abc"), false);
    }

    #[test]
    fn test_valid_eye() {
        let ref validator = EyeValidator;
        assert_eq!(validator.is_valid("brn"), true);
        assert_eq!(validator.is_valid("wat"), false);
    }

    #[test]
    fn test_valid_passport_id() {
        let ref validator = PassportIdValidator;
        assert_eq!(validator.is_valid("000000001"), true);
        assert_eq!(validator.is_valid("0123456789"), false);
    }
}
