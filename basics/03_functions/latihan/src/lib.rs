#[cfg(test)]
mod tests {

    fn multiply(a: i32, b: i32) -> i32 {
        todo!()
    }

    #[test]
    fn start() {
        let a = 10;
        let b = 20;

        let mult = multiply(a, b);

        assert_eq!(200, mult);
    }
}
