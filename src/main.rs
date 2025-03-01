pub use coinz::coin::feature::Coin;
use std::io;

fn main() {
    let mut coin_list: Vec<Coin> = Vec::new();

    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if input == "/test" {
            println!("Running...");
        }

        if input == "/new coin" {
            let new_coin: Coin = Coin::new();
            coin_list.push(new_coin);
        }

        if input == "/coin list" {
            for coin in &coin_list {
                println!("{} : {}", coin.name, coin.price);
            }
        }

        if input == "/exit" {
            println!("Exiting...");
            break;
        }
    }
}
