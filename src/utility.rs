use rand::seq::SliceRandom;
use colored::*;

pub fn purify_word(mut word: String) -> String {
    let not_allowed = vec!['[', ']', '"'];
    word.retain(|c| !not_allowed.contains(&c));
    word
}

pub fn shuffle_word(word: &String) -> String {
    let word_copy = word.clone();
    let mut rng = rand::thread_rng();
    let mut bytes = word_copy.into_bytes();
    bytes.shuffle(&mut rng);
    let shuffled = String::from_utf8(bytes).unwrap();
    shuffled
}

pub fn print_guess(word: &String, guess: &String) {
    let vec_word: Vec<char> = word.chars().collect();
    let vec_guess: Vec<char> = guess.chars().collect();
    let mut count = 0;

    for c in vec_word {
        if c == vec_guess[count] {
            print!("{}", vec_guess[count].to_string().green());
        }
        else {
            print!("{}", vec_guess[count]);
        }
        count += 1;
    }
}
