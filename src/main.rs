use colored::*;
use std::io;
use std::io::Write;

mod utility;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    loop {

        clearscreen::clear().unwrap();
        println!("Welcome to anacrab!\nA game based on anagrams written by xflea in Rust.");
        println!("Words are provided by REST API at https://random-word-api.herokuapp.com/word");
        println!("\nPlease select the language:");
        println!("it - Italiano");
        println!("es - Español");
        println!("de - Deutsch");
        println!("Other input will extract an English word.");
        println!("\nIf you want to quit the game, type 'q'.");

        let mut language = String::new();
        let mut language_resp = String::from("https://random-word-api.herokuapp.com/word");
        print!("\n> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut language).unwrap();

        if language.trim() == "it" || language.trim() == "es" || language.trim() == "de" {
            let mut param = String::from("?lang=");
            param.push_str(&language.trim());
            language_resp.push_str(&param);
        }
        else if language.trim() == "q" {
            break;
        }

        let mut guess = String::new();
        let mut streak: i32 = 0;

        loop {

            clearscreen::clear().unwrap();
            println!("Retriving the word from server... Please wait...");

            let resp: String = reqwest::Client::new().get(&language_resp).send().await?.text().await?;
            let word = utility::purify_word(resp);
            let word_shuffled = utility::shuffle_word(&word);
            let mut string_guess;
            let mut errors: i8 = 5;

            string_guess = "Choose wisely".bold().italic().to_string();

            // game loop starts
            loop {

                clearscreen::clear().unwrap();

                println!("Errors available: {} - {}", errors, string_guess);
                println!("Current streak - {}", streak);
                println!("Test - {}", word);
                println!("{}", word_shuffled);
                if guess.trim().to_ascii_lowercase().chars().count() == word.chars().count() {
                    utility::print_guess(&word, &guess);
                }
                guess = "".to_string();

                // read the guess
                print!("\n> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut guess).unwrap();

                if guess.trim().to_ascii_lowercase().chars().count() == word.chars().count() {

                    if guess.trim().to_ascii_lowercase() == word {
                        string_guess = "CORRECT!".green().bold().italic().to_string();
                        streak = streak + 1;
                        println!("\nYou won! The word was {}.\nYou're awsome!", word.green().bold().italic());
                        break;
                    }
                    else {
                        errors = errors - 1;
                        string_guess = "Wrong...".red().bold().italic().to_string();

                        if errors < 0 {
                            println!("\nYou lost... better luck next time!");
                            streak = 0;
                            break;
                        }
                    }

                }
                else {
                    string_guess = "Please provide a valid input...".bold().italic().to_string();
                }

            }

            let mut to_continue = String::new();
            print!("\nIf you wish to stop, type 'q', if you wish to continue, press any key!");
            print!("\n> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut to_continue).unwrap();

            if to_continue.trim() == "q" {
                break;
            }

        }

    }

    println!("Thanks for playing!");
    Ok(())

}
