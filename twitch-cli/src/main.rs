use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;

use std::io;

use dotenv::dotenv;
use std::env;

fn user_login_name() -> String {
    loop {

        println!("Please enter your Twitch account name:");
        println!("");

        let mut login_name = String::new();

        io::stdin()
            .read_line(&mut login_name)
            .expect("Failed to read line");

        let mut login_name = login_name.trim();

        if !login_name.is_empty() {
            break login_name.to_string();
        }else {
            println!("Enter a valid Account name please: ");
            println!("");
        }
    }


}

fn user_input() -> String {

    println!("Please enter a streamer name: ");
    println!("");
    loop {


        let mut eingabe = String::new();

        io::stdin()
            .read_line(&mut eingabe)
            .expect("Failed to read line");
        


        let eingabe = eingabe.trim().to_lowercase();

        if !eingabe.is_empty() {
            println!("Connecting ...");
            break eingabe.to_string();

        }else {
            println!("Please type a valid name");
            println!("");
        }




    }
    
}


#[tokio::main]
async fn main() {
    
    // Lädt die .env Datei
    dotenv().ok();

    let oauth_token= env::var("TOKEN").expect("TOKEN not set in .env");
    

    let login_name = user_login_name();
    let streamer = user_input(); 
     
    // &str ist ein verweis und muss mit .to_owned in String umgewandelt werden
    let login_name = login_name.to_owned();


    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(login_name,Some(oauth_token))
    );

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    client.join(streamer.to_owned()).unwrap();
    
    while let Some(message) = incoming_messages.recv().await {
        match message {
            ServerMessage::Privmsg(msg) => {
                println!("({}) | {} : {}", msg.channel_login,msg.sender.name,msg.message_text);
            },
            _ => {}
        }
    }



}


