fn main() {
    let contents = std::fs::read_to_string("data.txt").unwrap();

    let valid = contents.split("\n\n").into_iter().map(|p| p.replace(" ", "\n")).filter(|p| is_valid_passport(p)).count();
    println!("{}", valid);
}

fn is_valid_passport(passport :&str) -> bool {
    let data_fields = passport.lines().map(|s| s.split(":").nth(0).unwrap()).collect::<Vec<&str>>();
    let required_fields:Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_fields.iter().all(|req| data_fields.contains(req))
}