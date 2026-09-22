fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let vec1 = vec.iter();

    let vec2 = vec1.filter(|x| *x % 2 == 1);

    let doubleoddvec = vec2.map(|x| (*x, *x * 2));

    for (odd, doubled) in doubleoddvec {
        println!("Odd: {}", odd);
        println!("Doubled: {}", doubled);
    }
}
