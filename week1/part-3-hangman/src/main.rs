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

fn swap_letter(vec1: &mut Vec<char>, vec2: &mut Vec<char>, ch: char) {
    // vec1 is the original word.
    // vec2 is the empty guessed word.
    // We have to swap the letter found in 'ch', so substituting the
    // original char with '-' and substitute the '-' char in the guessed
    // word with the letter.
    // Comment out the prints to debug
    // println!("vec1: {:?}", vec1);
    // println!("vec2: {:?}", vec2);
    // println!("ch: {:?}", ch);
    if vec1.contains(&ch) {
        let pos = vec1.iter().position(|x| *x == ch).unwrap();
        // println!("pos: {:?}", pos);
        vec1[pos] = '-';
        vec2[pos] = ch;
    }
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let mut secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let mut guessed: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut guessed_wrong_num = NUM_INCORRECT_GUESSES;
    let mut guessed_correct_num = 0;
    let mut guessed_letters = String::new();
    //println!("guessed word: {:?}", guessed);
    loop {
        println!(
            "\nThe word so far is {}",
            guessed.iter().collect::<String>()
        );
        println!(
            "You have guessed the following letters: {}",
            guessed_letters
        );
        println!("You have {} guesses left", guessed_wrong_num);
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let guess_char: Vec<char> = guess.chars().collect();
        if secret_word_chars.contains(&guess_char[0]) {
            swap_letter(&mut secret_word_chars, &mut guessed, guess_char[0]);
            guessed_correct_num += 1;
            guessed_letters.push(guess_char[0]);
            if guessed_correct_num == secret_word_chars.len() {
                println!(
                    "\nCongratulations you guessed the secret word: {}",
                    guessed.into_iter().collect::<String>()
                );
                break;
            }
        } else {
            guessed_wrong_num -= 1;
            println!("Sorry, that letter is not in the word");
            if guessed_wrong_num.eq(&0) {
                println!("\nSorry, you ran out of guesses!");
                break;
            }
        }
    }
}
