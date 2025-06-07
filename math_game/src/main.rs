use std::io;
use rand::Rng;



fn main() {

    println!("Welcome to the math game");

    let mut game_running = true;

    while game_running {
        
        println!("User:");
        let mut eingabe = String::new();
        
        let result = random_aufgabe();


        // println!("{result}");

        io::stdin()
            .read_line(&mut eingabe)
            .expect("Failed to readline");
        
        
        let eingabe: i32 = match eingabe.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if eingabe == 69 {
            game_running = false;
        }       


        prüfe_ergebnis(eingabe,result);

        


    }


}

fn random_aufgabe() -> i32 {
        let seed: u32 = rand::thread_rng().gen_range(0..=1);

        if seed == 0 {
            let result = plus();
            println!("Result: {}",result);
            result

        }else {
            let result = minus();
            println!("Result: {}",result);
            result
        }



}

fn prüfe_ergebnis(eingabe: i32,result: i32) {
    // DEBUG: println!("Der Benutzer hat eingegeben: {}",eingabe);
    // DEBUG: println!("Die Antwort war: {}",result);
    
    if eingabe != 69 {

        if eingabe == result {
            println!("Richtig!");
        }else {
            println!("Falsch!");
        }

    }else {
        println!("Bye");
    }

}

fn plus() -> i32 {
    
    const OPERATOR: char = '+';
    let zahl_1 = rand::thread_rng().gen_range(1..=10);
    let zahl_2 = rand::thread_rng().gen_range(1..=10);

    println!("{zahl_1} {OPERATOR} {zahl_2}");

    zahl_1 + zahl_2

}

fn minus() -> i32 {

    const OPERATOR: char = '-';
    let zahl_1 = rand::thread_rng().gen_range(1..=10);
    let zahl_2 = rand::thread_rng().gen_range(1..=10);

    println!("{zahl_1} {OPERATOR} {zahl_2}");

    zahl_1 - zahl_2

}
