use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    users.insert(String::from("Ankit"), 19);
    users.insert(String::from("Arnav"), 19);
    users.insert(String::from("Bipin"), 19);

    let user1: Option<&i32> = users.get("Bipin");

    println!("{}", user1.unwrap())
}
