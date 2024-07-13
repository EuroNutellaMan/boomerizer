use rand::{Rng, seq::SliceRandom};
use colored::*;
use std::io;

fn main() {
    // introducing the variables for dots and a list of emojis
    let dot: char = '.';
    let emojis: [char; 14] = ['🤣', '💀', '🤪', '😍', '😡', '😂', '😊', '🙄', '🥲', '😘', '😏', '👍', '🙅', '🤦'];

    // get the percentage that will be used to generate the text later
    let percent_dots = percentage_assigner("dots");
    let percent_emojis = percentage_assigner("emojis");

    // ask user for the text to be converted
    println!("{}", "Input a text that needs to be boomerized:".yellow());
    let mut normal_text = String::new();
    io::stdin()
        .read_line(&mut normal_text)
        .expect(&"Failed to read line!".red());

    // this will store the boomerized text
    let mut boomer_text = String::new();

    // splits the normal text by whitespace and iterates through every word
    // for each word the program decides if dots and emojis should be added using the
    // aforementioned percentages, if yes then the program will add dots and/or emojis
    // then it will add the new word to boomer_text
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

    // deletes the last ' ', it may remove a '.' if the last word has been dotted but that's ok
    boomer_text.pop();

    // prints the result
    println!("{}", "Boomerized text:".green());
    println!("{boomer_text}");
    
    // on windows it keeps the command line open until the user decides to close it instead of
    // closing it as soon as the program is done, which is a problem because it makes it impossible
    // for the user to actually read and copy the boomerized text. What a silly OS that is.
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;
        println!("{}", "Press Enter to exit...".yellow());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}

fn add_dots(word: &str, dot: &char) -> String {
    // how many dots should be added
    let num_dots: u32 = rand::thread_rng()
        .gen_range(3..=5);

    let mut result = String::from(word);

    // adds the appropriate number of dots to the word
    for _ in 0..num_dots {
        result.push(*dot);
    }

    // decide if we should add a space or not after the dots
    let space: bool = rand::thread_rng()
        .gen_bool(0.3);
    if space {
        format!("{result} ")
    } else {
        format!("{result}")
    }
}

fn add_emoji(word: &str, emojis: &[char; 14]) -> String {
    // choose which emoji and how many copies of it should be added to the word
    let mut choice = rand::thread_rng();
    let emoji: &char = emojis.choose(&mut choice).unwrap();
    let num_emojis: u32 = rand::thread_rng()
        .gen_range(1..=5);

    let mut result = String::from(word);

    // add said emoji num_emoji amount of times
    for _ in 0..num_emojis {
        result.push(*emoji);
    }

    // returns the emojified word with a ' '
    format!("{result} ")
}

// this fn serves the purpose of asking the user to input a percentage
// it asks for a word to tell the user what the percentage is for
fn percentage_assigner(word: &str) -> f64 {
    // loop to ask the user and handle eventual errors
    let percentage = loop {
        println!("{} {}{}", "Percentage for text that needs to be followed by".yellow(), format!("{word}").blue(), ":".yellow());
        println!("{}", "Input a number between 0.0 and 1.0 (ex: 0.5):".yellow());
        let mut percentage = String::new();

        io::stdin()
            .read_line(&mut percentage)
            .expect(&"failed to read line!".red());

        // check the user input if it's convertible to f64 and if it's between 0 and 1
        // if not it loops back and ask the user to input the percentage again
        // if yes it returns the f64
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

    // returns the percentage obtained from the loop
    percentage
}
