use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Blank,
    NA,
    RightChar,
    RightPos,
}

impl State {
    pub fn to_str(&self) -> &str {
        match self {
            State::Blank => "⬛",
            State::NA => "⬜",
            State::RightChar => "🟨",
            State::RightPos => "🟩",
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Incomplete,
    Win,
    Loose,
}

type Word = Vec<char>;
type GuessResult = [State; 5];

pub struct Game {
    pub word: Word,
    pub game_state: Vec<GuessResult>,
    pub turn: usize,
}

impl Game {
    pub fn new(word: &str) -> Self {
        Self {
            word: word.trim().chars().collect(),
            turn: 0,
            game_state: vec![
                [
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                ],
                [
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                ],
                [
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                ],
                [
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                ],
                [
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                ],
                [
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                    State::Blank,
                ],
            ],
        }
    }

    pub fn guess(&mut self, guess: &str) -> GameResult {
        let mut res: GuessResult = [
            State::Blank,
            State::Blank,
            State::Blank,
            State::Blank,
            State::Blank,
        ];

        for (i, char) in guess.trim().chars().enumerate() {
            if &self.word[i] == &char {
                res[i] = State::RightPos;
            } else if self.word.contains(&char) {
                res[i] = State::RightChar
            } else {
                res[i] = State::NA;
            }
        }

        self.game_state[self.turn] = res;
        self.turn += 1;

        if res.clone().iter().all(|state| state == &State::RightPos) {
            return GameResult::Win;
        } else if self.turn > self.game_state.len() - 1 {
            return GameResult::Loose;
        }

        GameResult::Incomplete
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = String::new();

        for state in &self.game_state {
            for space in state {
                grid += space.to_str();
            }

            grid += "\n";
        }

        write!(f, "{}", grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_res_0() {
        let mut game = Game::new("hello");
        game.guess("hello");

        assert_eq!(
            game.game_state[0],
            [
                State::RightPos,
                State::RightPos,
                State::RightPos,
                State::RightPos,
                State::RightPos
            ]
        );
    }

    #[test]
    fn should_return_correct_res_1() {
        let mut game = Game::new("hello");
        game.guess("lhoel");

        assert_eq!(
            game.game_state[0],
            [
                State::RightChar,
                State::RightChar,
                State::RightChar,
                State::RightChar,
                State::RightChar
            ]
        );
    }

    #[test]
    fn should_return_correct_res_2() {
        let mut game = Game::new("hello");
        game.guess("helps");

        assert_eq!(
            game.game_state[0],
            [
                State::RightPos,
                State::RightPos,
                State::RightPos,
                State::NA,
                State::NA
            ]
        );
    }

    #[test]
    fn should_return_correct_res_3() {
        let mut game = Game::new("hello");
        game.guess("hille");

        assert_eq!(
            game.game_state[0],
            [
                State::RightPos,
                State::NA,
                State::RightPos,
                State::RightPos,
                State::RightChar
            ]
        );
    }

    #[test]
    fn should_return_loss() {
        let mut game = Game::new("hello");

        assert_eq!(game.guess("hille"), GameResult::Incomplete);
        assert_eq!(game.guess("hille"), GameResult::Incomplete);
        assert_eq!(game.guess("hille"), GameResult::Incomplete);
        assert_eq!(game.guess("hille"), GameResult::Incomplete);
        assert_eq!(game.guess("hille"), GameResult::Incomplete);
        assert_eq!(game.guess("hille"), GameResult::Loose);
    }

    #[test]
    fn should_return_win() {
        let mut game = Game::new("hello");

        assert_eq!(game.guess("hille"), GameResult::Incomplete);
        assert_eq!(game.guess("hille"), GameResult::Incomplete);
        assert_eq!(game.guess("hello"), GameResult::Win);
    }
}
