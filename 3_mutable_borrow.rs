fn main() {
    // a is mutable
    let mut a = String::from("yay");

    println!("a = {}", a);

    {
        // b borrows a mutably
        let b = &mut a;

        // b can modify a
        b.push_str("!!!");

        println!("b = {}", b);
    }

    println!("a now = {}", a);
}
