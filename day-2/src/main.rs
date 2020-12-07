use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Found {} lines", contents.lines().count());

    let valid = contents.lines().into_iter()
        .filter(|s| valid_line_part_one(s))
        .collect::<Vec<&str>>();

    println!("Part one: {}", valid.len());

    let valid = contents.lines().into_iter()
        .filter(|s| valid_line_part_two(s))
        .collect::<Vec<&str>>();
    println!("Part two: {}", valid.len());
}

fn valid_line_part_one(line : &&str) -> bool {
    let parts  = line.split(" ").collect::<Vec<&str>>();

    let min = parts[0].split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
    let max = parts[0].split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

    let char = parts[1].chars().nth(0).unwrap();

    let password = parts[2];

    let num_chars = password.chars().filter(|c| c.eq(&char)).count();

    return num_chars >= min && num_chars <= max;
}

fn valid_line_part_two(line : &&str) -> bool {
    let parts  = line.split(" ").collect::<Vec<&str>>();

    let first = parts[0].split("-").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
    let second = parts[0].split("-").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

    let char = parts[1].chars().nth(0).unwrap();

    let password = parts[2];

    let first_match = password.chars().nth(first-1).unwrap().eq(&char);
    let second_match = password.chars().nth(second-1).unwrap().eq(&char);

    return (first_match || second_match) && !(first_match && second_match);
}