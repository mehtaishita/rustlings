// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        self + "Bar"
    }
}

impl AppendBar for u64 {
    fn append_bar(self) -> Self {
        self + self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);

    let i = 34.append_bar();
    println!("i {}", i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

    #[test]
    fn is_int_double() {
        assert_eq!(
            68,
            34.append_bar()
        );
    }
}
