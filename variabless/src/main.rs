fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("This is the value of inner scope of x: {x}");
    }
    println!("This is the value of x:{x}")
}
