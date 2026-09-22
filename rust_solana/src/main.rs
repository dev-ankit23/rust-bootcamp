fn main() {
    let s = String::from("Ankittrewytgre4tre");
    println!("{}", is_longer_than(s, 10));
}

fn is_longer_than(s: String, num: usize) -> bool {
    if s.len() > num {
        return true;
    } else {
        return false;
    }
}
