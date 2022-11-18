use std::io;


fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new(); // Criando uma varíavel mutável que irá armazenar uma nova instância vazia do tipo String.

    io::stdin()
       .read_line(&mut guess)  // Precisamos colocar "&mut guess" por ser uma refência mutável. 
       .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
