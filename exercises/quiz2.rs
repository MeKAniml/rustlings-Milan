// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    ///In the test we see that the input is a vector of tuples, the first element is a string and the second one is a command based on the enum Command
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        //as the output is a vector of strings we can declare it as a vector of strings
        let mut output: Vec<String>  = vec![];
        //we iterate over the input vector of tuples (and we remember that iter let the input vector be immutable, so we can't change it, as we will transform it in outpu)
        for (string, command) in input.iter() {
            //as we have different command, we will use a match expression to process the different command variants(as seen in previous exercises)
            match command {
                //as general, syntax is output.push as its the method to add in the vec collection, and for the append we use the string.clone() to avoid moving the string, then &"bar", the & is because we want a string and not a slice for the repeat method, and the *n to dereference the usize (as it's a borrow)
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().to_string()),
                Command::Append(n) => output.push(string.clone() + &"bar".repeat(*n)),
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    //the syntax is first super to go to the superior scope (outside of the module tests), then the module name, and then the function name of the wanted module
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
