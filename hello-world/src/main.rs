use std::fmt;

#[derive(Debug)]
struct Value(i32);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let values = &self.0;

        write!(f, "[")?;
        for (index, value) in values.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}", index, value)?;
        }
        write!(f, "]")
    }
}

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
    println!("Here's a display representation of the same struct: {}", value);

    let values = List(vec![1, 4, 9, 16]);
    println!("Here's a display representation of a list: {}", values);
}
