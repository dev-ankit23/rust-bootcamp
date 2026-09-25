 
pub fn main() {
    let v = vec![1, 2, 3];
    let v2 = vec![String::from("Harkirat"), String::from("Singh")];
    let v3 = vec![1.0, 2.0, 3.0];
    println!("{}", first_element(v).unwrap());
    println!("{}", first_element(v2).unwrap());
    println!("{}", first_element(v3).unwrap());
}

fn first_element<T>(v: Vec<T>) -> Option<T> {
    return v.into_iter().nth(0);
}