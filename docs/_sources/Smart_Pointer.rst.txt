"""""""""""""""
Smart Pointer
"""""""""""""""

..  contents:: Overview
    :depth: 3

==================
Box Smart Pointer
==================

Sebuah pointer, sebuah konsep umum untuk sebuah variabel yang menyimpan alamat memory. Alamat memory ini menunjuk - pointing pada sebuah data. Smart Pointer, atau pointer pintar namun, adalah struktur data yang tidak hanya berperilaku seperti sebuah pointer, namun juga memiliki kapabilitas lain. Konsep smart pointer ini berawal dari C++.

Di artikel ini, kita akan membahas penggunaan Box, sebuah smart pointer yang sangat umum digunakan di Rust. Box digunakan untuk menunjuk pada data di heap. Box memiliki kapabilitas untuk mengalokasikan data di heap dan menghapusnya ketika sudah tidak digunakan.

Penggunaan Box adalah sebagai berikut

.. code::

      fn main() {
      	let boxed_value = Box::new(10);
      	println!("{}", boxed_value);
      }
      

Dalam kode diatas, kita mengalokasikan 10 yang merupakan sebuah integer - tipe primitif pada heap, yang seharusnya ada pada stack. Box tidak mengimplementasikan ``Copy`` karena ia bukan tipe primitif sehingga, bila ingin menggunakannya berulang kali, kita harus menggunakan borrow (&), atau clone untuk variabel box.

.. code::

      fn main() {
      	let boxed_value = Box::new(10);
      	let clone_box = boxed_value.clone();
      	println!("{}", clone_box);
      }
      

Penggunaan Box Lebih Mendalam
-----------------------------

Sekarang, lihat ``enum`` berikut

.. code::

      #[derive(Debug)]
      enum List<T> {
          Cons(T, List<T>),
          Nil,
      }
      

``enum`` di atas merupakan struktur data ``Cons List`` yang berasal dari bahasa Lisp. Struktur data diatas akan terus memuat data di dalam varian ``Cons`` secara rekursif hingga ia menemukan ``Nil``. Setelah menemukan ``Nil``, ia akan berhenti disana tanpa data apapun lagi. Bila kalian sudah membaca bab ``enum``, kalian pasti mengerti cara kerjanya.

Namun, perlu ditegaskan bahwa ``enum`` di atas tidak akan bekerja. Mengapa?

Rust harus mengetahui berapa besar ruang yang sebuah tipe ambil pada saat compile time. Sedangkan pada ``enum`` diatas, ia bersifat rekursif yang dalam teori, dia dapat berulang selamanya - tidak terbatas. Ia dapat terus memuat varian `Cons` yang memuat tipe `List` yang berupa varian ``Cons`` juga dan terus begitu. Rust tidak mengetahui berapa besar si ``enum`` ``List`` pada saat compile time.

Mari kita coba mengimplementasikan contoh Cons List diatas lalu kita compile.

.. code::

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
      

Bila kita mengcompile kode diatas, kita akan mendapatkan error berikut:

.. code::

      2 | enum List<T> {
        | ^^^^^^^^^^^^ recursive type has infinite size
      3 |     Cons(T, List<T>),
        |             ------- recursive without indirection
        |
      help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
        |
      3 |     Cons(T, Box<List<T>>),


Error diatas menunjukkan bahwa kita memiliki tipe rekursif dengan ukuran tidak terbatas - rekursif tanpa indirection. Error diatas juga membantu kita dengan memberitahu bahwa kita harus me-wrap ``List`` dalam ``Box``. Indirection disini berarti daripada kita menyimpan nilai dari ``List`` yang rekursif secara langsung, kita harus menyimpan sebuah pointer, yang mengarah kepada nilai dari ``List`` - yaitu ``Box`` tersebut.

Sebelum kita membahas mengapa Box menyelesaikan masalah diatas, mari kita bahas bagaimana compiler Rust menghitung enum yang non-rekursif.

.. code::

      enum Enum {
      	A,
      	B(i32, i32),
      	C(f64, i64, String),
      }
      

Disini kita memiliki sebuah ``enum`` dengan 3 varian dimana dua varian memiliki nilai didalamnya. Cara Rust menghitung besar ``enum`` diatas adalah dengan mengecek setiap varian dan tipe nilai yang dimiliki varian dan mencari varian mana yang membutuhkan ruang paling banyak - atau varian dengan ukuran terbesar. Karena kita hanya bisa menggunakan satu varian dalam satu waktu, maka besar varian yang paling besar akan menjadi besar dari ``Enum`` itu sendiri.

Namun, untuk ``Cons`` ``List`` kita, saat Rust bertemu dengan tipe ``List`` dalam varian ``Cons``, ia akan kembali lagi pada ``List``, dan berulang terus seperti itu sehingga tidak ada cara untuk mengetahui berapa besar si varian ``Cons`` kita dan Rust tidak akan tahu juga berapa besar ``enum`` ``List`` kita.

Sekarang, seperti yang Rust compiler sarankan, kita akan me-wrap ``List`` kita di dalam ``Box``. Mari kita lakukan.

.. code::

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
      

Dan kode kita akan tercompile:
.. code::

      Cons(42, Cons(69, Cons(613, Nil)))

Lalu bagaimanakah ``Box`` menyelesaikan masalah ini? Pertama-tama, ``Box`` adalah sebuah pointer. Ukuran dari sebuah pointer itu tetap. Ukuran pointer tidak berdasarkan besar atau jumlah data yang dia tunjuk. ``Box`` menunjuk pada nilai ``List`` kita selanjutnya yang berada pada memori heap, bukan pada varian ``Cons`` sehingga ini akan seperti menaruh sesuatu bersebelahan dengan sesuatu yang lain, bukan menaruh sesuatu didalam sesuatu yang lain dan ``Box`` menunjuk pada sesuatu yang bersebelahan tersebut yang dalam hal ini adalah nilai dari ``List`` yang di-wrap dalam ``Box`` pada varian ``Cons``.

Kesimpulan: Pada Rust, ``usize`` itu pointer-sized sehingga ukuran dari ``Cons`` adalah ukuran dari tipe yang kita berikan pada genericnya, dan ``usize`` karena kita menyimpan pointer.

=================
`Deref` Trait
=================

Sebuah smart pointer adalah sebuah tipe yang mengimplementasikan trait Deref dan trait Drop. Di artikel kali ini, kita akan membahas tentang trait Deref yang membuat kita dapat memperlakukan sebuah pointer seperti sebuah reference biasa. Lalu apa maksud dari memperlakukan sebuah pointer seperti sebuah reference biasa?

Sebelum itu, mari kita membahas lebih lanjut tentang Dereferencing.


Dereferencing
-------------

Dereferencing adalah sebuah cara untuk mengakses nilai dari sebuah lokasi memori yang ditunjuk oleh sebuah pointer. Pada Rust, seperti dalam bahasa seperti C++, kita menggunakan operator * untuk dereferencing.

Sekarang, mari kita lihat kode berikut

.. code::

      fn main() {
        let a = 10;
        let b = &a;
        
        assert_eq!(10, a);
        assert_eq!(10, b);
      }


Kita menggunakan macro assert_eq! pada kode diatas untuk mengecek apakah sebuah nilai setara (equal) dengan nilai yang lainnya. Namun, hal yang akan terjadi saat kita compile kode diatas adalah compile error yakni sebagai berikut

.. code::

      error[E0277]: can't compare `{integer}` with `&{integer}`
       --> deref.rs:7:2
        |
      7 |     assert_eq!(10, b);
        |     ^^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
        |
        = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`


Error diatas menyatakan kalau kita tidak bisa membandingkan sebuah integer dengan sebuah reference kepada sebuah integer. Mereka adalah tipe yang berbeda sehingga kita harus menggunakan dereference operator.

Dereference operator menggunakan tanda asterisk (*)

.. code::

      fn main() {
        let a = 10;
        let b = &a;
        
        assert_eq!(10, a);
          assert_eq!(10, *b);
      }


Pada Rust, sebuah reference (&) sebenarnya juga merupakan pointer. Jadi, variabel b diatas adalah sebuah pointer yang menyimpan alamat memory a dan menunjuk kepada dimana valuenya, 10 disimpan.

Pada assertion pertama, kita membandingkan 10 dan a, yang hasilnya adalah benar. Pada assertion kedua, kita membandingkan 10 dan b yang sudah kita dereference sehingga b disana merupakan value yang ia tunjuk, yaitu 10. Kode akan berjalan dengan baik.

Apa itu Trait `Deref`?
----------------------
Kita dapat memperlakukan sebuah smart pointer seperti sebuah reference biasa.

Untuk contoh yang lebih lanjut, kita akan mengganti kode diatas dan menggunakan sebuah smart pointer daripada sebuah reference. Kita akan menggunakan ``Box<T>``.

.. code::

      fn main() {
        let a = 10;
        let b = Box::new(a);
        
        assert_eq!(10, a);
        assert_eq!(10, *b);
      }


Seperti reference, ``Box`` juga menunjuk kepada nilai yang disimpan di suatu tempat di memori, yang dalam hal ini adalah ``10``. Perbedaannya disini adalah ``b`` menunjuk pada sebuah copy dari ``10`` karena value tipe primitif akan di-copy, bukan di-move ownershipnya.

``Box<T>`` merupakan sebuah smart pointer yang mengimplementasikan trait ``Deref``. Inilah yang dimaksud dengan memperlakukan sebuah pointer seperti sebuah reference biasa. Trait ``Deref`` memperbolehkan dereference operator bekerja pada ``Box<T>`` sama seperti ia bekerja pada reference biasa.

Untuk mengerti bagaimana itu bekerja, kita akan mendefinisikan sebuah smart pointer kita sendiri yang akan mengimplementasikan ``Deref``.

Mendefinisikan smart pointer kita sendiri
-----------------------------------------

Kita akan mendefiniskan sebuah smart pointer yang serupa dengan ``Box<T>``. Hanya saja, disini kita tidak akan menyimpan nilai pada heap. Disini kita akan berfokus pada dereference operator, bukan dimana lokasi data disimpan.

.. code::

      struct Kotak<T>(T);

      impl<T> Kotak<T> {
        fn new(x: T) -> Self {
          Self(x)
        }
      }


Sekarang, kita ganti ``Box<T>`` di fungsi main dengan ``Kotak<T>`` kita.

.. code::

      fn main() {
        let a = 10;
        let b = Kotak::new(a);
        
        assert_eq!(10, a);
        assert_eq!(10, *b);
      }


Namun pada kode diatas, kita akan mendapatkan error dimana kita tidak bisa melakukan dereference pada tipe ``Kotak`` kita. Sekarang, mari kita implementasikan trait ``Deref``.

Mengimplementasikan `Deref` pada smart pointer kita
---------------------------------------------------

Pertama-tama. panggil trait ``Deref`` di baris paling atas kode.

.. code::

      use std::ops::Deref;


Lalu kita implementasikan pada ``Kotak<T>``.

.. code::

      impl<T> Deref for Kotak<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }


Kalian tidak perlu terlalu mengkhawatirkan ``type Target = T`` untuk sekarang. Itu adalah sebuah associated type yang akan kita bahas di lain waktu. Kalian juga bisa mengganti return type method ``deref`` menjadi hanya ``&T``.

Trait ``Deref`` mengharuskan kita untuk mengimplementasikan satu method bernama ``deref`` yang menerima ``&self``, dan mengembalikan sebuah reference kepada inner data atau data didalam ``struct`` kita. Ingat, kita disini memakai ``tuple struct`` yang menggunakan index 0, 1, dan seterusnya untuk mengambil inner data.

Sekarang, kode kita akan terlihat seperti ini

.. code::

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


Dan assertion kedua kita akan berhasil. Kita akan dapat melakukan dereference pada tipe ``Kotak`` kita. Kode akan dapat kita compile.

Tanpa trait ``Deref``, compiler hanya mengetahui cara dereference reference saja. Trait ``Deref`` membuat compiler Rust untuk memanggil method ``deref`` untuk semua tipe yang mengimplementasikannya - untuk mendapatkan sebuah reference kepada sebuah nilai (Self::Target atau &T kita diatas), yang si compiler tahu bagaimana cara dereferencenya.

Saat kita menggunakan operator dereference kepada sebuah nilai yang telah mengimplementasikan trait `Deref`, assertion kedua kita di atas contohnya, sebenarnya Rust memanggil kode seperti berikut:

.. code::

      assert_eq!(10, *(b.deref()));


Rust akan memanggil method ``deref`` terlebih dahulu untuk mendapatkan reference kepada nilai kita, yang pada kasus diatas adalah ``10``, lalu melakukan dereferencing dengan operator dereference sehingga kira-kira hal yang terjadi adalah berikut:

``Kotak(10) -> deref() terpanggil -> &10 -> dereference operator digunakan -> 10``

Karena Rust melakukan hal itu secara otomatis, kita tidak perlu memikirkan perlu atau tidaknya memanggil method ``deref`` secara eksplisit sehingga kita bisa memperlakukan reference biasa, dan sebuah tipe yang mengimplementasikan trait ``Deref`` dengan sama.

Lalu kenapa method ``deref`` mengembalikan reference kepada suatu nilai bukan nilainya itu sendiri?

Tentunya itu berhubungan dengan ownership pada Rust. Kalau ``deref`` mengembalikan nilainya secara langsung, maka ownership dari nilai tersebut akan di-move keluar dari tipe kita, yang dalam kasus ini, smart pointer kita Kotak<T>. Dan di banyak kasus saat kita menggunakan operator dereference, kita tidak mau itu terjadi.


===============
Deref Coercion
===============

Kita telah melihat bagaimana trait ``Deref`` bekerja. Sekarang, kita akan melihat bagaimana trait ``Deref`` bekerja dengan ``Deref Coercion``.

Deref Coercion adalah sebuah fitur sangat praktis yang Rust akan secara otomatis gunakan pada dan hanya pada tipe yang mengimplementasikan trait ``Deref`` ketika tipe tersebut dijadikan argumen untuk fungsi atau method. Deref Coercion akan mengubah sebuah reference dari satu tipe kepada sebuah reference dari tipe yang berbeda.

Untuk lebih jelasnya, mari kita lanjutkan kode diatas dengan menambahkan sebuah prosedur untuk mencetak `&str`.

.. code::

      fn main() {
          let a = 10;
          let b = Kotak::new(a);

          assert_eq!(10, a);
          assert_eq!(10, *b);
      }

      fn prosedur(a: &str) {
        println!("A adalah: {}", a);
      }


Sekarang, kita akan membuat sebuah variabel baru yang menggunakan smart pointer ``Kotak<T>`` kita dan sebuah `String` untuk nilai didalam ``Kotak<T>`` kita, lalu kita panggil prosedur kita dengan variabel tersebut sebagai argumen.

.. code::

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


Seperti yang kalian lihat, prosedur ``prosedur`` menerima ``&str`` sebagai argumen. Diatas, kita memberikannya sebuah reference kepada ``Kotak<T>`` yang memiliki ``String`` didalamnya. Namun, kode diatas tidak akan error! Kode diatas akan berjalan dengan sempurna.

Apa yang terjadi disini?

Inilah yang terjadi:

Saat kita memakai operator reference di argumen prosedur pada variabel ``c``, method ``deref`` akan terpanggil dan kita akan mendapatkan sebuah reference kepada ``String``, nilai yang kita wrap dalam ``Kotak<T>``.

.. code::

      &Kotak<String> -> &String
 
Lalu, karena ``String`` juga mengimplementasikan trait ``Deref``, bila kita menggunakan operator dereference, ``String`` akan mengembalikan sebuah ``&str`` sehingga hal yang akan terjadi berikutnya adalah:

.. code::

      &Kotak<String> -> &String -> &str

Rust melakukannya dengan otomatis. Tanpa Deref Coercion, bila ingin melakukan hal seperti diatas, kita harus menuliskannya seperti ini:

.. code::

      prosedur(&(*c)[..]);


Disana kita melakukan dereference pada ``c`` sehingga kita mendapat sebuah ``String``, kemudian ``&`` dan ``[..]`` (``slice`` yang berisi operator ``RangeFull``) akan membuat sebuah ``&str`` dari si ``String`` yang setara dengan panjang penuh (full range) si ``String``. Sangat merepotkan bukan? Kode akan lebih sulit ditulis dan lebih sulit dibaca. Terima kasih Deref Coercion!

Untuk mutable reference, kita harus menggunakan trait ``DerefMut``.

Rust melakukan Deref Coercion bila ia bertemu tipe dan implementasi trait dalam tiga kasus:

- Dari ``&T`` ke ``&U`` ketika ``T: Deref<Target=U>``
- Dari ``&mut T`` ke ``&mut U`` ketika ``T: DerefMut<Target=U>``
- Dari ``&mut T`` ke ``&U`` ketika ``T: Deref<Target=U>``
