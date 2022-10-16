// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.



fn trim_me(input: &str) -> String {
    return String::from(input.trim());
}

fn compose_me(input: &str) -> String {
    input.to_owned() + " world!" // can't be used to concatenate &str + &str, that's why need the to_owned()

}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons") // replace returns a String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
