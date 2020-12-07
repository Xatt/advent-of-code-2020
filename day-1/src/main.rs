use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let ints = contents.lines().into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    find_two(&ints);
    find_three(&ints);
}

fn find_two(ints: &Vec<i32>) {
    let mut count = 0;
    while ints.get(count).is_some() {
        let first = ints.get(count).unwrap();
        ints.iter()
            .skip(count + 1)
            .for_each(|second| if first + second == 2020 {
                println!("{}", first * second);
            });

        count += 1;
    }
}

fn find_three(ints: &Vec<i32>) {
    let mut count = 0;
    while ints.get(count).is_some() {
        let first = ints.get(count).unwrap();
        ints.iter()
            .skip(count + 1)
            .for_each(|second| {
                ints.iter()
                    .skip(count + 2)
                    .for_each(|third| if first + second + third == 2020 {
                        // TODO: Fix this logic, needs to break out of the iter.
                        println!("{}", first * second * third);
                    });
            });

        count += 1;
    }
}
