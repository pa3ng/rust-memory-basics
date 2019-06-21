# Understanding ownership, moves, copies, and borrows

## Ownership

The concept is simple: any piece of memory can only be owned by a single owner. When that owner goes out of scope (is "dropped" in Rust lingo), the memory is automatically freed. From [the book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html):

* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## Moves

If you assign a piece of memory to a different variable, Rust reassigns ownership of the memory to the new variable. In Rust terminology, a _move_ occurs. By the ownership rule, the original owner no longer owns it. For example:

```rust
let a = String::from("ownership"); // <- a owns "ownership"
let b = a; // <- b now owns "ownership"; a is "dead" and can no longer be used
```

See [0_move_string.rs](0_move_string.rs). Note the `Copy` in the compile error:

```
error[E0382]: use of moved value: `a`
 --> move_string.rs:5:20
  |
3 |     let b = a;
  |         - value moved here
4 | 
5 |     println!("{}", a);
  |                    ^ value used here after move
  |
  = note: move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
  ```

## Copies

What happens when we move an integer?:

```rust
let a = 0; // <- a owns the value 0
let b = a; // <- b gets a copy of a; a is still valid
```

This compiles (see [1_move_int.rs](1_move_int.rs)). This works because primitive types implement the [Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait (see also [Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)). (What is a trait? A Java developer will recognize it as an interface, [rust-by-example: traits](https://doc.rust-lang.org/rust-by-example/trait.html) provides a simple example.)

The difference between a _move_ and a _copy_ explains why our concurrency attempt produced `0`: the `move` resulted in a copy, each of our 10 threads incremented its own counter to 1,000,000 and the original counter was left untouched.

## Borrows

You can _borrow_ from an owner using the `&` syntax:

```rust
let a = String::from("owner") // <- a owns "owner"
let b = &a; // <- b creates a reference to a, borrows it
```

The above example borrows `a` _immutably_ (`a` itself is immutible, so a mutable borrow is disallowed). `b` is also called a `reference` See [2_immutable_borrow.rs](2_immutable_borrow.rs) for a more comprehensive example. You can also _mutably_ borrow:

```rust
let mut a = String::from("yay"); // <- a is mutable, owns "yay"
let b = &mut a; // <- b mutably borrows from a

b.push_str("!!!"); // <- b is allowed to modify a
```

See [3_mutable_borrow.rs](3_mutable_borrow.rs) for an example mutable borrow. Try removing the `{}`s around the borrowing logic to see why they're there.

The rules for borrowing are fairly simple:

* At any given time, you can have **one** mutable reference (assuming the owner is mutable) ___or___ any number of immutable references
* A mutable reference _freezes_ the owner (see [4_mutable_borrow_freezes_owner.rs](4_mutable_borrow_freezes_owner.rs))
* A reference may not outlive its owner (the owner is always dropped when it goes out of scope; in other words, a reference must always have smaller or equal scope to the owner; see [5_borrow_outlives_owner.rs](5_borrow_outlives_owner.rs)).

## What this has to do with memory

Rust does not use garbage collection. Yet, you neither see `malloc` nor do you see `free`. How does this work?

1. Rust allocates memory for you: `String::from("hello")` will automatically allocate the space to store `"hello"`.
2. An object -- or piece of memory -- must have one and only one owner.
3. Borrowing allows an object to be shared and the rules of borrowing ensure that sharing is _safe_. You will become very familiar with the "borrow checker".
4. When the owner goes out of scope, its memory is freed ("dropped" in Rust terminology); Rust guarantees nobody is borrowing the object when it would be dropped.
