use std::io;

const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B';
const OPPONENT_SCISSORS: char = 'C';

const PLAYER_ROCK: char = 'X';
const PLAYER_PAPER: char = 'Y';
const PLAYER_SCISSORS: char = 'Z';

const ROCK_POINTS: usize = 1;
const PAPER_POINTS: usize = 2;
const SCISSORS_POINTS: usize = 3;
const LOSS_POINTS: usize = 0;
const DRAW_POINTS: usize = 3;
const WIN_POINTS: usize = 6;

// Rust probably has switches and stuff
fn hand_points(person: char) -> usize {
    if person == PLAYER_ROCK {
        return ROCK_POINTS;
    }
    if person == PLAYER_PAPER {
        return PAPER_POINTS;
    }

    SCISSORS_POINTS
}

fn rock_paper_scissors_points(opponent: char, person: char) -> usize {
    if opponent == person {
        return DRAW_POINTS;
    }

    if opponent == OPPONENT_ROCK {
        if person == PLAYER_PAPER {
            return WIN_POINTS;
        }
        if person == PLAYER_ROCK {
            return DRAW_POINTS;
        }
    }
    if opponent == OPPONENT_PAPER {
        if person == PLAYER_SCISSORS {
            return WIN_POINTS;
        }
        if person == PLAYER_PAPER {
            return DRAW_POINTS;
        }
    }
    if opponent == OPPONENT_SCISSORS {
        if person == PLAYER_ROCK {
            return WIN_POINTS;
        }
        if person == PLAYER_SCISSORS {
            return DRAW_POINTS;
        }
    }

    //
    LOSS_POINTS
}

// Calculate the player's points for that round
fn calculate_round_points(opponent: char, person: char) -> usize {
    hand_points(person) + rock_paper_scissors_points(opponent, person)
}

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("Error reading");
    let mut total_points: usize = 0;
    while !buf.is_empty() {
        let opponent: char = buf.chars().nth(0).unwrap();
        let person = buf.chars().nth(2).unwrap();

        total_points += calculate_round_points(opponent, person);

        buf.clear();
        io::stdin().read_line(&mut buf).expect("Error reading");
    }

    println!("{}", total_points)
}
