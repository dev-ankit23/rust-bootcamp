struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Ankit"),
        email: String::from("ankit@gmail.com"),
        sign_in_count: 1,
    };
    println!("user name is: {:?}", user1.username);
    println!("user email is: {:?}", user1.email);
    println!("user is: {:?}", user1.active);
    println!("user have signin : {:?}", user1.sign_in_count)
}
