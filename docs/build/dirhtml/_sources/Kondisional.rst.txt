"""""""""""
Kondisional
"""""""""""

..  contents:: Overview
   :depth: 3

=============
Kondisional
=============


Di chapter sebelumnya, kita telah mengetahui tentang ``match``. Sekarang, kita akan masuk ke dalam ``if`` dan ``else`` statement. If-else statement pada Rust dapat digunakan dengan cara yang sama, dan juga berbeda dari bahasa lain. If-else dalam Rust dapat digunakan pada saat assignment variabel, seperti ``match``. Penggunaan secara normal tidak berbeda jauh dengan bahasa lainnya.


.. code::

            fn main() {

                let a = 10;

                if a > 10 {
                    println!("A lebih besar dari 10");
                } 
                else if a == 10 {
                    println!("A sama dengan 10");
                }
                else {
                    println!("A kurang dari 10");
                }

            }



Dan penggunaannya dalam assignment,

            
.. code::

            fn main() {
                
                let a = 10;

                let b = if a == 10 {
                    7
                } else {
                    2
                };

                // OUTPUT: 7
                println!("{}", b);

            }
            
Namun, kita dapat menggunakan ``if`` dengan agak spesial, untuk pengganti pattern matching.


----------
If Let
----------

``if let`` merupakan suatu fitur spesial yang dapat digunakan untuk ``enum``. Statement ini digunakan untuk mengecek apakah sebuah ``varian`` benar-benar merupakan varian tersebut. ``if let`` dapat digunakan untuk hal yang straightforward dan tidak memerlukan pattern matching.

Sekarang perhatikan kode dibawah ini.

.. code::

            let optional = Some(7);

            match optional {
                Some(i) => {
                    println!("{}", i);

                },
                _ => {},
            };
            

Kode diatas sangatlah ``awkward`` dan terlalu panjang untuk operasi seperti itu. Kita bahkan tidak memakai ``None`` dan semua operasi adalah ``void`` atau tidak mengembalikan nilai apapun. Kita hanya ingin mencetak nilai yang di-wrap didalam varian ``Some`` pada variabel ``optional``. disini kita hanya membutuhkan satu varian saja, yaitu ``Some(i)`` dimana ia sudah pasti bukan ``None``. Nah, Rust memiliki cara yang lebih baik untuk hal seperti ini dengan menggunakan ``if let``.

.. code::

            let optional = Some(7);

            if let Some(i) = optional {
                println!("{}", i);
            }


Nah, dengan begini kode akan lebih pendek dari sebelumnya, dan juga tidak terlihat canggung lagi. Penggunaan ``if let`` adalah dengan menggunakan varian yang diinginkan, kemudian mengisinya dengan nilai bila varian tersebut memuat nilai, lalu menggunakan operator assignment, kita assign variabel yang ingin kita gunakan seperti ``optional`` pada diatas, kemudian dalam scope ``if let`` tersebut, kita dapat menggunakan nilai yang di-wrap didalam varian tersebut yaitu pada contoh diatas adalah ``i``, sesuka hati kita.

Kita juga dapat menggunakannya pada ``enum`` apapun termasuk yang kita definisikan sendiri.

.. code::

            enum Test {
                A,
                B(i32),
            }

            fn main() {
                let a = Test::A;
                let b = Test::B(10);

                if let Test::A = a {
                    println!("Varian A tanpa nilai")
                }

                if let Test::B(v) = b {
                    println!("Varian B memiliki nilai: {}", v)
                }
            }
            

# LATIHAN

TODO