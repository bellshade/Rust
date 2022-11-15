# Drop Trait

Seperti yang telah kita ketahui, smart pointer adalah sebuah tipe yang mengimplementasikan trait `Deref` dan `Drop`. Kita sudah membahas tentang `Deref` di artikel sebelumnya. Kali ini, kita akan membahas tentang `Drop`.

Trait `Drop` dapat diimplementasikan pada tipe apapun, dan hampir akan selalu digunakan ketika kita mengimplementasikan sebuah smart pointer. Trait `Drop` adalah sebuah trait yang membuat kita dapat mengatur atau mengkustomisasi apa yang akan terjadi bila sebuah nilai keluar dari scope-nya (out of scope). Mari kita ambil `Box<T>` sebagai contoh. Implementasi kustom tentang apa yang akan terjadi ketika sebuah nilai keluar dari scope-nya pada `Box<T>` adalah, ia akan mendealokasikan nilai yang ia tunjuk pada heap.

Sekarang, mari kita gunakan constraint trait `Debug` pada smart pointer `Kotak<T>` kita yang telah kita buat di materi `Deref` dan semua implementasinya agar kita dapat menampilkan value `T` pada smart pointer `Kotak<T>` kita.

```rust
use std::ops::Deref;

struct Kotak<T: std::fmt::Debug>(T);

impl<T> Kotak<T>
    where T: std::fmt::Debug {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for Kotak<T>
    where T: std::fmt::Debug {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

Lalu kita implementasikan `Drop` pada `Kotak<T>` kita.

```rust
impl<T> Drop for Kotak<T>
    where T: std::fmt::Debug {
    fn drop(&mut self) {
        println!("Dropping Kotak yang memiliki data {:?}!", self.0);
    }
}
```

Kita hanya akan menggunakan macro `println!` untuk sekarang, karena sekarang kita hanya akan berfokus pada bagaimana trait `Drop` bekerja, dan tidak pada implementasi kustom aslinya seperti mendealokasikan nilai di heap. Sekarang, mari kita lihat fungsi `main` kita.

```rust
fn main() {
	let x = Kotak::new(20);
	{
		let y = Kotak::new("Halo");
	}	
}
```

Bila kita compile lalu kita jalankan kode kita, maka kita akan menerima output seperti ini:

```
Dropping Kotak yang memiliki data "Halo"!
Dropping Kotak yang memiliki data 20!
```

Lihat, begitu variabel `Kotak<T>` kita menyentuh akhir dari scope, maka apa yang ada di dalam method drop kita akan terpanggil. Diatas, variabel `x` akan di-drop ketika ia menyentuh akhir dari scope `main` sehingga fungsi `y` akan lebih dahulu di-drop. Karena itulah output `Dropping Kotak yang memiliki data "Halo"!` keluar lebih dahulu. Selalu ingat bahwa trait `Drop` ini digunakan untuk mengkustomisasi apa yang akan dilakukan ketika data kita di-drop, bukan kita harus mengimplementasikan trait `Drop` terlebih dahulu baru data kita dapat di-drop. Rust akan secara otomatis men-drop nilai yang sudah mencapai akhir scope.

## Early Drop

Ada kalanya kita ingin men-drop nilai kita lebih dahulu sebelum mencapai akhir scope, seperti saat kita menggunakan lock atau `Mutex`. Rust melarang kita untuk memanggil method `drop` secara langsung dari sebuah instance yang mengimplementasikannya. Kita harus menggunakan fungsi `drop` dari `std::mem::drop`. Kita tidak perlu menggunakan use untuk memanggil fungsi tersebut. Seperti `Vec`, fungsi itu sudah tersedia di dalam prelude.

```rust
fn main() {
	let x = Kotak::new(20);
	drop(x);
	{
		let y = Kotak::new("Halo");
	}	
}
```

Dan output yang akan dikeluarkan adalah:

```
Dropping Kotak yang memiliki data 20!
Dropping Kotak yang memiliki data "Halo"!
```

Variabel `x` akan di-drop terlebih dahulu.