use std::io::Read;
use std::ops::Rem;

fn main() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines().collect::<Vec<&str>>();

    let mut collisions = count_collisions(&lines, 1, 1);
    collisions *= count_collisions(&lines, 3, 1);
    collisions *= count_collisions(&lines, 5, 1);
    collisions *= count_collisions(&lines, 7, 1);
    collisions *= count_collisions(&lines, 1, 2);

    println!("{}", collisions);
}

fn count_collisions(lines: &Vec<&str>, across: usize, down: usize) -> usize {
    let mut count = 0;
    let mut collisions = 0;
    let mut row = 0;
    while lines.get(row).is_some() {
        let line = lines.get(row).unwrap();
        let pos = count * across;
        let index = pos.rem(line.len());

        if line.chars().nth(index).unwrap().eq(&'#') {
            collisions += 1;
        }

        count += 1;
        row += down;
    }
    collisions
}
