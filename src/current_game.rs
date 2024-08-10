pub struct CurrentGame {
    word: String,
    masked_word: String,
    attempts_left: i32,
    guessed_letters: Vec<char>,
}

impl CurrentGame {
    pub fn new(word: String, attempts: i32) -> Self {
        let masked_word = word.chars().map(|_| '*').collect();
        CurrentGame {
            word,
            masked_word,
            attempts_left: attempts,
            guessed_letters: Vec::new(),
        }
    }

    pub fn word(&self) -> &str {
        &self.word
    }

    pub fn masked_word(&self) -> &str {
        &self.masked_word
    }

    pub fn attempts_left(&self) -> i32 {
        self.attempts_left
    }

    pub fn guessed_letters(&self) -> &Vec<char> {
        &self.guessed_letters
    }

    pub fn add_guess(&mut self, guess: char) {
        self.guessed_letters.push(guess);
    }

    pub fn already_guessed(&self, guess: &char) -> bool {
        self.guessed_letters.contains(guess)
    }

    pub fn word_contains(&self, guess: char) -> bool {
        self.word.contains(guess)
    }

    pub fn reveal_letter(&mut self, guess: char) {
        self.masked_word = self.word
            .chars()
            .enumerate()
            .map(|(i, c)| if c == guess { c } else { self.masked_word.chars().nth(i).unwrap() })
            .collect();
    }

    pub fn decrement_attempts(&mut self) {
        if self.attempts_left > 0 {
            self.attempts_left -= 1;
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.attempts_left <= 0 || self.masked_word == self.word
    }

    pub fn is_won(&self) -> bool {
        self.masked_word == self.word
    }
}
