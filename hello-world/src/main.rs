#[derive(Debug)]
struct Value(i32);

fn main() {
    println!("Hello world!");
    println!("Here's a new line!");

    let phrase = "Here's a phrase from another variable.";
    println!("{}", phrase);

    println!("Here are arguments referenced {0} and {1}, then flipped as {1} and {0}.", "first", "second");

    println!("Here are named arguments: {one}, {two}, and {three}.", one = "first", two = "second", three = "third");

    let value = Value(42);
    println!("Here's a debug representation of a struct: {:?}", value);
    println!("Here's a pretty debug representation of the same struct: {:#?}", value);
}
