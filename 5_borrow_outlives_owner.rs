fn main() {
    let b;

    {
        let a = String::from("hello");
        b = &a;
    }

    println!("b = {}", b);
}
