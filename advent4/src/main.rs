use eyre::Result;
use std::collections::{HashSet, HashMap};
use std::fs;
use scan_fmt::scan_fmt;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REQUIRED_FIELDS: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .cloned()
            .collect();
    static ref EYE_COLORS: HashSet<&'static str> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .cloned()
            .collect();
}

pub fn check_fields(passport: &str) -> bool {
    let mut fields_seen: HashSet<&str> = HashSet::new();
    for entry in passport.split_whitespace() {
        let field_name: &str = entry.split(":").next().unwrap();
        fields_seen.insert(&field_name);
    }
    fields_seen.is_superset(&REQUIRED_FIELDS)
}

pub fn check_passport(passport: &str) -> bool {
    if !check_fields(passport) {
        return false;
    }
    let mut fields: HashMap<String, String> = HashMap::new();
    for entry in passport.split_whitespace() {
        if let Ok((name, value)) = scan_fmt!(entry, "{}:{}", String, String) {
            fields.insert(name, value);
        } else {
            println!("Invalid entry: {}", entry);
            return false
        }
    }

    is_number_in_range(&fields["byr"], 1920, 2002)
    && is_number_in_range(&fields["iyr"], 2010, 2020)
    && is_number_in_range(&fields["eyr"], 2020, 2030)
    && check_height(&fields["hgt"])
    && check_color(&fields["hcl"])
    && check_eye_color(&fields["ecl"])
    && check_9digits(&fields["pid"])
}

fn check_height(strvalue: &str) -> bool {
    let height_re = Regex::new(r"^([0-9]+)(in|cm)$").unwrap();
    if let Some(caps) = height_re.captures(strvalue) {
        if &caps[2] == "cm" {
            is_number_in_range(&caps[1], 150, 193)
        } else if &caps[2] == "in" {
            is_number_in_range(&caps[1], 59, 76)
        } else {
            false
        }
    } else {
        false
    }
}

fn check_eye_color(strvalue: &str) -> bool {
    EYE_COLORS.contains(strvalue)
}

fn check_color(strvalue: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(strvalue)
}

fn check_9digits(strvalue: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(strvalue)
}

fn is_number_in_range(strvalue: &str, min: i32, max: i32) -> bool {
    if let Ok(num) = strvalue.parse::<i32>() {
        num >= min && num <= max
    } else {
        false
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let passports = input.split("\n\n");
    let mut num_valid: u32 = 0;
    for passport in passports {
        if check_passport(passport) {
            num_valid += 1;
            println!("VALID\n{}\n", passport);
        } else {
            println!("INVALID\n{}\n", passport);
        }
    }
    println!("{}", num_valid);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() -> Result<()> {
        let ex1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm";
        assert!(check_fields(ex1));
        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let ex2: &str = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929";
        assert!(!check_fields(ex2));
        Ok(())
    }

    #[test]
    fn test_example3() -> Result<()> {
        let ex3: &str = "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm";
        assert!(check_fields(ex3));
        Ok(())
    }

    #[test]
    fn test_example4() -> Result<()> {
        let ex4: &str = "hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";
        assert!(!check_fields(ex4));
        Ok(())
    }

    #[test]
    fn test_valid1() -> Result<()> {
        let ex = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f";
        assert!(check_passport(ex));
        Ok(())
    }

    #[test]
    fn test_valid2() -> Result<()> {
        let ex = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        
        // check individual parts
        assert!(check_fields(ex));
        assert!(is_number_in_range("2010", 2010, 2020));
        assert!(check_height("158cm"));
        assert!(check_color("#b6652a"));
        assert!(check_9digits("093154719"));

        assert!(check_passport(ex));
        Ok(())
    }

    #[test]
    fn test_valid3() -> Result<()> {
        let ex = "eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        assert!(check_passport(ex));
        Ok(())
    }

    #[test]
    fn test_valid4() -> Result<()> {
        let ex = "hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022";
        assert!(check_passport(ex));
        Ok(())
    }

    #[test]
    fn test_invalid1() -> Result<()> {
        let ex = "eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        assert!(!check_passport(ex));
        Ok(())
    }

    #[test]
    fn test_invalid2() -> Result<()> {
        let ex = "iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946";
        assert!(!check_passport(ex));
        Ok(())
    }
    #[test]
    fn test_invalid3() -> Result<()> {
        let ex = "hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        assert!(!check_passport(ex));
        Ok(())
    }
    #[test]
    fn test_invalid4() -> Result<()> {
        let ex = "hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007";
        assert!(!check_passport(ex));
        assert!(!check_height("59cm"));
        assert!(!check_eye_color("zzz"));
        assert!(!is_number_in_range("2038", 2020, 2030));
        Ok(())
    }

    #[test]
    fn test_9digits() -> Result<()> {
        assert!(!check_9digits("3556412378"));
        assert!(check_9digits("355641237"));
        assert!(!check_9digits("35564128"));
        Ok(())
    }

    #[test]
    fn test_hair_color() -> Result<()> {
        assert!(check_color("#ffffff"));
        assert!(!check_color("#fffff"));
        assert!(!check_color("#fffffff"));
        assert!(!check_color("fffffff"));
        Ok(())
    }

    #[test]
    fn test_eye_color() -> Result<()> {
        assert!(check_eye_color("hzl"));
        assert!(!check_eye_color("oof"));
        Ok(())
    }

    #[test]
    fn test_height() -> Result<()> {
        assert!(!check_height("59cm"));
        assert!(check_height("59in"));
        assert!(!check_height("59ink"));
        Ok(())
    }

}
