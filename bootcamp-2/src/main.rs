fn get_string_len(str: String) -> usize {
    str.chars().count()
}
fn main() {
    let name = String::from("Ankit Maurya");
    let len = get_string_len(name);
    println!("The len of the string is:- {}", len)
}
