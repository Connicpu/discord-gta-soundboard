extern crate discord;
extern crate rpassword;
extern crate rustc_serialize;

use std::sync::Arc;
use std::io::BufRead;

mod login;
mod state;
mod server;

fn main() {
    let discord: discord::Discord = match login::login() {
        Some(d) => d,
        None => return,
    };
    let discord = Arc::new(discord);
    let state = state::StateData::load();

    let (connection, ready) = discord.connect().expect("Connection failed");
    println!("[Ready] {} is serving {} servers", ready.user.username, ready.servers.len());

    let bot_state = state::Bot {
        discord: discord.clone(),
        connection: connection,
        ready: ready.clone(),
    };

    let _loop_handle = server::run_server(bot_state, state.clone());

    let stdin = std::io::stdin();
    for command in stdin.lock().lines() {
        let command = command.expect("Stdin failed");
        let split: Vec<_> = command.split(' ').collect();
        match split[0] {
            "add" => {
                let key = split[1].into();
                let path = split[2].into();
                println!("Adding {} => {}", key, path);
                let mut lock = state.write().unwrap();
                lock.musical_treats.insert(key, path);
                lock.save().unwrap();
            },
            _ => println!("Unknown command"),
        }
    }
}
