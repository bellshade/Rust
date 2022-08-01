#[cfg(test)]
mod tests {

    fn greet(name: String) -> String {
        format!("Selamat Datang di {}", name)
    }
    
    fn greet_2(name: String) -> String {
        format!("Semoga mendapat ilmu di {}", name)
    }    

    #[test]
    fn start() {
        let a = String::from("Bellshade");

        assert_eq!("Selamat Datang di Bellshade", greet(a));
    
        assert_eq!("Semoga mendapat ilmu di Bellshade", greet_2(a));    
    }
}
