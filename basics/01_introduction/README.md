- [Pengenalan Umum](#pengenalan-umum)
- [Instalasi dan Cargo](#instalasi-dan-cargo)
  - [Instalasi Rust](#instalasi-rust)
  - [Cargo](#cargo)
    - [Pembuatan Package](#pembuatan-package)
    - [Kompilasi Program](#kompilasi-program)
    - [Cargo.toml](#cargotoml)
    - [Instalasi Package](#instalasi-package)



## Pengenalan Umum

Rust merupakan bahasa pemrograman yang dikembangkan oleh Graydon Hoare dan disokong oleh [Mozilla Foundation](https://en.wikipedia.org/wiki/Mozilla_Foundation) pada 2014 lalu. Rust memaksakan keamanan memory yang berarti, semua _references_ pada Rust adalah valid, dan menunjuk pada memory yang benar-benar ada. Rust mencapainya tanpa penggunaan _garbage collector_.

Bagi pemula, bahkan developer berpengalaman sekalipun, Rust akan terasa agak menyulitkan di awal. Berikut adalah struktur kode Rust:

```rust
fn main() {
    // Entry Point
}
```


## Instalasi dan Cargo

### Instalasi Rust

Cara Instalasi Rust yang paling umum adalah dengan menggunakan [rustup](https://rustup.rs/). Pada halaman web tersebut, OS kalian akan otomatis terdeteksi dan kalian dapat langsung mengikuti instruksi yang ada untuk menginstall Rust. Sebuah prompt akan bertanya pada anda tentang apa saja yang ingin anda install.

Bila kalian telah menginstall Rust lewat rustup, jalankanlah command berikut untuk mengecek instalasi dan versi dari Rust dan Cargo:

```
cargo --version
rustc --version
```

### Cargo

Cargo merupakan package manager Rust. Cargo digunakan untuk mulai dari pembuatan project, kompilasi, hingga pemasangan package. 

#### Pembuatan Package

Tanpa membuat pusing, package disini dapat kalian artikan sebagai project. Ini adalah cara untuk membuat sebuah project baru pada Rust. Untuk membuat sebuah package baru pada Rust, kita dapat memilih apakah package kita akan menjadi sebuah library, atau binary. By default, cargo akan membuat sebuah binary package. Karena itu, untuk membuat sebuah library, dibutuhkan flag `--lib`.

```
cargo new --lib library_saya
```

Command diatas akan membuat sebuah package baru Rust dengan `lib.rs` didalamnya. Untuk membuat sebuah package binary, cukup buat sebuah project tanpa flag apapun, atau menggunakan `--bin` sebagai substitusi flag `--lib` diatas.

```
cargo new binary_saya
```

Project anda akan terstruktur seperti ini:

```
├── Cargo.toml
└── src
    └── main.rs
```

#### Kompilasi Program

Untuk mengkompilasi seluruh project Rust, kita juga menggunakan cargo. Kita dapat membangun, mengecek error, ataupun langsung menjalankan sebuah project Rust bagi project binary.

Setelah menjalankan perintah `cargo new binary_saya` diatas, akan ada sebuah program hello world pada `main.rs`. Kalian dapat menggunakannya untuk mengetes kompilasi Rust dengan menjalankan:

```
cargo run
```

Dan program akan langsung berjalan. Kalian juga dapat melakukan `build` tanpa menjalankannya langsung dengan command:

```
cargo build
```

Dan tergantung apakah build kalian merupakan `release` atau `debug`, package kalian akan berada dalam folder `target/`.

Yang terakhir, dikarenakan kompilasi program pada Rust cukup lama, bila ingin mengecek error, kita tidak harus membangun project kita berulang kali. Kita cukup menggunakan

```
cargo check
```

Untuk mengecek apakah ada error berlangsung.

#### Cargo.toml

File `Cargo.toml` merupakan sebuah file yang berisi semua detail tentang project kita mulai dari nama package, versi package, hingga nama author. Pada file `Cargo.toml` jugalah kita menuliskan nama-nama package dan versi package yang kita inginkan.


#### Instalasi Package

Untuk menginstall package yang ingin kita gunakan pada sebuah project Rust, kita menuliskannya di `Cargo.toml`. Penulisannya ada dibawah tag `[dependencies]` seperti berikut:

```toml
[dependencies]
serde = "1.0.140"
```

Dan package akan terinstall ketika kita menjalankan `build` atau `run`. Yang kedua adalah, bila kita ingin menginstall package binary, maka kita dapat menggunakan command `cargo install` seperti berikut:

```
cargo install diesel_cli
```