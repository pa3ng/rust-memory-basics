fn main() {
    let mut a = 0;
    let mut b = a;

    println!("a: {}", a);
    println!("b: {}", b);

    a += 42;
    b += 66;

    println!("a after a += 42: {}", a);
    println!("b after b += 66: {}", b);
}
