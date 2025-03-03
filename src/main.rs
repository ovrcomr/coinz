use coinz::coin::feature::Coin;
use coinz::server;
use coinz::user::feature::{display_user_list, user_regis, User};
use std::env;

use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <server|client> <address>", args[0]);
        return;
    }

    let role = &args[1];
    let address = &args[2];

    match role.as_str() {
        "server" => server::server::start_server(address),
        "client" => server::client::start_client(address),
        _ => eprintln!("Unknown role: {}", role),
    }

    let mut coin_list: Vec<Coin> = Vec::new();
    let mut user_list: Vec<User> = Vec::new();

    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        match input {
            "/test" => {
                println!("Running...");
            }
            "/new coin" => {
                let new_coin: Coin = Coin::new();
                coin_list.push(new_coin);
            }
            "/coin list" => {
                for coin in &coin_list {
                    println!("{} : {}", coin.name, coin.price);
                }
            }
            "/exit" => {
                println!("Exiting...");
                break;
            }

            "/register" => {
                user_regis(&mut user_list);
            }

            "/user list" => {
                display_user_list(&mut user_list);
            }

            _ => {
                // This is the "catch-all" case if no match is found
                println!("Unknown command");
            }
        }
    }
}
