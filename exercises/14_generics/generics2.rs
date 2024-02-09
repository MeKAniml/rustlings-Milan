// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

/// <T> is used to define a generic type. This means that the Wrapper struct can be used to wrap any type of value, not just u32.
/// the <T> in rust is mean to be generic type parameter that permit us to be more flexible to the type of the value that we want to store in the Wrapper struct
struct Wrapper<T> {
    value: T,
}

///we need to respecify the <T> after the impl keyword to specify that the implementation for the Wrapper struct will apply to any type of value
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
