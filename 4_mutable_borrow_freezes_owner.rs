fn main() {
    let mut a = String::from("This won't compile...");

    let b = &mut a;

    b.push_str(" Or will it?");
    a.push_str(" Any bets?");
}
