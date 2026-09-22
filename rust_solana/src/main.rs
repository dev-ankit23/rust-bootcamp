struct User {
    name: String,
    age: i32,
    gender: Gender,
}
enum Gender {
    Male,
    Female,
}

fn main() {
    let user1 = User {
        name: String::from("Ankit"),
        age: 19,
        gender: Gender::Male,
    };
    let user2 = User {
        name: String::from("ankit2"),
        age: 17,
        gender: Gender::Female,
    };
    println!("{}", is_allowed_voitng(user1));
    println!("{}", is_allowed_voitng(user2));
}
fn is_allowed_voitng(u: User) -> bool {
    if u.age >= 18 {
        return true;
    } else {
        return false;
    }
}
