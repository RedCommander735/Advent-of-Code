use fancy_regex::Regex;
use std::cmp;
use std::env;
use std::fs::read_to_string;

#[derive(PartialEq, PartialOrd, Debug)]
struct Brg {
    blue: u32,
    red: u32,
    green: u32,
}

impl Brg {
    fn product(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input.txt");

    let output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    println!("{}", output);
}

fn parse_file(path: &str) -> u32 {
    let mut all_min_cubes: Vec<u32> = Vec::new();

    for line in read_to_string(&path).unwrap().lines() {
        let line = line.to_string().replace("Game ", "");
        let line: Vec<&str> = line.split(":").collect();
        let min_cubes = parse_line(&line.get(1));
        all_min_cubes.append(&mut vec![min_cubes])
    }
    let sum: u32 = all_min_cubes.iter().sum();
    sum
}

fn parse_line(line: &Option<&&str>) -> u32 {
    let regex_blue: Regex = Regex::new(r"\d+(?= blue)").unwrap();
    let regex_red = Regex::new(r"\d+(?= red)").unwrap();
    let regex_green = Regex::new(r"\d+(?= green)").unwrap();

    match line {
        Some(string) => {
            let mut brg = Brg {
                blue: 0,
                red: 0,
                green: 0,
            };
            let sets: Vec<&str> = string.split(";").collect();
            for set in sets {
                let rb = regex_blue.find(set).unwrap();
                let b = match rb {
                    Some(x) => x.as_str().parse::<u32>().unwrap(),
                    None => 0,
                };
                let rr = regex_red.find(set).unwrap();
                let r = match rr {
                    Some(x) => x.as_str().parse::<u32>().unwrap(),
                    None => 0,
                };
                let rg = regex_green.find(set).unwrap();
                let g = match rg {
                    Some(x) => x.as_str().parse::<u32>().unwrap(),
                    None => 0,
                };

                brg.blue = cmp::max(brg.blue, b);
                brg.red = cmp::max(brg.red, r);
                brg.green = cmp::max(brg.green, g);
            }

            // println!("{:?}", &brg);
            brg.product()
        }
        None => 0,
    }
}
