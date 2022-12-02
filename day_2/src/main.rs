use std::fs;

#[derive(Debug)]
enum Response {
    Rock,
    Paper,
    Scissors,
}

impl Response {
    fn new(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        match self {
            Response::Rock => 1,
            Response::Paper => 2,
            Response::Scissors => 3,
        }
    }

    fn get_desired_result(&self, result: GameResult) -> Response {
        match self {
            Response::Rock => match result {
                GameResult::Win => Response::Paper,
                GameResult::Loss => Response::Scissors,
                GameResult::Tie => Response::Rock,
            },
            Response::Paper => match result {
                GameResult::Win => Response::Scissors,
                GameResult::Loss => Response::Rock,
                GameResult::Tie => Response::Paper,
            },
            Response::Scissors => match result {
                GameResult::Win => Response::Rock,
                GameResult::Loss => Response::Paper,
                GameResult::Tie => Response::Scissors,
            },
        }
    }

    fn check_win(&self, response: &Response) -> GameResult {
        match self {
            Response::Rock => match response {
                Response::Rock => GameResult::Tie,
                Response::Paper => GameResult::Loss,
                Response::Scissors => GameResult::Win,
            },
            Response::Paper => match response {
                Response::Rock => GameResult::Win,
                Response::Paper => GameResult::Tie,
                Response::Scissors => GameResult::Loss,
            },
            Response::Scissors => match response {
                Response::Rock => GameResult::Loss,
                Response::Paper => GameResult::Win,
                Response::Scissors => GameResult::Tie,
            },
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Loss,
    Tie,
}

impl GameResult {
    fn new(value: &str) -> Self {
        match value {
            "X" => Self::Loss,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        match self {
            GameResult::Win => 6,
            GameResult::Loss => 0,
            GameResult::Tie => 3,
        }
    }
}

#[derive(Debug)]
struct Game {
    player: Response,
    opponent: Response,
}

impl Game {
    fn new_part_one(value: &str) -> Self {
        let responses = value.split_once(' ').unwrap();

        Self {
            player: Response::new(responses.1),
            opponent: Response::new(responses.0),
        }
    }

    fn new_part_two(value: &str) -> Self {
        let responses = value.split_once(' ').unwrap();

        let opponent_response = Response::new(responses.0);
        let required_response = GameResult::new(responses.1);

        let player_response = opponent_response.get_desired_result(required_response);

        Self {
            player: player_response,
            opponent: opponent_response,
        }
    }

    fn result(&self) -> usize {
        let select_response_value = self.player.value();
        let result = self.player.check_win(&self.opponent).value();

        select_response_value + result
    }
}

fn main() {
    let input = fs::read_to_string("day_2/input.txt").unwrap();

    let mut games_part_one = Vec::new();
    let mut games_part_two = Vec::new();

    for game in input.split('\n') {
        games_part_one.push(Game::new_part_one(game).result());
        games_part_two.push(Game::new_part_two(game).result());
    }

    println!(
        "Your score in the first part will be: {}",
        games_part_one.iter().sum::<usize>()
    );

    println!(
        "Your score in the second part will be: {}",
        games_part_two.iter().sum::<usize>()
    );
}
