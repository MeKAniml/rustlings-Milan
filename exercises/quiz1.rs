// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)


// Put your function here!

///As stated in the name, the price of apple change if you have more than 40 it's 1 , if not Z
/// So to simplify atmost, we do a simple if that return the number by the define price if more than 40
/// it's not the most easy to read and modify 
fn calculate_price_of_apples (AppleNumber:i32) -> i32 {

    if AppleNumber > 40 {
        AppleNumber * 1
    }else {
        AppleNumber * 2
    }
}
// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
