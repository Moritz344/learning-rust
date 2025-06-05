use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {

    println!("Type a Number:");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // guess var in u32 umwandeln/Wenn ein str eingegeben
        // wird dann continue
        // parse gibt ein Result type zurück also Ok und Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        

        println!("You guessed: {guess}"); // man kann auch das machen: "{}",guess
    

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }


    }



}
