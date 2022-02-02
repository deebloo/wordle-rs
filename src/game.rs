#[derive(Debug, PartialEq)]
pub enum State {
    NA,
    RightChar,
    RightPos,
}

type Word = Vec<char>;
type GuessResult = [State; 5];

pub struct Game {
    word: Word,
}

impl Game {
    pub fn new(word: &'static str) -> Self {
        Self {
            word: word.chars().collect(),
        }
    }

    pub fn guess(&self, guess: &'static str) -> GuessResult {
        let mut res: GuessResult = [State::NA, State::NA, State::NA, State::NA, State::NA];

        for (i, char) in guess.chars().enumerate() {
            if self.word.contains(&char) {
                res[i] = State::RightChar
            }

            if &self.word[i] == &char {
                res[i] = State::RightPos;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_res_0() {
        let game = Game::new("hello");
        let res = game.guess("hello");

        assert_eq!(
            res,
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
        let game = Game::new("hello");
        let res = game.guess("lhoel");

        assert_eq!(
            res,
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
        let game = Game::new("hello");
        let res = game.guess("helps");

        assert_eq!(
            res,
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
        let game = Game::new("hello");
        let res = game.guess("hille");

        assert_eq!(
            res,
            [
                State::RightPos,
                State::NA,
                State::RightPos,
                State::RightPos,
                State::RightChar
            ]
        );
    }
}
