use std::fs;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

struct Move {
    me: Hand,
    opponent: Hand,
}

fn hand_score(hand: &Hand) -> i32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

#[rustfmt::skip]
fn result_score(a_move: &Move) -> i32 {
    match a_move {
        Move { me: Hand::Rock, opponent: Hand::Paper } => 0,
        Move { me: Hand::Rock, opponent: Hand::Rock } => 3,
        Move { me: Hand::Rock, opponent: Hand::Scissors } => 6,

        Move { me: Hand::Paper, opponent: Hand::Paper } => 3,
        Move { me: Hand::Paper, opponent: Hand::Rock } => 6,
        Move { me: Hand::Paper, opponent: Hand::Scissors } => 0,

        Move { me: Hand::Scissors, opponent: Hand::Paper } => 6,
        Move { me: Hand::Scissors, opponent: Hand::Rock } => 0,
        Move { me: Hand::Scissors, opponent: Hand::Scissors } => 3
    }
}

fn hand_by_value(value: &str) -> Hand {
    match value {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,

        _ => panic!("What?!"),
    }
}

fn get_moves(lines: Vec<&str>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let line: Vec<&str> = line.split(" ").collect();

        let me = hand_by_value(line[1]);
        let opponent = hand_by_value(line[0]);

        moves.push(Move { me, opponent })
    }

    moves
}

fn main() {
    let contents = fs::read_to_string("./input/moves").expect("Could not read file!");
    let lines: Vec<&str> = contents.split("\n").collect();
    let moves = get_moves(lines);
    let result_scores: Vec<i32> = moves.iter().map(|m| result_score(m)).collect();
    let hand_scores: Vec<i32> = moves.iter().map(|m| hand_score(&m.me)).collect();

    let total_result_score: i32 = result_scores.iter().sum();
    let total_hand_score: i32 = hand_scores.iter().sum();

    let total_score = total_result_score + total_hand_score;

    println!("{}", total_score);
}
