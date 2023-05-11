use std::{fs, io, io::Read};
//use loxable::{Operator};

fn main() {
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
}