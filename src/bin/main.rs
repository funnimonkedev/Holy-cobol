use std::io;
use loxable::Lexer;
// use std::{fs, io, io::Read};

fn main() {
    /*
    println!("welcome to loxable shell, please input file name.");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("not a valid input. (please dont use emojis or etc)");
    let filename = filename.trim();
    let mut args = String::new();
    let mut file = fs::File::open(filename).unwrap();
    file.read_to_string(&mut args).unwrap();
    dbg!(file);
    let args = args.trim();
    // println!("would you like to use untested/unstable version? (y/n)   \n");
    println!("{}",args);

    if args.starts_with("loxabla beta/") {
        println!("version type declared succesfully.");
    } else if !args.starts_with("loxable beta/") {
        panic!("current version of loxable is unavailable in this version, please upgrade your loxable version.");
    }
    */
    let mut i = 0;
    loop {
        println!("\nwelcome to loxable shell, alpha version. commands executed so far: {}",i);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("not a valid ascii string");
        let input = input.trim();
        let input: Vec<char> = input.chars().collect();
        let mut to_lex = Lexer { text: input, pos: 0, current_char: ' '  };
        let to_lex2 = to_lex.make_tokens();
        println!("{:#?}",to_lex2);
        i += 1;
    }
}