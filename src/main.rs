use std::io;

fn main() {
    println!("Hello!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("");
}

