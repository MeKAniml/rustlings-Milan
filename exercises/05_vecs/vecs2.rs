// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

///The hint asked us wich pattern is more use between rust dev, i would like to take a guess and 
/// think that is the iter + map, even though more long, it assure us that the original vec is keep untouched, so no errors could happen on the data it contain,
/// and if we need to use it so we create a usable double , keeping immutability ?
///in this code we see two iteration of a vec object.
///the first one with iter_mut, that make a "element" mutable reference to the vec content. normal iter don't create mutable reference so you will not be able to modify the vec content
///for multiply by too each element, we need to first deference with * element to access the content and then multiply by 2. I choose *= to simplify the code


fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}
/// For this one, we do an iter then a map. as previously said iter don't create mutable reference of the content of the vec, so we can view and access but not modify
/// then we apply a map method on every non mutable reference, that for every |X| apply a {fonction } , here for every element, we will multiply by 2.
/// the result is then collect and return as a new vec with the .collect method
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
