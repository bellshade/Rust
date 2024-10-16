mod example;

#[cfg(test)]
mod tests {
    use crate::example;

    #[test]
    fn example_test() {
        example::result_types();
        example::option_types();
    }

    #[test]
    fn start() {}
}