#[cfg(test)]
mod tests {
    // untuk error handling di rust menggunakan result sebagai hasil dari suatu proses
    // result memiliki 2 kondisi utama yaitu Ok = hasil, dan Err = error

    fn get_ok() -> Result<String, String> {
        // untuk mengembalikan hasil, dengan keyword Ok(value)
        Ok("ini result".to_string())
    }

    fn get_err() -> Result<String, String> {
        // untuk mengembalikan error, dengan keyword Err(value)
        Err("ini error".to_string())
    }

    #[test]
    fn result_types() {
        // memanggil function untuk return result terlebih dahulu
        let result = get_ok();
        // memastikan hasil yang di return Ok
        assert!(result.is_ok());
        // simulasi Ok pattern matching untuk tipe data Result
        match result {
            Ok(result) => println!("Hasil Ok: {}", result),
            Err(e) => println!("Hasil Err: {}", e),
        }

        // sama seperti di atas, namun kali ini simulasi error
        let error = get_err();
        // memastikan hasil yang di return Err
        assert!(error.is_err());
        // simulasi Err pattern matching untuk tipe data Result
        match error {
            Ok(result) => println!("Hasil Ok: {}", result),
            Err(e) => println!("Hasil Err: {}", e),
        }

        // syntax function dengan pattern matching bisa di persingkat menjadi
        match get_ok() {
            Ok(result) => println!("Hasil dipersingkat Ok: {}", result),
            Err(e) => println!("Hasil Err: {}", e),
        }

        // syntax function dengan pattern matching bisa di persingkat menjadi
        match get_ok() {
            // disini kita bisa menambahkan wildcard (_) untuk mengabaikan hasil yang tidak digunakan
            // namun ini tidak disarankan, lebih baik di match secara explicit
            Ok(result) => println!("Hasil dengan wildcard Ok: {}", result),
            _ => (),
        }
    }

    /// bahasa pemrograman rust tidak memiliki nilai Null atau Nil seperti bahasa lainnya
    /// rust memiliki fitur option untuk mencegah null value yang berpotensi bug
    /// option memiliki 2 kondisi utama yaitu Some = ada value, dan None = tidak ada value

    fn get_some() -> Option<String> {
        // Some di gunakan untuk mengembalikan Option dengan value
        Some("hasil dengan value".to_string())
    }

    fn get_none() -> Option<String> {
        // None di gunakan untuk mengembalikan Option tanpa value
        None
    }

    #[test]
    fn option_types() {
        // memanggil function untuk return option terlebih dahulu
        let option = get_some();
        // memastikan return some value
        assert!(option.is_some());
        // simulasi Some di pattern matching
        match option {
            Some(result) => println!("Hasil Some: {}", result),
            None => println!("Hasil None"),
        }

        // memanggil function untuk return option terlebih dahulu
        let option = get_none();
        // memastikan return none value
        assert!(option.is_none());
        // simulasi None di pattern matching
        match option {
            Some(result) => println!("Hasil Some: {}", result),
            None => println!("Hasil None"),
        }

        // persingkat pattern matching dengan function
        match get_some() {
            Some(result) => println!("Hasil persingkat Some: {}", result),
            None => println!("Hasil None"),
        }

        // kita bisa melakukan unwrap yaitu mengambil value di dalam type option ini
        // cara ini tidak disarankan, karena juga function return none akan menyebabkan panic
        let hasil_option = get_none();
        assert!(hasil_option.is_none());
        // kita bisa menggunakan default value, sangat disarankan sewaktu waktu hasil nya none, agar tidak panic
        let hasil = hasil_option.unwrap_or("ini hasil default".to_string());
        // kode dibawah ini akan meyebabkan error, jangan unwrap begitu saja pada option!
        // let hasil = hasil_option.unwrap();
        println!("Hasil unwrap dengan default: {}", hasil);

        let hasil_kondisi = get_some();
        // if ini akan jalan jika di temui value dari option tersebut
        if let Some(result) = hasil_kondisi {
            // variable result yand di deklarasi sebagai some hanya bisa di akses di dalam if block saja
            println!("kondisi result true: {}", result);
        }

        // bisa di persingkat & simulasi jika value adalah none
        if let Some(result) = get_none() {
            // ini tidak akan jalan karna false!
            // variable result mengharapkan some value dari function, namun function mengembalikan none.
            println!("kondisi None & false: {}", result);
        }
    }
}
