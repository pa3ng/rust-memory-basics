fn main() {
    // a is immutable
    let a = String::from("borrow test");

    // a can have any number of immutable borrowers
    let b = &a;
    let c = &a;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    // a is mutable
    let mut a = String::from("borrow test");

    // a can still be borrowed immutably:
    let b = &a;

    println!("{}", a);
    println!("{}", b);
}
