use std::io::Error;
use std::io::Read;

enum HangmanStatus {
    Win,
    Lose,
    Playing,
    Starting,
}

struct Hangman {
    word: String,
    current_word: String,
    guesses: u8,
    status: HangmanStatus,
}

impl Hangman {
    fn new(word: &str, guesses: u8) -> Option<Self> {
        Some(Hangman {
            word: word.to_string(),
            current_word: Hangman::initial_word(&word)?,
            guesses: guesses,
            status: HangmanStatus::Starting,
        })
    }

    fn initial_word(word: &str) -> Option<String> {
        let length = word.len();
        let mut initial = String::new();
        initial.push(word.chars().nth(0)?);
        initial.push_str(&"_".repeat(length - 2));
        initial.push(word.chars().last()?);
        Some(initial)
    }

    fn game_loop(&mut self) -> Option<bool> {
        match &self.status {
            HangmanStatus::Starting => {
                println!("Let's start, here is the word: {}", self.current_word);
                self.status = HangmanStatus::Playing;
                Some(true)
            }

            HangmanStatus::Playing => {
                let mut input_line = String::new();
                let len = std::io::stdin().read_line(&mut input_line).unwrap();
                if len > 2 {
                    //includes the \n
                    if input_line.to_lowercase()[0..input_line.len() - 1] == self.word.to_lowercase() {
                        self.status = HangmanStatus::Win;
                        Some(true);
                    } else {
                        self.guesses = self.guesses - 1;
                        println!("Nope, wrong word :(");
                        Some(true);
                    }
                } else {
                    let guessed = input_line.chars().nth(0)?;
                    match &self.guess_char(guessed) {
                        Some(guessed_correctly) => {
                            self.guesses = self.guesses - 1;
                            match guessed_correctly {
                                true => {
                                    println!("Congrats, the word now is: {}", self.current_word);
                                    if self.guesses == 1 {
                                        println!("Time for your last guess!");
                                    }
                                    if !self.current_word.contains("_") {
                                        // no chars left to guess, word is found
                                        self.status = HangmanStatus::Win;
                                    }
                                }
                                false => {
                                    println!("I'm sorry, that doesn't appear in the word :(");
                                }
                            }
                        }
                        None => println!("Character not valid, try again"),
                    };
                }
                if self.guesses == 0 {
                    self.status = HangmanStatus::Lose;
                }
                Some(true)
            }

            HangmanStatus::Win => {
                println!("Congrats, you won! The word was {}", self.word);
                Some(false)
            }

            HangmanStatus::Lose => {
                println!("Sadly, you finished all your guesses. The word was {} :(", self.word);
                Some(false)
            }
        }
    }

    fn guess_char(&mut self, guessed: char) -> Option<bool> {
        if (guessed >= 'a' && guessed <= 'z') || (guessed >= 'A' && guessed <= 'Z') {
            let mut guessed_correctly = false;
            let mut new_word = String::new();
            for (i, c) in self.word.chars().enumerate() {
                if c.to_lowercase().eq(guessed.to_lowercase()) && self.current_word.chars().nth(i)? == '_' {
                    guessed_correctly = true;
                    new_word.push(c);
                } else {
                    new_word.push(self.current_word.chars().nth(i)?);
                }
            }
            self.current_word = new_word;
            Some(guessed_correctly)
        } else {
            None
        }
    }
}

fn main() -> Result<(), Error> {
    // todo already guessed chars?
    // todo multiple words (accept spaces?)
    let mut hangman = Hangman::new("Xylophone", 10).unwrap();
    while hangman.game_loop().unwrap() {}
    Ok(())
}