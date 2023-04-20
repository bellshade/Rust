# Lifetime

Dalam bahasa Rust, setiap variabel atau objek memiliki lifetime yang terkait dengannya, yaitu berapa lama variabel atau objek tersebut diperlukan dan digunakan dalam program. Rust memastikan bahwa memori yang digunakan oleh variabel atau objek hanya disediakan selama periode waktu yang diperlukan, dan tidak lebih lama dari itu.

Lifetime biasanya ditentukan oleh tempat variabel atau objek dideklarasikan, serta hubungannya dengan variabel atau objek lain dalam program. Rust menggunakan aturan-aturan tertentu untuk menentukan lifetime secara otomatis, sehingga programmer tidak perlu secara manual menentukan lifetime setiap variabel atau objek.

Sebagai contoh, Rust akan menentukan lifetime dari variabel `x` sebagai berikut:

```rust
fn main() {
    let x = 5;
}
```

Karena `x` dideklarasikan di dalam fungsi `main`, maka `x` akan memiliki lifetime yang sama dengan fungsi `main`. Ketika fungsi `main` selesai dieksekusi, maka `x` akan dihapus dari memori.

Sekarang, lihat kode dibawah ini.

```rust
fn main() {
    let x = 5;

    {
        let y = &x;
        println!("{}", y);
    }
}
```

Karena `y` dideklarasikan di dalam sebuah *scope* baru, maka `y` akan memiliki lifetime yang sama dengan *scope* tersebut dan lifetimenya akan berakhir ketika ia mencapai akhir scope. Di sini, `y` adalah *reference* yang menunjuk pada variabel `x`. Karena `y` hanya memiliki lifetime yang sama dengan *scope*-nya, maka Rust memastikan bahwa referensi ini tidak akan mencoba untuk mengakses memori yang tidak valid. Lifetime disini menunjukkan bahwa `y` akan tetap valid selama `x` masih ada di memori, dan ia belum keluar dari *scope* miliknya.

## Explicit Annotation

Ada kalanya Rust tidak dapat menentukan lifetime secara implisit, seperti saat kita menggunakan *reference* pada tipe data kompleks seperti `struct` atau `enum`, Rust tidak dapat menentukan lifetime secara otomatis. Dalam kasus ini, programmer harus secara manual menentukan lifetime dari variabel atau objek tersebut. Hal ini disebut dengan *explicit annotation*.

Lihat contoh dibawah ini.

```rust
fn main() {
    let x = 5;

    let result = get_value(&x);

    println!("{}", result);
}

fn get_value(&x: i32) -> &i32 {
    x
}

```

Kode diatas akan berjalan dengan baik karena Rust dapat menentukan lifetime secara otomatis. Hal ini disebut dengan *lifetime elision*. 

Namun, jika kita mengubah kode tersebut menjadi seperti dibawah ini, maka Rust akan mengeluarkan error.

```rust
fn main() {
    let x = 5;
    let y = 10;

    let result = get_bigger(&x, &y);

    println!("{}", result);
}

fn get_bigger(a: &i32, b:  &i32) -> &i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

```
   |
10 | fn get_bigger(a: &i32, b:  &i32) -> &i32 {
   |                  ----      ----     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
   |
10 | fn get_bigger<'a>(a: &'a i32, b:  &'a i32) -> &'a i32 {
   |     
```

Hal ini disebabkan oleh Rust yang tidak dapat menentukan *reference* mana yang akan dikembalikan oleh fungsi `get_bigger`. Bisa jadi `a` dan `b` memiliki lifetime yang berbeda dan Rust tidak mengetahui harus mengembalikan lifetime yang mana untuk *reference* yang dikembalikan. Karena itu, kita harus secara manual menentukan lifetime parameter dari variabel `a` dan `b` dengan menambahkan lifetime specifier di antara reference dan tipe data disana.

Biasanya, nama lifetime parameter dimulai dari `'a`, `'b`, dan seterusnya. Namun, kita juga dapat memberikan nama lain untuk lifetime parameter, seperti `'x`, `'y`, dan seterusnya.

```rust
fn main() {
    let x = 5;
    let y = 10;

    let result = get_bigger(&x, &y);

    println!("{}", result);
}

fn get_bigger<'a>(a: &'a i32, b:  &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

Dengan begini, Rust mengetahui bahwa fungsi `get_bigger` akan mengembalikan *reference* yang memiliki lifetime yang sama dengan variabel `a` dan `b`.

## Lifetime pada Struct

Bila sebuah `struct` memiliki field yang merupakan *reference*, Rust tidak dapat menentukan lifetime secara otomatis. Kita harus secara manual menentukan lifetime parameter dari `struct` tersebut.

```rust
struct Magician<'a> {
    name: &'a str,
    power: &'a str,
}

fn main() {
    let name = "Marisa Kirisame";
    let power = "Heat Magic";

    let magician = Magician {
        name,
        power,
    };

    println!("{} has {} power", magician.name, magician.power);
}
```

Dan kemudian, pada *implementation block* kita dapat menuliskannya seperti ini.

```rust
impl<'a> Magician<'a> {
    fn new(name: &'a str, power: &'a str) -> Self {
        Self {
            name,
            power,
        }
    }

    fn introduce(&self) {
        println!("{} has {} power", self.name, self.power);
    }
}
```

## Static Lifetime

Lifetime `static` adalah lifetime yang paling panjang, yaitu selama program berjalan. Lifetime `static` biasanya digunakan untuk variabel atau objek yang memiliki nilai yang tetap selama program berjalan, seperti konstanta.

```rust
static PI: f64 = 3.14159265359;

fn main() {
    println!("{}", PI);
}
```

`PI` yang dideklarasikan sebagai `static` akan memiliki lifetime `static` yang sama dengan program. Ketika program selesai dieksekusi, maka `PI` akan dihapus dari memori.

Penggunaan lifetime annotation `'static` juga dapat digunakan. Biasanya dalam kasus pengembalian *value* yang bertipe `&str` dari fungsi.

```rust
fn main() {
    let result = get_name();

    println!("{}", result);
}

fn get_name() -> &'static str {
    "Marisa Kirisame"
}
```

Di Rust, `str` selalu menjadi sebuah *reference* karena ia merepresentasikan string slice yang menunjuk ke sebuah urutan byte-byte yang UTF-8 di dalam memori. String slice merupakan tampilan atau representasi dari sebuah string, sehingga ia meminjam memori yang mendasari yang memuat byte-byte dari string tersebut. Oleh karena itu, str selalu menjadi sebuah reference, yaitu &str.

Karena lifetime sebuah *reference* hanya berlaku pada *scope* tempat ia dibuat, `&str` yang merupakan sebuah *reference* akan memaksa kita untuk menggunakan lifetime `static` dimana ia akan memiliki lifetime yang sama dengan program dalam kasus di atas.