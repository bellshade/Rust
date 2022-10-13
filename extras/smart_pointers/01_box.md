# Box Smart Pointer

Sebuah pointer, sebuah konsep umum untuk sebuah variabel yang menyimpan alamat memory. Alamat memory ini menunjuk - pointing pada sebuah data. Smart Pointer, atau pointer pintar namun, adalah struktur data yang tidak hanya berperilaku seperti sebuah pointer, namun juga memiliki kapabilitas lain. Konsep smart pointer ini berawal dari C++.

Di artikel ini, kita akan membahas penggunaan Box, sebuah smart pointer yang sangat umum digunakan di Rust. Box digunakan untuk menunjuk pada data di heap. Box memiliki kapabilitas untuk mengalokasikan data di heap dan menghapusnya ketika sudah tidak digunakan.

Penggunaan Box adalah sebagai berikut

```rust
fn main() {
	let boxed_value = Box::new(10);
	println!("{}", boxed_value);
}
```

Dalam kode diatas, kita mengalokasikan 10 yang merupakan sebuah integer - tipe primitif pada heap, yang seharusnya ada pada stack. Box tidak mengimplementasikan `Copy` karena ia bukan tipe primitif sehingga, bila ingin menggunakannya berulang kali, kita harus menggunakan borrow (&), atau clone untuk variabel box.

```rust
fn main() {
	let boxed_value = Box::new(10);
	let clone_box = boxed_value.clone();
	println!("{}", clone_box);
}
```

### Penggunaan Box Lebih Mendalam

Sekarang, lihat `enum` berikut

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, List<T>),
    Nil,
}
```

`enum` di atas merupakan struktur data `Cons List` yang berasal dari bahasa Lisp. Struktur data diatas akan terus memuat data di dalam varian `Cons` secara rekursif hingga ia menemukan `Nil`. Setelah menemukan `Nil`, ia akan berhenti disana tanpa data apapun lagi. Bila kalian sudah membaca bab `enum`, kalian pasti mengerti cara kerjanya.

Namun, perlu ditegaskan bahwa `enum` di atas tidak akan bekerja. Mengapa?

Rust harus mengetahui berapa besar ruang yang sebuah tipe ambil pada saat compile time. Sedangkan pada `enum` diatas, ia bersifat rekursif yang dalam teori, dia dapat berulang selamanya - tidak terbatas. Ia dapat terus memuat varian `Cons` yang memuat tipe `List` yang berupa varian `Cons` juga dan terus begitu. Rust tidak mengetahui berapa besar si `enum` `List` pada saat compile time.

Mari kita coba mengimplementasikan contoh Cons List diatas lalu kita compile.

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, List<T>),
    Nil,
}

use List::*; 

fn main() {
    let l = Cons(42, Cons(69, Cons(613, Nil)));

    println!("{:?}", l);
}
```

Bila kita mengcompile kode diatas, kita akan mendapatkan error berikut:

```
2 | enum List<T> {
  | ^^^^^^^^^^^^ recursive type has infinite size
3 |     Cons(T, List<T>),
  |             ------- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
3 |     Cons(T, Box<List<T>>),
```

Error diatas menunjukkan bahwa kita memiliki tipe rekursif dengan ukuran tidak terbatas - rekursif tanpa indirection. Error diatas juga membantu kita dengan memberitahu bahwa kita harus me-wrap `List` dalam `Box`. Indirection disini berarti daripada kita menyimpan nilai dari `List` yang rekursif secara langsung, kita harus menyimpan sebuah pointer, yang mengarah kepada nilai dari `List` - yaitu `Box` tersebut.

Sebelum kita membahas mengapa Box menyelesaikan masalah diatas, mari kita bahas bagaimana compiler Rust menghitung enum yang non-rekursif.

```rust
enum Enum {
	A,
	B(i32, i32),
	C(f64, i64, String),
}
```

Disini kita memiliki sebuah `enum` dengan 3 varian dimana dua varian memiliki nilai didalamnya. Cara Rust menghitung besar `enum` diatas adalah dengan mengecek setiap varian dan tipe nilai yang dimiliki varian dan mencari varian mana yang membutuhkan ruang paling banyak - atau varian dengan ukuran terbesar. Karena kita hanya bisa menggunakan satu varian dalam satu waktu, maka besar varian yang paling besar akan menjadi besar dari `Enum` itu sendiri.

Namun, untuk `Cons` `List` kita, saat Rust bertemu dengan tipe `List` dalam varian `Cons`, ia akan kembali lagi pada `List`, dan berulang terus seperti itu sehingga tidak ada cara untuk mengetahui berapa besar si varian `Cons` kita dan Rust tidak akan tahu juga berapa besar `enum` `List` kita.

Sekarang, seperti yang Rust compiler sarankan, kita akan me-wrap `List` kita di dalam `Box`. Mari kita lakukan.

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::*;

fn main() {

	// Untuk me-wrap nilai dalam Box, gunakan Box::new(nilai)
    let l = Cons(42, Box::new(Cons(69, Box::new(Cons(613, Box::new(Nil))))));

    println!("{:?}", l);
}
```

Dan kode kita akan tercompile:

Cons(42, Cons(69, Cons(613, Nil)))

Lalu bagaimanakah `Box` menyelesaikan masalah ini? Pertama-tama, `Box` adalah sebuah pointer. Ukuran dari sebuah pointer itu tetap. Ukuran pointer tidak berdasarkan besar atau jumlah data yang dia tunjuk. `Box` menunjuk pada nilai `List` kita selanjutnya yang berada pada memori heap, bukan pada varian `Cons` sehingga ini akan seperti menaruh sesuatu bersebelahan dengan sesuatu yang lain, bukan menaruh sesuatu didalam sesuatu yang lain dan `Box` menunjuk pada sesuatu yang bersebelahan tersebut yang dalam hal ini adalah nilai dari `List` yang di-wrap dalam `Box` pada varian `Cons`.

Kesimpulan: Pada Rust, `usize` itu pointer-sized sehingga ukuran dari `Cons` adalah ukuran dari tipe yang kita berikan pada genericnya, dan `usize` karena kita menyimpan pointer.