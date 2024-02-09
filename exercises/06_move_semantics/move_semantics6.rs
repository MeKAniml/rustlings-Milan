// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

///on this code we can see a case of borrowing and lifetime.
/// Get char using data is doing a borrowing of data var, so string uppercase is not allowed to borrow it even as a reference
/// we swap for get char to reference data in the function, so string uppercase can use data with issue of borrowing.
/// to remember, when a var is use in a function, it's borrowed, and as the function don't end, you can't use the var anywhere else.
fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
