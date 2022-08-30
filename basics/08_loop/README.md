# Loop

Loop adalah urutan instruksi yang akan terus mengulang hingga suatu kondisi terpenuhi, atau tanpa batas. Loop pada bahasa Rust tidak jauh berbeda dengan bahasa lainnya. Rust juga memiliki `for loop` dan `while loop` seperti bahasa lainnya.

## For Loop

Pada Rust, `for loop` terlihat seperti berikut:

```rust
for i in iterable {
    println!("{}", &i);
}
```

Dimana `iterable` merupakan tipe yang dapat diiterate. Contohnya, `Vec` dan `slice`. Kita juga dapat membuat `Range` dalam Rust, seperti pada python yaitu dengan cara berikut:

```rust
for i in 0..100 {
    println!("{}", i);
}
```

Kode diatas akan mencetak angka 0 sampai 99.

### Contoh pada Vector

```rust
let vec = vec!["Satu", "Dua", "Tiga"];

for i in vec {
    println!("{}", i);
}
```

## While 

`while` loop pada Rust tidaklah berbeda dengan bahasa lainnya. Ia akan terus melakukan perulangan hingga kondisi yang tertulis tercapai.

```rust
let mut i = 0;

while i < 10 {
    i += 1;
}
```

Pada kode diatas, while akan terus mengulang hingga `i` berjumlah 9.

## `Loop`

`loop` disini digunakan sebagai perulangan yang akan terus mengulang, seperti `while true`. Dapat digunakan sebagai mainloop sebuah aplikasi. Penulisannya adalah sebagai berikut:

```rust
use std::io;
use std::io::Write;

fn main() {
   let mut input = String::new();
   loop {
      io::stdin().read_line(&mut input).unwrap();
      println!("Yang anda masukkan adalah: {}", input);
      match io::stdout().flush() {
         Ok(_) => (),
         Err(e) => println!("{}", e),
      };
   }
}
```

`loop` akan terus mengulang kode diatas, dan akan terus meminta user input.

# Latihan

// TODO



