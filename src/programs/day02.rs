use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let strategy_guide = collect_turns(input);
    let result = compute_result(part_number, strategy_guide);
    format!("{}", result)
}

fn compute_result(part_number: Parts, strategy_guide: Vec<(&str, &str)>) -> i32 {
    match part_number {
        Parts::One => compute_all_turns_score(strategy_guide, RPSStrategy::Play),
        Parts::Two => compute_all_turns_score(strategy_guide, RPSStrategy::Outcome),
    }
}

#[derive(Clone, Copy, Debug)]
enum RPSPlay {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RPSStrategy {
    Play,
    Outcome,
}

impl RPSStrategy {
    fn decode_turn(&self, opponent_code: &str, second_code: &str) -> RPSTurn {
        match self {
            Self::Play => self.play_strategy(opponent_code, second_code),
            Self::Outcome => self.outcome_strategy(opponent_code, second_code),
        }
    }

    fn opponent_play(&self, turn_code: &str) -> RPSPlay {
        match turn_code {
            "A" => RPSPlay::Rock,
            "B" => RPSPlay::Paper,
            "C" => RPSPlay::Scissors,
            _ => panic!("Unknown turn code for opponent {}", turn_code),
        }
    }

    fn play_strategy(&self, opponent_code: &str, player_code: &str) -> RPSTurn {
        let player_play = match player_code {
            "X" => RPSPlay::Rock,
            "Y" => RPSPlay::Paper,
            "Z" => RPSPlay::Scissors,
            _ => panic!("Unknown turn code for player {}", player_code),
        };
        RPSTurn {
            opponent: self.opponent_play(opponent_code),
            player: player_play,
        }
    }

    fn outcome_strategy(&self, opponent_code: &str, outcome_code: &str) -> RPSTurn {
        let opponent_play = self.opponent_play(opponent_code);
        let outcome = match outcome_code {
            "X" => RPSTurnOutcome::Loss,
            "Y" => RPSTurnOutcome::Tie,
            "Z" => RPSTurnOutcome::Win,
            _ => panic!("Unknown outcome code {}", outcome_code),
        };
        match outcome {
            RPSTurnOutcome::Tie => RPSTurn {
                opponent: opponent_play,
                player: opponent_play.clone(),
            },
            RPSTurnOutcome::Win => RPSTurn {
                opponent: opponent_play,
                player: opponent_play.defeated_by(),
            },
            RPSTurnOutcome::Loss => RPSTurn {
                opponent: opponent_play,
                player: opponent_play.defeats(),
            },
        }
    }
}

impl RPSPlay {
    fn defeated_by(&self) -> Self {
        match self {
            RPSPlay::Rock => RPSPlay::Paper,
            RPSPlay::Paper => RPSPlay::Scissors,
            RPSPlay::Scissors => RPSPlay::Rock,
        }
    }

    fn defeats(&self) -> Self {
        match self {
            RPSPlay::Rock => RPSPlay::Scissors,
            RPSPlay::Paper => RPSPlay::Rock,
            RPSPlay::Scissors => RPSPlay::Paper,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum RPSTurnOutcome {
    Win,
    Tie,
    Loss,
}

#[derive(Debug)]
struct RPSTurn {
    opponent: RPSPlay,
    player: RPSPlay,
}

impl RPSTurn {
    fn score(&self) -> i32 {
        self.match_score() + self.player.score()
    }

    fn match_outcome(&self) -> RPSTurnOutcome {
        match (&self.player, &self.opponent) {
            (RPSPlay::Rock, RPSPlay::Scissors) => RPSTurnOutcome::Win,
            (RPSPlay::Rock, RPSPlay::Paper) => RPSTurnOutcome::Loss,
            (RPSPlay::Scissors, RPSPlay::Paper) => RPSTurnOutcome::Win,
            (RPSPlay::Scissors, RPSPlay::Rock) => RPSTurnOutcome::Loss,
            (RPSPlay::Paper, RPSPlay::Rock) => RPSTurnOutcome::Win,
            (RPSPlay::Paper, RPSPlay::Scissors) => RPSTurnOutcome::Loss,
            (_, _) => RPSTurnOutcome::Tie,
        }
    }

    fn match_score(&self) -> i32 {
        match self.match_outcome() {
            RPSTurnOutcome::Win => 6,
            RPSTurnOutcome::Tie => 3,
            RPSTurnOutcome::Loss => 0,
        }
    }
}

fn compute_all_turns_score(strategy_guide: Vec<(&str, &str)>, strategy: RPSStrategy) -> i32 {
    strategy_guide
        .iter()
        .map(|(code1, code2)| strategy.decode_turn(code1, code2).score())
        .sum()
}

fn collect_turns(input: &str) -> Vec<(&str, &str)> {
    let mut turns: Vec<(&str, &str)> = vec![];
    for l in input.split('\n') {
        if l.trim().is_empty() {
            continue;
        }
        let code_pair = l.trim().split(' ').collect::<Vec<_>>();
        let turn = (code_pair[0], code_pair[1]);
        turns.push(turn);
    }
    turns
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_helpers::vec_compare;

    use std::fs;

    #[test]
    fn test_collect_turns() {
        let fixture_file = "./data/day02/test.txt";
        let test_input = fs::read_to_string(fixture_file).expect("Failed to read input file");
        let result = collect_turns(&test_input);
        let expected = vec![("A", "Y"), ("B", "X"), ("C", "Z")];
        assert!(vec_compare(&result, &expected));
    }

    #[test]
    fn test_play_strategy() {
        let test_data = vec![("A", "Y"), ("B", "X"), ("C", "Z")];
        let result = compute_all_turns_score(test_data, RPSStrategy::Play);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_outcome_strategy() {
        let test_data = vec![("A", "Y"), ("B", "X"), ("C", "Z")];
        let result = compute_all_turns_score(test_data, RPSStrategy::Outcome);
        assert_eq!(result, 12);
    }
}
