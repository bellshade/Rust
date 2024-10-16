#[cfg(test)]
mod tests {
    // basic pattern matching
    #[test]
    fn pattern_match() {
        let kata = "bakso";
        // pattern matching string
        match kata {
            "bakso" => println!("bakso"),
            "burger" => println!("burger"),
            _ => println!("pilihan selain di atas"),
        }

        let angka = 100;
        // pattern matching angka, bisa integer, float, dll
        match angka {
            100 => println!("ini angka 100"),
            200 => println!("ini angka 200"),
            _ => println!("pilihan selain di atas"),
        }

        let boolean = true;
        // pattern matching boolean
        match boolean {
            true => println!("ini true"),
            false => println!("ini false"),
        }
    }
    // setelah memahami pattern matching
    // kita masuk ke enum type
    enum Animal {
        Dog,
        Cat,
        Fish
    }
    // penggunaan dasar enum
    #[test]
    fn basic_enum() {
        // buat variable dengan enum type dog
        let dog = Animal::Dog;
        // pattern match enum
        // masing masing pattern return string
        let result = match dog {
            Animal::Dog => "dog".to_string(),
            Animal::Cat => "cat".to_string(),
            Animal::Fish => "fish".to_string(),
        };
        // cek jika return sesuai pattern match
        assert_eq!(result, "dog");

        let cat = Animal::Cat;
        // pattern match cat
        let result = match cat {
            Animal::Dog => "dog".to_string(),
            Animal::Cat => "cat".to_string(),
            Animal::Fish => "fish".to_string(),
        };
        // cek jika return sesuai pattern match
        assert_eq!(result, "cat");

        let fish = Animal::Fish;
        // pattern match cat
        let result = match fish {
            Animal::Dog => "dog".to_string(),
            Animal::Cat => "cat".to_string(),
            Animal::Fish => "fish".to_string(),
        };
        // cek jika return sesuai pattern match
        assert_eq!(result, "fish");
    }
    // setelah memahami enum
    // enum bisa menampung suatu nilai dengan type yang di tentukan
    // ini bisa di sebut sebagai enum states
    enum Person {
        Budi(String),
        Andi(String, i8),
        Anton(String, i8, f64),
    }
    // penggunaan enum states
    #[test]
    fn enum_state() {
        // deklarasi budi dengan enum states yang telah di definisikan
        let budi = Person::Budi("polisi".to_string());
        // pattern match budi
        let pekerjaan_budi = match budi {
            Person::Budi(pekerjaan) => {
                println!("pekerjaan budi adalah: {}", pekerjaan);
                pekerjaan
            },
            Person::Andi(pekerjaan, umur) => pekerjaan,
            Person::Anton(pekerjaan, umur, gaji) => pekerjaan,
        };
        // validasi pekerjaan budi
        assert_eq!(pekerjaan_budi, "polisi");

        // deklarasi andi dengan enum states yang telah di definisikan
        let andi = Person::Andi("programmer".to_string(), 25);
        // pattern match andi
        let umur_andi = match andi {
            Person::Andi(pekerjaan, umur) => {
                println!("umur andi adalah: {}", umur);
                umur
            },
            Person::Anton(pekerjaan, umur, gaji) => umur,
            _ => 0
        };
        // validasi umut andi
        assert_eq!(umur_andi, 25);

        // deklarasi anton dengan enum states yang telah di definisikan
        let anton = Person::Anton("dokter".to_string(), 30, 25000000.00);
        // pattern match anton
        let gaji_anton = match anton {
            Person::Anton(pekerjaan, umur, gaji) => {
                println!("gaji anton adalah: {}", gaji);
                gaji
            },
            _ => 0.0
        };
        // validasi gaji anton
        assert_eq!(gaji_anton, 25000000.00);
    }
    // setelah memahami enum states
    // kita bisa membuat enum method dan mengakses states tersebut
    enum Tebakan {
        Kalah(String),
        Menang(String),
    }
    // deklarasi enum method di dalam keyword impl
    impl Tebakan {
        // method dari enum tebakan
        fn tebak(&self) -> String {
            match self {
                Tebakan::Kalah(reason_kalah) => {
                    println!("Tebakan salah: {}", reason_kalah);
                    reason_kalah.to_string()
                }
                Tebakan::Menang(reason_menang) => {
                    println!("Tebakan benar: {}", reason_menang);
                    reason_menang.to_string()
                }
            }
        }
    }
    // penggunaan enum method
    #[test]
    fn enum_method() {
        // buatkan enum type kalah dengan alasan kalah sebagai enum state
        let pilihan = Tebakan::Kalah("ini alasan tebakan mu salah".to_string());
        // panggile enum method
        let alasan = pilihan.tebak();
        // validasi alasan
        assert_eq!(alasan, "ini alasan tebakan mu salah");

        // buatkan enum type menang dengan alasan menang sebagai enum state
        let pilihan = Tebakan::Menang("ini alasan tebakan mu menang".to_string());
        // panggil enum method
        let alasan = pilihan.tebak();
        // validasi alasan
        assert_eq!(alasan, "ini alasan tebakan mu menang");
    }
}
