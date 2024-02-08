// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

///This function is use to calculate a square, in rust to return a simple var / type you don't need to specify the 'return' keyword but for this one as we want to return the multiplication we need to specify return (or store it in a different var)
fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}
