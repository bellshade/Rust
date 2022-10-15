# `Deref` Trait

Sebuah smart pointer adalah sebuah tipe yang mengimplementasikan trait Deref dan trait Drop. Di artikel kali ini, kita akan membahas tentang trait Deref yang membuat kita dapat memperlakukan sebuah pointer seperti sebuah reference biasa. Lalu apa maksud dari memperlakukan sebuah pointer seperti sebuah reference biasa?

Sebelum itu, mari kita membahas lebih lanjut tentang Dereferencing.

## Dereferencing

Dereferencing adalah sebuah cara untuk mengakses nilai dari sebuah lokasi memori yang ditunjuk oleh sebuah pointer. Pada Rust, seperti dalam bahasa seperti C++, kita menggunakan operator * untuk dereferencing.

Sekarang, mari kita lihat kode berikut

```rust
fn main() {
	let a = 10;
	let b = &a;
	
	assert_eq!(10, a);
	assert_eq!(10, b);
}
```

Kita menggunakan macro assert_eq! pada kode diatas untuk mengecek apakah sebuah nilai setara (equal) dengan nilai yang lainnya. Namun, hal yang akan terjadi saat kita compile kode diatas adalah compile error yakni sebagai berikut

```
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> deref.rs:7:2
  |
7 |     assert_eq!(10, b);
  |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
```

Error diatas menyatakan kalau kita tidak bisa membandingkan sebuah integer dengan sebuah reference kepada sebuah integer. Mereka adalah tipe yang berbeda sehingga kita harus menggunakan dereference operator.

Dereference operator menggunakan tanda asterisk (*)

```rust
fn main() {
	let a = 10;
	let b = &a;
	
	assert_eq!(10, a);
    assert_eq!(10, *b);
}
```

Pada Rust, sebuah reference (&) sebenarnya juga merupakan pointer. Jadi, variabel b diatas adalah sebuah pointer yang menyimpan alamat memory a dan menunjuk kepada dimana valuenya, 10 disimpan.

Pada assertion pertama, kita membandingkan 10 dan a, yang hasilnya adalah benar. Pada assertion kedua, kita membandingkan 10 dan b yang sudah kita dereference sehingga b disana merupakan value yang ia tunjuk, yaitu 10. Kode akan berjalan dengan baik.

## Apa itu Trait `Deref`?

Kita dapat memperlakukan sebuah smart pointer seperti sebuah reference biasa.

Untuk contoh yang lebih lanjut, kita akan mengganti kode diatas dan menggunakan sebuah smart pointer daripada sebuah reference. Kita akan menggunakan `Box<T>`.

```rust
fn main() {
	let a = 10;
	let b = Box::new(a);
	
	assert_eq!(10, a);
	assert_eq!(10, *b);
}
```

Seperti reference, `Box` juga menunjuk kepada nilai yang disimpan di suatu tempat di memori, yang dalam hal ini adalah `10`. Perbedaannya disini adalah `b` menunjuk pada sebuah copy dari `10` karena value tipe primitif akan di-copy, bukan di-move ownershipnya.

`Box<T>` merupakan sebuah smart pointer yang mengimplementasikan trait `Deref`. Inilah yang dimaksud dengan memperlakukan sebuah pointer seperti sebuah reference biasa. Trait `Deref` memperbolehkan dereference operator bekerja pada `Box<T>` sama seperti ia bekerja pada reference biasa.

Untuk mengerti bagaimana itu bekerja, kita akan mendefinisikan sebuah smart pointer kita sendiri yang akan mengimplementasikan `Deref`.

### Mendefinisikan smart pointer kita sendiri

Kita akan mendefiniskan sebuah smart pointer yang serupa dengan `Box<T>`. Hanya saja, disini kita tidak akan menyimpan nilai pada heap. Disini kita akan berfokus pada dereference operator, bukan dimana lokasi data disimpan.

```rust
struct Kotak<T>(T);

impl<T> Kotak<T> {
	fn new(x: T) -> Self {
		Self(x)
	}
}
```

Sekarang, kita ganti `Box<T>` di fungsi main dengan `Kotak<T>` kita.

```rust
fn main() {
	let a = 10;
	let b = Kotak::new(a);
	
	assert_eq!(10, a);
	assert_eq!(10, *b);
}
```

Namun pada kode diatas, kita akan mendapatkan error dimana kita tidak bisa melakukan dereference pada tipe `Kotak` kita. Sekarang, mari kita implementasikan trait `Deref`.

#### Mengimplementasikan `Deref` pada smart pointer kita

Pertama-tama. panggil trait `Deref` di baris paling atas kode.

```rust
use std::ops::Deref;
```

Lalu kita implementasikan pada `Kotak<T>`.

```rust
impl<T> Deref for Kotak<T> {
	type Target = T;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
```

Kalian tidak perlu terlalu mengkhawatirkan `type Target = T` untuk sekarang. Itu adalah sebuah associated type yang akan kita bahas di lain waktu. Kalian juga bisa mengganti return type method `deref` menjadi hanya `&T`.

Trait `Deref` mengharuskan kita untuk mengimplementasikan satu method bernama `deref` yang menerima `&self`, dan mengembalikan sebuah reference kepada inner data atau data didalam `struct` kita. Ingat, kita disini memakai `tuple struct` yang menggunakan index 0, 1, dan seterusnya untuk mengambil inner data.

Sekarang, kode kita akan terlihat seperti ini

```rust
use std::ops::Deref;

struct Kotak<T>(T);

impl<T> Kotak<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for Kotak<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let a = 10;
    let b = Kotak::new(a);

    assert_eq!(10, a);
    assert_eq!(10, *b);
}
```

Dan assertion kedua kita akan berhasil. Kita akan dapat melakukan dereference pada tipe `Kotak` kita. Kode akan dapat kita compile.

Tanpa trait `Deref`, compiler hanya mengetahui cara dereference reference saja. Trait `Deref` membuat compiler Rust untuk memanggil method `deref` untuk semua tipe yang mengimplementasikannya - untuk mendapatkan sebuah reference kepada sebuah nilai (Self::Target atau &T kita diatas), yang si compiler tahu bagaimana cara dereferencenya.

Saat kita menggunakan operator dereference kepada sebuah nilai yang telah mengimplementasikan trait `Deref`, assertion kedua kita di atas contohnya, sebenarnya Rust memanggil kode seperti berikut:

```rust
assert_eq!(10, *(b.deref()));
```

Rust akan memanggil method `deref` terlebih dahulu untuk mendapatkan reference kepada nilai kita, yang pada kasus diatas adalah `10`, lalu melakukan dereferencing dengan operator dereference sehingga kira-kira hal yang terjadi adalah berikut:

`Kotak(10) -> deref() terpanggil -> &10 -> dereference operator digunakan -> 10`

Karena Rust melakukan hal itu secara otomatis, kita tidak perlu memikirkan perlu atau tidaknya memanggil method `deref` secara eksplisit sehingga kita bisa memperlakukan reference biasa, dan sebuah tipe yang mengimplementasikan trait `Deref` dengan sama.

Lalu kenapa method `deref` mengembalikan reference kepada suatu nilai bukan nilainya itu sendiri?

Tentunya itu berhubungan dengan ownership pada Rust. Kalau `deref` mengembalikan nilainya secara langsung, maka ownership dari nilai tersebut akan di-move keluar dari tipe kita, yang dalam kasus ini, smart pointer kita Kotak<T>. Dan di banyak kasus saat kita menggunakan operator dereference, kita tidak mau itu terjadi.

## Deref Coercion

Kita telah melihat bagaimana trait `Deref` bekerja. Sekarang, kita akan melihat bagaimana trait `Deref` bekerja dengan `Deref Coercion`.

Deref Coercion adalah sebuah fitur sangat praktis yang Rust akan secara otomatis gunakan pada dan hanya pada tipe yang mengimplementasikan trait `Deref` ketika tipe tersebut dijadikan argumen untuk fungsi atau method. Deref Coercion akan mengubah sebuah reference dari satu tipe kepada sebuah reference dari tipe yang berbeda.

Untuk lebih jelasnya, mari kita lanjutkan kode diatas dengan menambahkan sebuah prosedur untuk mencetak `&str`.

```rust
fn main() {
    let a = 10;
    let b = Kotak::new(a);

    assert_eq!(10, a);
    assert_eq!(10, *b);
}

fn prosedur(a: &str) {
	println!("A adalah: {}", a);
}
```

Sekarang, kita akan membuat sebuah variabel baru yang menggunakan smart pointer `Kotak<T>` kita dan sebuah `String` untuk nilai didalam `Kotak<T>` kita, lalu kita panggil prosedur kita dengan variabel tersebut sebagai argumen.

```rust
fn main() {
    let a = 10;
    let b = Kotak::new(a);

    assert_eq!(10, a);
    assert_eq!(10, *b);
    
    let c = Kotak::new(String::from("Hai"));
    prosedur(&c);
}

fn prosedur(a: &str) {
	println!("A adalah: {}", a);
}
```

Seperti yang kalian lihat, prosedur `prosedur` menerima `&str` sebagai argumen. Diatas, kita memberikannya sebuah reference kepada `Kotak<T>` yang memiliki `String` didalamnya. Namun, kode diatas tidak akan error! Kode diatas akan berjalan dengan sempurna.

Apa yang terjadi disini?

Inilah yang terjadi:

Saat kita memakai operator reference di argumen prosedur pada variabel `c`, method `deref` akan terpanggil dan kita akan mendapatkan sebuah reference kepada `String`, nilai yang kita wrap dalam `Kotak<T>`.

```rust
&Kotak<String> -> &String
```
Lalu, karena `String` juga mengimplementasikan trait `Deref`, bila kita menggunakan operator dereference, `String` akan mengembalikan sebuah `&str` sehingga hal yang akan terjadi berikutnya adalah:

```rust
&Kotak<String> -> &String -> &str
```

Rust melakukannya dengan otomatis. Tanpa Deref Coercion, bila ingin melakukan hal seperti diatas, kita harus menuliskannya seperti ini:

```rust
prosedur(&(*c)[..]);
```

Disana kita melakukan dereference pada `c` sehingga kita mendapat sebuah `String`, kemudian `&` dan `[..]` (`slice` yang berisi operator `RangeFull`) akan membuat sebuah `&str` dari si `String` yang setara dengan panjang penuh (full range) si `String`. Sangat merepotkan bukan? Kode akan lebih sulit ditulis dan lebih sulit dibaca. Terima kasih Deref Coercion!

Untuk mutable reference, kita harus menggunakan trait `DerefMut`.

Rust melakukan Deref Coercion bila ia bertemu tipe dan implementasi trait dalam tiga kasus:

- Dari `&T` ke `&U` ketika `T: Deref<Target=U>`
- Dari `&mut T` ke `&mut U` ketika `T: DerefMut<Target=U>`
- Dari `&mut T` ke `&U` ketika `T: Deref<Target=U>`
