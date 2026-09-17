fn main() {
    let vec = vec![1, 2, 4, 6, 7, 8, 9, 5];
    let vec1 = vec.iter();
    let vec2 = vec1.filter(|x| *x % 2 == 1);

    for x in vec2 {
        println!("{}", x)
    }
}
