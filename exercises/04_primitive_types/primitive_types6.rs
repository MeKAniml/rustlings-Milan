// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.


///In this code we saw how to use a test with the call #[test] and the use of asser_eq! test function that take 3 args
/// 1st one is the value wanted , 2nd the variable to test equality  with the 1st value and Last one is the error message in case of a failed test
/// the notion seing here is how to access a specific value of a tuple, the index method where you call the tuple name follow by a dot and the index of the wanted value of the tuple


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
