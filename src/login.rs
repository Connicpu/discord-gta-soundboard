use discord::Discord;
use rpassword::read_password;
use std::{io, env};
use std::io::Write;

pub fn login() -> Option<Discord> {
    match env::var("GTA_BOT_TOKEN").map(|token| Discord::from_bot_token(&token)) {
        Ok(Ok(discord)) => return Some(discord),
        Ok(Err(_)) => println!("GTA_BOT_TOKEN is invalid"),
        _ => println!("Env var GTA_BOT_TOKEN not found, loggin in with email/password"),
    }

    let email = match env::var("GTA_BOT_EMAIL") {
        Ok(email) => email,
        _ => {
            println!("Env var GTA_BOT_EMAIL was not found.");
            println!("You should consider specifying it, but please enter it now.");
            print!("Email: "); io::stdout().flush().unwrap();

            let mut email = String::new();
            match io::stdin().read_line(&mut email) {
                Ok(_) => {},
                Err(_) => return None,
            }
            email.trim().into()
        }
    };

    let password = match Discord::new_cache("discord-tokens.txt", &email, None) {
        Ok(discord) => return Some(discord),
        Err(_) => {
            println!("You haven't signed into this bot email yet.");
            println!("Please enter your password (it will not be saved)");
            print!("Passowrd: "); io::stdout().flush().unwrap();

            read_password().unwrap()
        } 
    };

    match Discord::new_cache("discord-tokens.txt", &email, Some(&password)) {
        Ok(discord) => return Some(discord),
        Err(e) => {
            println!("Login failed.");
            println!("{:?}", e);
        }
    }

    None
}
