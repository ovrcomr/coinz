use std::io;

pub struct Coin {
    pub name: String,
    pub price: f64,
}

impl Coin {
    pub fn new() -> Coin {
        let mut coin_name: String = String::new();
        let mut coin_price: String = String::new();

        io::stdin()
            .read_line(&mut coin_name)
            .expect("Failed to read coin_name");
        io::stdin()
            .read_line(&mut coin_price)
            .expect("Failed to read coin_price");

        let coin_name: String = coin_name.trim().to_string();
        let coin_price: f64 = coin_price
            .trim()
            .parse()
            .expect("Failed to parse coin_price");

        Coin {
            name: coin_name,
            price: coin_price,
        }
    }

    pub fn info(&self) {
        println!("Coin name : {}", self.name);
        println!("Coin price : {}", self.price);
    }
}
