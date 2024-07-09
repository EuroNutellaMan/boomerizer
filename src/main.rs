use rand::{Rng, seq::SliceRandom};
use colored::*;
use std::io;

fn main() {
    let dot: char = '.';
    let emojis: [char; 14] = ['🤣', '💀', '🤪', '😍', '😡', '😂', '😊', '🙄', '🥲', '😘', '😏', '👍', '🙅', '🤦'];

    let percent_dots = percentage_assigner("dots");

    let percent_emojis = percentage_assigner("emojis");

    println!("{}", "Input a text that needs to be boomerized:".yellow());
    let mut normal_text = String::new();
    io::stdin()
        .read_line(&mut normal_text)
        .expect(&"Failed to read line!".red());

    let mut boomer_text = String::new();

    for word in normal_text.split_whitespace() {
        let dotting: bool = rand::thread_rng()
            .gen_bool(percent_dots);
        let emojing: bool = rand::thread_rng()
            .gen_bool(percent_emojis);

        let boomer_word = if dotting && emojing {
            let half_boomerized = add_dots(word, &dot);
            add_emoji(&half_boomerized, &emojis)
        } else if dotting {
            add_dots(word, &dot)
        } else if emojing {
            add_emoji(word, &emojis)
        } else {
            format!("{word} ")
        };

        boomer_text += &boomer_word;
    }

    boomer_text.pop();

    println!("{}", "Boomerized text:".green());
    println!("{boomer_text}");
}

fn add_dots(word: &str, dot: &char) -> String {
    let num_dots: u32 = rand::thread_rng()
        .gen_range(3..=5);

    let mut result = String::from(word);

    for _ in 0..num_dots {
        result.push(*dot);
    }

    let space: bool = rand::thread_rng()
        .gen_bool(0.3);
    if space {
        format!("{result} ")
    } else {
        format!("{result}")
    }
}

fn add_emoji(word: &str, emojis: &[char; 14]) -> String {
    let mut choice = rand::thread_rng();
    let emoji: &char = emojis.choose(&mut choice).unwrap();
    let num_emojis: u32 = rand::thread_rng()
        .gen_range(1..=5);

    let mut result = String::from(word);

    for _ in 0..num_emojis {
        result.push(*emoji);
    }

    format!("{result} ")
}

fn percentage_assigner(word: &str) -> f64 {
    let percentage = loop {
        println!("{} {}{}", "Percentage for text that needs to be followed by".yellow(), format!("{word}").blue(), ":".yellow());
        println!("{}", "Input a number between 0.0 and 1.0 (ex: 0.5):".yellow());
        let mut percentage = String::new();

        io::stdin()
            .read_line(&mut percentage)
            .expect(&"failed to read line!".red());

        let _percentage: f64 = match percentage
            .trim()
            .parse() {
                Ok(num) => {
                    if num <= 1.0 && num >= 0.0 {
                        break num;
                    } else {
                        println!("{}", "Failed to generate percentage: number is not between 0.0 and 1.0! Try again.".red());
                        continue;
                    }
                }
                Err(_) => {
                    println!("{}", "Failed to generate percentage: number is not a float! Try again.".red());
                    continue;
                }
            };
    };

    percentage
}
