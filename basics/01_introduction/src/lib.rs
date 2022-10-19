#[cfg(test)]
mod tests {
    #[test]
    fn intro() {
        let a = "Bellshade!";

        println!("Hello World!");
        println!("Hello {}", a);

        assert_eq!("Bellshade!", a);
    }
}
