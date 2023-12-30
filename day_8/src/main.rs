mod part_1;
mod part_2;
use std::env;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input");

    let output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => {
            let part_1 = part_1::part_1(s);
            let part_2 = part_2::part_2(s);
            (part_1, part_2)
        }
    };

    println!("{:?}", output);
}
