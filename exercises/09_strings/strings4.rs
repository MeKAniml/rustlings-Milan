// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}
///under each call we will explain why we chose string or string_slice
fn main() {
    string_slice("blue");
    //it's a string slice because it's a string literal
    string("red".to_string());
    //it's a string because it's a string literal that's been converted to a string
    string(String::from("hi"));
    //it's a string because we used the String::from method to create a string
    string("rust is fun!".to_owned());
    //it's a string because we used the to_owned method that returns a string 
    string_slice("nice weather".into());
    //it's a string slice because we used the into method that returns a string slice, this method is used to convert a value to a string slice
    string(format!("Interpolation {}", "Station"));
    //it's a string because we used the format! macro that returns a string
    string_slice(&String::from("abc")[0..1]);
    //it's a string slice because we used the & operator to get a reference to the string and then used the slicing method to get a string slice
    string_slice("  hello there ".trim());
    //it's a string slice because we used the trim method that returns a string slice
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    //it's a string because we used the to_string method to convert a string literal to a string and then used the replace method that returns a string
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
    //it's a string because we used the to_lowercase method that returns a string
}
