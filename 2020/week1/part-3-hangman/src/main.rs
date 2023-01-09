// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();

    let mut counter: usize = 0;
    let mut guess_counter = NUM_INCORRECT_GUESSES;
    let word_length: usize = secret_word_chars.len();

    // Create an array map for the characters
    let mut map: [usize; 26] = [0; 26];
    for i in 0..word_length {
        map[secret_word_chars[i] as usize - 'a' as usize] += 1;
    }

    let show_line: String = String::from("The word so far is ");
    let mut already_guessed = String::from("");

    let dash = '-';
    let mut print_word = String::from("");

    for _ in 0..word_length {
        print_word.push(dash);
    }

    while guess_counter > 0 && counter < word_length {
        println!("{}{}", show_line, print_word);
        print!("You have guessed the following letters:");
        println!("{}", already_guessed);
        println!("You have {} guesses left", guess_counter);
        print!("Please guess a letter: ");

        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let ch = guess.chars().next().unwrap();
        already_guessed.push(ch);
        
        let index = ch as usize - 'a' as usize;
        if map[index] > 0 {
            counter += map[index];
            map[index] = 0;
            for i in 0..word_length {
                if secret_word_chars[i] == ch {
                    print_word.replace_range(i..i + 1, &ch.to_string());
                }
            }
        } else {
            guess_counter -= 1;
            println!("Sorry, that letter is not in the word");
        }
        println!("");
    }

    if guess_counter == 0 {
        println!("Sorry, you ran out of guesses!");
    } else {
        println!("Congratulations you guessed the secret word: {}!", print_word);
    }
}
