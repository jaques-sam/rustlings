// traits2.rs
//
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    // see here why the `mut` keyword is allowed here
    // https://stackoverflow.com/questions/66700509/why-is-mut-self-in-trait-implementation-allowed-when-trait-definition-only-uses
    // "The caller does not need to know about it:
    //  you move the value one way or the other,
    //  so it's up to the callee if it needs to mutate it or not.""
    fn append_bar(mut self) -> Self {
        self.push("Bar".into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
