use rand::Rng;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct Word {
    word_text: String,
    displayed_text: Vec<char>,
}

impl Word {
    fn new(word_text: String) -> Self {
        // constructs a word object from text
        let displayed_text = vec!['_'; word_text.len()];
        Word {
            word_text,
            displayed_text,
        }
    }

    fn display(&self) -> String {
        // turns a collection of char into a string to be displayed
        return self.displayed_text.iter().collect();
    }

    fn check_guess(&mut self, guess: char) -> bool {
        // checks the guessed word and modifies
        // the displayed text if a guessed letter
        // is found
        let mut found = false;
        for (index, letter) in self.word_text.chars().enumerate() {
            if letter == guess {
                self.displayed_text[index] = letter;
                found = true;
            }
        }
        return found;
    }

    fn is_guessed(&self) -> bool {
        return !self.displayed_text.contains(&'_');
    }
}


// Manages the game state: the word, guessed letters, and incorrect guesses.
struct Guess {
    // the player's guessed character
    letter: char,
}

struct Hangman {
    word: Word,
    guessed_letters: Vec<char>,
    incorrect_guesses: i32,
}

impl Hangman {
    fn new(word_text: String) -> Self {
        let word = Word::new(word_text); // creates a word object from the text
        let guessed_letters = Vec::new(); // stores guessed letters
        let incorrect_guesses = 0;
        Hangman {
            word,
            guessed_letters,
            incorrect_guesses,
        }
    }

    fn get_guess(&self) -> Guess {
        let stdin = io::stdin(); // lets you read input from user

        loop {
            println!("Enter your guess:");

            let mut input = String::new();
            stdin
                .lock() // returns a locked stdinLock to ensure exclusive access
                .read_line(&mut input)
                .expect("Failed to read line");

            // trim whitespace and convert to lowercase
            let trimmed_input = input.trim().to_lowercase();

            // make sure a single character is entered
            if trimmed_input.len() != 1 {
                println!("Please enter a single character.");
                continue;
            }

            // get the first character
            let guess = trimmed_input.chars().next().unwrap();

            // ensure the character is alphabetic
            if !guess.is_alphabetic() {
                println!("Please enter an alphabetic character.");
                continue;
            }

            return Guess { letter: guess };
        }
    }

    fn display_word(&self) {
        // use the word object display method to display
        // the position of the guessed letters
        println!("Word: {}", self.word.display());
    }

    fn display_guessed_letters(&self) {
        // turns a collection of char into a string to be displayed
        let guessed_letters: String = self.guessed_letters.iter().collect();
        println!("Guessed letters: {}", guessed_letters);
    }

    fn draw_hangman(&self) {
        //draws your hangman
        let hangman_figure = match self.incorrect_guesses {
            0 => {
                "
              +---+
              |   |
                  |
                  |
                  |
                  |
            =========\n"
            }
            1 => {
                "
              +---+
              |   |
              O   |
                  |
                  |
                  |
            =========\n"
            }
            2 => {
                "
              +---+
              |   |
              O   |
              |   |
                  |
                  |
            =========\n"
            }
            3 => {
                "
              +---+
              |   |
              O   |
             /|   |
                  |
                  |
            =========\n"
            }
            4 => {
                "
              +---+
              |   |
              O   |
             /|\\  |
                  |
                  |
            =========\n"
            }
            5 => {
                "
              +---+
              |   |
              O   |
             /|\\  |
             /    |
                  |
            =========\n"
            }
            6 => {
                "
              +---+
              |   |
              O   |
             /|\\  |
             / \\  |
                  |
            =========\n"
            }
            _ => "",
        };

        println!("{}", hangman_figure);
    }

    fn handle_guess(&mut self, guess: Guess) {
        // evaluates the guess object's letter
        // and makes sure it is not a repeated guess
        let letter = guess.letter;

        if self.guessed_letters.contains(&letter) {
            println!("Oops! @~@ You've already guessed that letter!");
        } else {
            self.guessed_letters.push(letter);
            let found = self.word.check_guess(letter);
            if !found {
                self.incorrect_guesses += 1;
                println!(
                    "Wrong guess! Try again. lives left: {}",
                    6 - self.incorrect_guesses
                );
            }
        }
    }

    fn determine_game_outcome(&self) -> bool {
        if self.word.is_guessed() {
            println!(
                "Congratulations! You've guessed the word: {}!",
                self.word.word_text
            );
            true
        } else {
            false
        }
    }

    fn play(&mut self) {
        // game loop, displays the word
        // displays the guessed letters
        // checks for a guess
        loop {
            self.draw_hangman();
            self.display_word();
            self.display_guessed_letters();
            let guess = self.get_guess();
            self.handle_guess(guess);

            if self.incorrect_guesses == 6 {
                // if you run out of lives you lose the game
                println!(
                    "Game Over! You ran out of lives. The word was: {}",
                    self.word.word_text
                );
                self.draw_hangman();
                break;
            }

            if self.determine_game_outcome() {
                // checks if you have won the game
                break;
            }
        }
    }
}

fn read_words_from_file(file_path: &str) -> Vec<String> {
    /*
     *  Takes a file path as input and reads
     *  the file using buffered reading to return
     *  a vector of words from the file.
     */

    let file = File::open(file_path) // try to open file
        .expect("Failed to open file"); // if the file can't be opened show this error
    let reader = BufReader::new(file); //create a new BufReader from file
    let mut words = Vec::new(); // create new mutable vector

    for line in reader.lines() {
        // if line read successfully push word to vector
        if let Ok(word) = line {
            words.push(word);
        }
    }

    return words;
}

fn pick_random_word(words: &[String]) -> String {
    /*
     *  Picks a random word from the given slice
     *  containing the vector of words
     *  and outputs a reference to the selected word
     */

    let word_index = rand::thread_rng().gen_range(0..words.len());
    words[word_index].clone()
}

fn get_category_choice() -> String {
    /*
     *  Helps the user select a category for
     *  the hangman game
     */
    loop {
        println!("Choose a category:");
        println!("1. Vegetables");
        println!("2. Fruits");
        println!("3. Animals");
        println!("4. Countries");

        // reads the user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // match the input to the result
        match input.trim() {
            "1" => return String::from("src/vegetables.txt"),
            "2" => return String::from("src/fruits.txt"),
            "3" => return String::from("src/animals.txt"),
            "4" => return String::from("src/countries.txt"),
            _ => println!("Invalid choice. Please try again. :P"),
        }
    }
}

fn play_again() -> bool {
    /*
     * Let's the player decide if they want to play again
     */

    loop {
        println!("Do you want to play again?:");
        println!("1. Yes");
        println!("2. No");

        // reads the user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // match the input to the result
        match input.trim() {
            "1" => return true,
            "2" => return false,
            _ => println!("Invalid choice. Please try again. :P"),
        }
    }
}

fn main() {
    loop {
        println!("Welcome to the Hangman Game!");

        let category_path = get_category_choice();
        let words = read_words_from_file(&category_path);

        let word_text = pick_random_word(&words);
        let mut game = Hangman::new(word_text);

        game.play();

        if play_again() {
            continue;
        } else {
            println!("Goodbye!");
            break;
        }
    }
}
