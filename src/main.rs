use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");
   
    let stdout = stdout();
    let message = String::from("Hello rs bro");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    rs();
}

fn rs(){
    println!("rohan shukla or rs?");
}
