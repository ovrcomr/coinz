use std::io::{self, Write};

pub struct User {
    username: String,
    password: String,
}

pub fn new(username: String, password: String) -> User {
    User { username, password }
}

pub fn display_user_list(user_list: &mut Vec<User>) {
    for user in user_list {
        println!("{} : {}", user.username, user.password);
    }
}

pub fn user_regis(user_list: &mut Vec<User>) {
    let mut username: String = String::new();
    let mut password: String = String::new();

    print!("Choose username : ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut username)
        .expect("Fail to read username for register");

    print!("Choose password : ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut password)
        .expect("Fail to read password for register");

    let username = username.trim().to_string();
    let password = password.trim().to_string();

    let new_user = new(username, password);
    user_list.push(new_user);
}
