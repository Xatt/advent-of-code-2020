use std::io::Read;
use std::ops::Rem;

fn main() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines().collect::<Vec<&str>>();

    let mut count = 0;
    let mut collisions = 0;
    while lines.get(count).is_some() {
        let line = lines.get(count).unwrap();
        let pos = count * 3;
        let index = pos.rem(line.len());

        if line.chars().nth(index).unwrap().eq(&'#') {
            collisions += 1;
        }

        count += 1;
    }

    println!("{}", collisions);
}
