mod example;

#[cfg(test)]
mod tests {
    use crate::example;

    #[test]
    fn example_test() {
        example::pattern_match();
        example::basic_enum();
        example::enum_state();
        example::enum_method();
    }

    #[test]
    fn start() {}
}