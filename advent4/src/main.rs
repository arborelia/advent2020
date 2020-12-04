use eyre::Result;
mod helpers;
use helpers::read_lines;
use std::collections::HashSet;
use std::fs;
use lazy_static::lazy_static;

lazy_static! {
    static ref REQUIRED_FIELDS: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .cloned()
            .collect();
}

fn check_fields(passport: &str) -> bool {
    let mut fields_seen: HashSet<&str> = HashSet::new();
    for entry in passport.split_whitespace() {
        let field_name: &str = entry.split(":").next().unwrap();
        fields_seen.insert(&field_name);
    }
    fields_seen.is_superset(&REQUIRED_FIELDS)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let passports = input.split("\n\n");
    let mut num_valid: u32 = 0;
    for passport in passports {
        if check_fields(passport) {
            num_valid += 1;
        }
    }
    println!("{}", num_valid);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> String {
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
        ".to_owned()
    }

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
}
