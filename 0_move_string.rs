fn main() {
    let a = String::from("ownership test");
    let b = a;

    println!("{}", a);
    println!("{}", b);
}
