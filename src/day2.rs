#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Clone, Copy)]
enum GameResult {
    Loss = -1,
    Draw = 0,
    Win = 1
}

enum YouValue {
    X,
    Y,
    Z
}

impl Move {
    fn from_you_value(you_value: &YouValue) -> Move {
        match you_value {
            YouValue::X => Move::Rock,
            YouValue::Y => Move::Paper,
            YouValue::Z => Move::Scissors
        }
    }
}

impl GameResult {
    fn from_you_value(you_value: &YouValue) -> GameResult {
        match you_value {
            YouValue::X => GameResult::Loss,
            YouValue::Y => GameResult::Draw,
            YouValue::Z => GameResult::Win
        }
    }
}

pub struct Round {
    opponent: Move,
    you: YouValue,
}

fn parse_opponent(input: &str) -> Option<Move> {
    match input {
        "A" => Some(Move::Rock),
        "B" => Some(Move::Paper),
        "C" => Some(Move::Scissors),
        _ => None
    }
}

fn parse_you(input: &str) -> Option<YouValue> {
    match input {
        "X" => Some(YouValue::X),
        "Y" => Some(YouValue::Y),
        "Z" => Some(YouValue::Z),
        _ => None
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split_whitespace()
                .collect();
            let opponent = parse_opponent(moves[0]).unwrap();
            let you = parse_you(moves[1]).unwrap();
            Round { opponent: opponent, you: you}
        })
        .collect()
}

fn get_result_from_moves(opponent: Move, you: Move) -> Option<GameResult> {
    match (opponent as i32 - you as i32 + 3) % 3 {
        0 => Some(GameResult::Draw),
        1 => Some(GameResult::Loss),
        2 => Some(GameResult::Win),
        _ => None
    }
}

fn get_move_from_result(opponent: Move, result: GameResult) -> Option<Move> {
    match (opponent as i32 + result as i32 + 3) % 3 {
        0 => Some(Move::Scissors),
        1 => Some(Move::Rock),
        2 => Some(Move::Paper),
        _ => None
    }
}

fn score_result(result: GameResult) -> u32 {
    match result {
        GameResult::Loss => 0,
        GameResult::Draw => 3,
        GameResult::Win => 6
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Round>) -> u32 {
    input
        .iter()
        .map(|round| {
            let you_move = Move::from_you_value(&round.you);
            let result = get_result_from_moves(round.opponent, you_move).unwrap();
            score_result(result) + you_move as u32
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Round>) -> u32 {
    input
        .iter()
        .map(|round| {
            let result = GameResult::from_you_value(&round.you);
            let you_move = get_move_from_result(round.opponent, result).unwrap();
            score_result(result) + you_move as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = input_generator("A Y\nB X\nC Z");
        assert_eq!(solve_part1(&input), 15);
        assert_eq!(solve_part2(&input), 12);
    }
}