use pig_latin::to_pig_latin_sentence;
use std::io;
fn main() {
    loop {
        println!("Please input your sentence to translate into Pig Latin.");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.is_ascii() {
            {
                println!("Translation: {}", to_pig_latin_sentence(&user_input));
                break;
            }
        } else {
            println!("Please only insert ASCII text.");
        }
    }
}
