use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Rucksack {
    first_part: String,
    second_part: String,
}

fn priorities() -> HashMap<String, i32> {
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut priorities: HashMap<String, i32> = HashMap::new();

    characters.chars().enumerate().for_each(|(i, c)| {
        priorities.insert(c.to_string(), (i + 1) as i32);
    });

    priorities
}

fn get_rucksacks(lines: Vec<&str>) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let (first, second) = line.split_at(line.chars().count() / 2);

        let first_part = first.to_string();
        let second_part = second.to_string();

        rucksacks.push(Rucksack {
            first_part,
            second_part,
        });
    }

    rucksacks
}

fn matching_characters(a: &String, b: &String) -> String {
    let mut matching: String = String::from("");

    a.chars().for_each(|character| match character {
        _ if b.contains(character) && !matching.contains(character) => {
            matching.push_str(&character.to_string())
        }
        _ => (),
    });

    matching
}

fn main() {
    let contents = fs::read_to_string("./input/rucksacks").expect("Could not read file!");
    let lines: Vec<&str> = contents.split("\n").collect();

    let rucksacks = get_rucksacks(lines);
    let priorities = priorities();

    let total_prio: i32 = rucksacks
        .iter()
        .map(|r| matching_characters(&r.first_part, &r.second_part))
        .fold(0, |acc, curr| {
            let mut total = acc;

            curr.chars().for_each(|character| {
                let priority = match priorities.get(&character.to_string()) {
                    Some(&kek) => kek,
                    _ => panic!("No priority for {}", &character.to_string()),
                };

                total = total + priority
            });

            total
        });

    println!("{}", total_prio);
}
