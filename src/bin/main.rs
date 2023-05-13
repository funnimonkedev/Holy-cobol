use std::io;
use loxable::Lexer;
use std::{fs, io::Read};

fn main() {
    
    println!("Would you like to provide a terminal based input or file based input? type `1` for terminal, or `2` for file.");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("failed to read user input.");
    let user_input = user_input.trim();
    if user_input == "2" {
        println!("\nWelcome to loxable cargo, please input filename.");    
        let mut filename = String::new();
        io::stdin().read_line(&mut filename).expect("not a valid input. (please dont use emojis or etc)");
        let filename = filename.trim();
        let mut args = String::new();
        let mut file = fs::File::open(filename).unwrap();
        file.read_to_string(&mut args).unwrap();
        dbg!(file);
        let input = args.trim();

        let input: Vec<char> = input.chars().collect();
        let mut to_lex = Lexer { text: input, pos: 0, current_char: ' ' };
        let to_lex2 = to_lex.make_tokens();
        println!("{:?}",to_lex2);
    } else if user_input == "1" {
        let mut i = 0;
        loop {
            println!("\nWelcome to loxable shell, alpha version. commands executed so far: {}",i);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("not a valid ascii string");
            let input = input.trim();
            let input: Vec<char> = input.chars().collect();
            let mut to_lex = Lexer { text: input, pos: 0, current_char: ' '  };
            let to_lex2 = to_lex.make_tokens();
            println!("{:?}",to_lex2);
            i += 1;
            println!("\n press ctrl+c to stop, or cmd+c to stop program");
        }
    }
}
