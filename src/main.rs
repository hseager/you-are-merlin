use std::io;

fn main() {
    println!("You are Merlin.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Your input: {input}");
}
