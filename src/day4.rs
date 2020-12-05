use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
#[derive(Debug)]
pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    /*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    */
    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HAIR_COLOR_VALID: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref EYE_COLOR_VALID: Regex =
                Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PASSPORT_ID_VALID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        let birth_year_valid = Passport::validate_year(&self.birth_year, 1920, 2002);
        let issue_year_valid = Passport::validate_year(&self.issue_year, 2010, 2020);
        let expiration_year_valid = Passport::validate_year(&self.expiration_year, 2020, 2030);
        let height_valid = Passport::validate_height(&self.height, 150, 193, 59, 76);
        let hair_color_valid = Passport::validate_with_regex(&self.hair_color, &HAIR_COLOR_VALID);
        let eye_color_valid = Passport::validate_with_regex(&self.eye_color, &EYE_COLOR_VALID);
        let passport_id_valid =
            Passport::validate_with_regex(&self.passport_id, &PASSPORT_ID_VALID);

        birth_year_valid
            && issue_year_valid
            && expiration_year_valid
            && height_valid
            && hair_color_valid
            && eye_color_valid
            && passport_id_valid
    }

    fn validate_year(val: &Option<String>, min_year: u32, max_year: u32) -> bool {
        lazy_static! {
            static ref YEAR_VALID: Regex = Regex::new(r"^\d{4}").unwrap();
        }
        if val.is_none() {
            return false;
        }

        let value = val.as_ref().unwrap();

        if !YEAR_VALID.is_match(&value) {
            return false;
        };

        let parsed = value.parse::<u32>().unwrap();
        min_year <= parsed && parsed <= max_year
    }

    fn validate_height(
        val: &Option<String>,
        min_cm: u32,
        max_cm: u32,
        min_in: u32,
        max_in: u32,
    ) -> bool {
        lazy_static! {
            static ref HEIGHT_VALID: Regex = Regex::new(r"^(\d+)(in|cm)").unwrap();
        }
        if val.is_none() {
            return false;
        }
        match HEIGHT_VALID.captures(val.as_ref().unwrap()) {
            Some(caps) => {
                let number = caps
                    .get(1)
                    .map(|i| i.as_str().parse::<u32>().unwrap())
                    .unwrap();
                let unit = caps.get(2).map(|i| i.as_str()).unwrap();
                if unit == "cm" {
                    min_cm <= number && number <= max_cm
                } else if unit == "in" {
                    min_in <= number && number <= max_in
                } else {
                    panic!("What happened?")
                }
            }
            None => false,
        }
    }

    fn validate_with_regex(val: &Option<String>, expr: &Regex) -> bool {
        match val {
            Some(v) => expr.is_match(v),
            None => false,
        }
    }
}

#[aoc_generator(day4)]
pub fn get_values(input: &str) -> Vec<Passport> {
    lazy_static! {
        static ref SPLIT_EXPR: Regex = Regex::new(r"\n\n").unwrap();
        static ref BIRTH_YEAR_EXPR: Regex = Regex::new(r"byr:(\S+)").unwrap();
        static ref ISSUE_YEAR_EXPR: Regex = Regex::new(r"iyr:(\S+)").unwrap();
        static ref EXPIRATION_YEAR_EXPR: Regex = Regex::new(r"eyr:(\S+)").unwrap();
        static ref HEIGHT_EXPR: Regex = Regex::new(r"hgt:(\S+)").unwrap();
        static ref HAIR_COLOR_EXPR: Regex = Regex::new(r"hcl:(\S+)").unwrap();
        static ref EYE_COLOR_EXPR: Regex = Regex::new(r"ecl:(\S+)").unwrap();
        static ref PASSPORT_ID_EXPR: Regex = Regex::new(r"pid:(\S+)").unwrap();
        static ref COUNTRY_ID_EXPR: Regex = Regex::new(r"cid:(\S+)").unwrap();
    }

    SPLIT_EXPR
        .split(input)
        .map(|passport_raw| Passport {
            birth_year: parse(&BIRTH_YEAR_EXPR, &passport_raw),
            issue_year: parse(&ISSUE_YEAR_EXPR, &passport_raw),
            expiration_year: parse(&EXPIRATION_YEAR_EXPR, &passport_raw),
            height: parse(&HEIGHT_EXPR, &passport_raw),
            hair_color: parse(&HAIR_COLOR_EXPR, &passport_raw),
            eye_color: parse(&EYE_COLOR_EXPR, &passport_raw),
            passport_id: parse(&PASSPORT_ID_EXPR, &passport_raw),
            country_id: parse(&COUNTRY_ID_EXPR, &passport_raw),
        })
        .collect::<Vec<Passport>>()
}

fn parse(expr: &Regex, passport_raw: &str) -> Option<String> {
    match expr.captures(passport_raw) {
        Some(val) => val.get(1).map(|i| i.as_str().to_string()),
        None => None,
    }
}

#[aoc(day4, part1)]
pub fn part1(inputs: &[Passport]) -> usize {
    inputs
        .iter()
        .filter(|passport| passport.has_required_fields())
        .collect::<Vec<&Passport>>()
        .len()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &[Passport]) -> usize {
    inputs
        .iter()
        .filter(|passport| passport.is_valid())
        .collect::<Vec<&Passport>>()
        .len()
}
