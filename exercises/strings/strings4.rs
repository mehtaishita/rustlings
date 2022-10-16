// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string()); // String
    string(String::from("hi")); // String
    string("rust is fun!".to_owned()); //String (to_owend())
    string_slice("nice weather".into()); // str because of into
    string(format!("Interpolation {}", "Station")); // format => String
    string_slice(&String::from("abc")[0..1]); // despite the 'fron' and the '&', it's still str because of taking a range of letters
    string_slice("  hello there ".trim()); // trim keeps it as a str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // replace => String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // to_lowercase also String
}
