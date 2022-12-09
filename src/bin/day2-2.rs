use std::io;

const OPPONENT_ROCK: char = 'A';
const OPPONENT_PAPER: char = 'B';


const PLAYER_ROCK: char = 'X';
const PLAYER_PAPER: char = 'Y';
const PLAYER_SCISSORS: char = 'Z';

const NEEDS_LOSS: char = 'X';
const NEEDS_DRAW: char = 'Y';
const NEEDS_WIN: char = 'Z';

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

fn calculate_play(opponent: char, person: char) -> usize {
    if person == NEEDS_WIN {
        if opponent == OPPONENT_ROCK {
            return hand_points(PLAYER_PAPER);
        }
        if opponent == OPPONENT_PAPER {
            return hand_points(PLAYER_SCISSORS);
        }
        return  hand_points(PLAYER_ROCK);
    }

    if person == NEEDS_DRAW {
        if opponent == OPPONENT_ROCK {
            return hand_points(PLAYER_ROCK);
        }
        if opponent == OPPONENT_PAPER {
            return hand_points(PLAYER_PAPER);
        }
        return  hand_points(PLAYER_SCISSORS);
    }

    if person == NEEDS_LOSS {
        if opponent == OPPONENT_ROCK {
            return hand_points(PLAYER_SCISSORS);
        }
        if opponent == OPPONENT_PAPER {
            return hand_points(PLAYER_ROCK);
        }
        return  hand_points(PLAYER_PAPER);
    }

    // Shouldn't happen, I should look into option
    0
}

// Calculate the player's points for that round
fn calculate_round_points(opponent: char, person: char) -> usize {
    calculate_play(opponent, person) + outcome(person)
}

fn outcome(person: char) -> usize {
    if person == NEEDS_WIN {
        return WIN_POINTS;
    }
    if person == NEEDS_DRAW {
        return DRAW_POINTS;
    }

    LOSS_POINTS
}

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).expect("Error reading");
    let mut total_points: usize = 0;
    while !buf.is_empty() {
        let opponent: char = buf.chars().next().unwrap();
        let outcome = buf.chars().nth(2).unwrap();

        total_points += calculate_round_points(opponent, outcome);

        buf.clear();
        io::stdin().read_line(&mut buf).expect("Error reading");
    }

    println!("{}", total_points)
}
