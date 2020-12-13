const REQUIRED_FIELDS: &'static[&'static str] =&["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let contents = std::fs::read_to_string("data.txt").unwrap();

    let valid = contents.split("\n\n").into_iter().filter(|p| is_valid_passport_part_one(p)).count();
    println!("{}", valid);
}

fn is_valid_passport_part_one(passport :&str) -> bool {
    let data_fields = passport.split_whitespace().map(|s| s.split(":").nth(0).unwrap()).collect::<Vec<&str>>();
    REQUIRED_FIELDS.iter().all(|req| data_fields.contains(req))
}