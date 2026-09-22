struct User {
    name: &str,
}
fn main() {
    let first_name = String::from("Ankit");
    let user = User { name: &first_name };
    println!("The name of the user is:- ", user.name)
}
