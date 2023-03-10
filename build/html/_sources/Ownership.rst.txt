"""""""""
Ownership
"""""""""

.. contents:: overview
    :depth: 3

=========
Ownership
=========

Ownership merupakan sebuah sistem yang membuat Rust itu unik. Disinilah nilai jual Rust, yang membedakan Rust dan bahasa lainnya. Dengan sistem ownership pada Rust, Rust dapat mencapai keamanan memory tanpa penggunaan garbage collector.

-------------------
Peraturan Ownership
-------------------

Ownershipnya Rust memiliki beberapa peraturan:

- Setiap nilai dalam Rust memiliki pemilik atau owner
- Hanya boleh ada satu pemilik dalam satu waktu
- Ketika si pemilik keluar dari scope, maka nilai akan di-drop atau dihapus

----------
Masalah
----------


Bagi pemula, saat belajar Rust pasti pernah melakukan hal semacam ini:

.. code::

            fn main() {
                let a = String::from("Halo");
                let b = a;

                println!("{}", a);
            }


Dan kemudian, saat dicompile, ternyata error.

.. code::

            error[E0382]: borrow of moved value: `a`
             --> test.rs:5:20
              |
            2 |     let a = String::from("Halo");
              |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
            3 |     let b = a;
              |             - value moved here
            4 | 
            5 |     println!("{}", a);
              |                    ^ value borrowed here after move


Hal ini tentu akan memusingkan bagi pemula yang baru belajar bahasa Rust. Apa sih yang dimaksud dengan `moved value`? Apa sih ``borrow``? Apa maksud dari `does not implement Copy trait`? Untuk mengetahuinya, mari kita kupas lebih lanjut.


Primitive Type vs Non-Primitive Type
------------------------------------

Tipe data primitif merupakan tipe data klasik seperti ``i32``, ``u32``, ``bool``, ``f32``, dan semacamnya sedangkan tipe data non-primitif merupakan tipe data seperti `String` atau `Vec`. Kunci utama dari ownership pada kedua jenis tipe data ini adalah, tipe data primitif mengimplementasikan trait ``Copy`` sedangkan tipe data non-primitif tidak.

**Copy Trait**
--------------

Copy trait merupakan sebuah ``trait`` (Akan dibahas lebih lanjut nanti) yang berfungsi untuk memberitahu compiler bahwa suatu tipe tidak akan me-move nilainya, melainkan mengkopinya.

.. code::

            fn main() {
                let a = 10;
                let b = a;

            println!("{}", a);
            println!("{}", b);
            }


Kode diatas akan berjalan dengan sempurna, tidak seperti kode dengan tipe data ``String`` diatasnya lagi. Mengapa? Karena ``a`` dan ``b`` memiliki tipe data ``i32``, maka ``a`` dan ``b`` mengimplementasikan trait ``Copy`` sehingga variabel ``a`` yang di-assign ke ``b`` akan menunjuk kepada nilai yang berbeda di memory. ``a`` yang di-assign pada `b` bukan merupakan variabel ``a`` yang sama lagi, melainkan nilai ``10`` baru di memory karena ``a`` merupakan sebuah kopi baru dari nilai ``10`` tersebut. Hal ini dilakukan secara otomatis untuk semua tipe yang mengimplementasikan trait ``Copy``. Kesimpulannya adalah, tipe yang mengimplementasikan trait ``Copy`` akan secara otomatis membuat nilai baru di memory setiap kali ia dipakai.

**Move**
----------

Kita telah mengetahui bahwa tipe yang mengimplementasikan ``Copy`` akan selalu membuat kopi dari dirinya sendiri setiap kali ia dipakai. Lalu bagaimana dengan move? Tipe yang tidak mengimplementasikan ``Copy`` akan di-move, bukan di-copy setiap kali ia dipakai. Untuk lebih jelasnya, mari kita lihat kembali kode yang error diatas.

.. code::

            fn main() {
                let a = String::from("Halo");
                let b = a;

                println!("{}", a);
            }


Disini terlihat bahwa variabel ``a`` di-assign pada variabel ``b``, kemudian kita memakai variabel ``a`` pada macro `println!` dibawahnya. Dan yang terjadi adalah error. Mengapa demikian? Karena disaat kita assign variabel ``a`` pada ``b``, kepemilikan akan nilai dari variabel ``a``, yaitu ``String "Halo"`` akan di-move ke variabel ``b`` sehingga ``a`` tidak memiliki kepemilikan, atau ownership terhadap `String "Halo"` lagi yang membuatnya tidak lagi valid. Hal ini merupakan salah satu dari peraturan ownership dimana hanya dapat ada satu pemilik atau owner dalam satu waktu. ``a`` yang di-assign ke ``b`` bukan merupakan kopi, namun merupakan nilai yang sama di memory.

------------------
Mengatasi Masalah
------------------

Lalu bagaimana cara kita untuk mengatasi masalah diatas? Bagaimana cara kita memakai variabel ``a`` berulang kali tanpa membuat error? Ada dua cara.

**Borrow**
----------

Dalam Rust, borrow atau meminjam merupakan sebuah cara yaitu menaruh **(&)** atau tanda reference di depan sebuah variabel. Dengan borrow, kepemilikan sebuah nilai hanya akan dipinjam, bukan di-move dan akan dikembalikan kepada pemilik asalnya setelah keluar dari scope.

.. code::

            fn main() {
                let a = String::from("Halo");
                let b = &a;

                println!("{}", a);
                println!("{}", b);
            }


Kode diatas akan berjalan dengan baik. Kita juga dapat memberikan reference pada fungsi lewat parameter.

.. code::

            fn greet(name: &String) {
                println!("Halo {}", name);
            }

            fn main() {
                let name = String::from("Rahman");
                greet(&name);
            }


Dan variabel ``name`` dapat tetap dipakai walau fungsi dipanggil berulang kali.

**Clone**
---------

Cara kedua adalah cloning. Dengan menggunakan trait ``Clone``, kita dapat melakukan hal yang mirip ``Copy``, namun secara eksplisit kepada suatu tipe yang mengimplementasikan ``Clone``.

.. code::

            fn main() {
                let a = String::from("Halo");
                let b = a.clone();

                println!("{}", a);
                println!("{}", b);
            }


Dan kode diatas akan berjalan dengan baik. Namun, perlu diketahui bahwa ``Clone`` itu "mahal", karena ``Clone`` akan membuat nilai yang baru di memory, dan memory yang dialokasikan oleh tipe non-primitif tidaklah kecil. Jadi ``a`` dan ``b`` diatas tidaklah menunjuk kepada nilai yang sama di memory.

# Latihan

Pada latihan kali ini, kita akan bertemu dengan dua fungsi. Namun, kedua-duanya error. Apa yang menyebabkannya? Tugas kalian adalah memperbaiki kode tersebut. Kerjakanlah soal pada file ``lib.rs`` dalam folder ``src`` kemudian run ``cargo test`` untuk mengecek jawaban kalian!