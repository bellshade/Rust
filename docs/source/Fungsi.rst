""""""""""
Fungsi
""""""""""


.. contents:: Overview
   :depth: 3
   
===========
Fungsi
===========
   
Fungsi merupakan sebuah abstraksi dari proses khusus yang kemudian mengembalikan nilai. Sebuah prosedur merupakan sebuah abstraksi rangkaian dari proses khusus yang serupa dengan fungsi, namun prosedur tidak mengembalikan apa-apa.

==================
Fungsi dalam Rust
==================

Dalam bahasa Rust, fungsi dideklarasikan dengan keyword ``fn``.

.. code::

            // Sebuah fungsi
            fn a_function() -> i32 {
    
            }

            // Sebuah fungsi dengan parameter
            fn a_function_with_param(a: i32) -> i32 {
               a + 1
            }


Saat sebuah fungsi mengembalikan nilai, kita harus menuliskan secara eksplisit tipe apa yang ia kembalikan dengan tanda ``->``. Lalu karena Rust merupakan expression-oriented language, kita tidak perlu menuliskan keyword ``return`` pada saat mengembalikan nilai. Cukup menuliskan nilai yang dikembalikan tanpa titik koma seperti yang terlihat diatas.

------------
Early Return
------------

Ada kalanya kita menginginkan sebuah nilai untuk dikembalikan duluan sebelum proses sebuah fungsi berakhir. Disinilah kita menggunakan keyword `return` dengan benar pada Rust.

.. code::

            fn early_return(a: i32) -> i32 {
                if a > 10 {
                    return 20;
                }
    
                a + 3
            }


Fungsi diatas akan mengembalikan ``20`` bila parameter ``a`` lebih besar dibanding ``10``, dan mengabaikan ``a + 3`` dibawahnya.

====================
Prosedur dalam Rust
====================

Disini kita akan membahas lebih detail tentang prosedur. Sebuah prosedur biasanya adalah sebuah rangkaian dari proses khusus yang dijalankan untuk mengubah sesuatu, dan tidak mengembalikan apa-apa. Contoh dari sebuah prosedur untuk mengubah nilai sebuah variabel adalah berikut:

.. code::

            fn change_value(a: &mut i32) {
                *a += 3;
            }

            fn main() {
                let mut a = 5;
            // 5
            println!("Sebelum prosedur: {}", a);
            change_value(&mut a);
            // 8
            println!("Setelah prosedur: {}", a);
            }


Prosedur tersebut menerima sebuah **mutable reference** atau reference yang dapat diubah kepada sebuah nilai, dan kemudian mengubah nilai tersebut. Tanda asterisk ``*`` diatas berfungsi untuk men-dereference nilai dari reference diatas, sehingga ia mengubah nilai yang direferensikan reference tersebut.



Latihan
---------


Seperti pada materi sebelumnya, kerjakan latihan yang berada pada file ``lib.rs`` dalam folder ``src``. Ganti ``todo!()`` dengan jawabanmu, kemudian jalankan `cargo test`!
