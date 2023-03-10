""""""""""""""""""""""""""""""
Enumerasi dan Pattern Matching
""""""""""""""""""""""""""""""

.. contents:: Overview
   :depth: 3

============
Enumeration
============

Enumeration atau dikenal juga dengan ``enum`` merupakan sebuah ``type`` dalam Rust. Berbeda dengan bahasa lain, enumerasi dalam Rust dapat menyimpan nilai. Fitur ini membuat enumerasi dalam Rust sangat powerful dan dapat digunakan untuk banyak hal, salah satunya adalah untuk ``Result`` type dan ``Option`` type pada Rust.

-----------
Deklarasi
-----------

Pendeklarasian ``enum`` dalam Rust menggunakan keyword ``enum``.

.. code::

            enum Payment {
                Cash,
                Credit,
                Debit,
            }
            

Tiap enumerasi didalam sebuah ``enum`` disebut juga dengan varian. Kita dapat menggunakan varian dalam ``enum`` dibawah satu tipe `enum` yang sama sehingga kita dapat menggunakan salah satu dari beberapa tipe dibawah satu tipe utama. 

Untuk kondisional tentang varian apa yang kita gunakan, kita dapat menggunakan pattern matching.

================
Pattern Matching
================

Pattern Matching merupakan salah satu fitur utama dalam Rust. Pattern matching ini mirip dengan `switch` pada bahasa lain, namun pattern matching dapat digunakan untuk lebih banyak hal seperti digunakan pada assignment. Pattern matching harus bersifat *exhaustive* atau mengcover semua kemungkinan yang ada.

Pattern matching menggunakan keyword ``match``.

.. code::

            fn main() {
                let a = 10;
                
                match a {
                    10 => println!("A adalah 10"),
                    20 => println!("A adalah 20"),
                    _ => println!("A adalah lainnya"),
                }
            }
            
Pada kode diatas, tanda underscore ``_`` menandakan value lain bila ``a`` bukanlah merupakan salah satu dari kedua kemungkinan diatas, yaitu ``10`` dan ``20``.

Pattern Matching pada Enum
--------------------------

Sekarang kita akan membahas penggunaan pattern matching pada ``enum`` yang telah kita buat diatas. Pertama-tama lihatlah kode ini.

.. code::

            enum Payment {
                Cash,
                Credit,
                Debit,
            }

            fn main() {
                let payment = Payment::Cash;

                match payment {
                    Payment::Cash => println!("Pembayaran dengan Cash"),
                    Payment::Debit => println!("Pembayaran dengan Debit"),
                    Payment::Credit => println!("Pembayaran dengan Credit"),
                }
            }
            

Kita menggunakan varian ``Cash`` pada variabel ``payment``, kemudian mencocokkannya dengan ``match``. Hasil yang akan keluar tentulah ``Pembayaran dengan Cash``. Sekarang, agar lebih jelas dan reusable, kita akan menggunakan prosedur.

.. code::

            enum Payment {
                Cash,
                Credit,
                Debit,
            }

            fn pay(method: &Payment) {
                match method {
                    Payment::Cash => println!("Membayar dengan tunai"),
                    Payment::Credit => println!("Membayar dengan credit"),
                    Payment::Debit => println!("Membayar dengan debit")
                }
            }

            fn main() {
                let payment = Payment::Cash;
                let payment2 = Payment::Credit;
                let payment3 = Payment::Debit;

                pay(&payment);
                pay(&payment2);
                pay(&payment3);
            }
            

Dan sekarang, kita dapat menggunakan satu fungsi untuk semua varian.

Sekarang, kita akan menambahkan nilai kedalam varian. Untuk melakukannya, kita harus menaruh tipe data di dalam varian.

.. code::

            enum Payment {
                Cash(f64),
                Credit(String, f64),
                Debit(String, f64),
            }


Dengan melakukan hal ini, kita dapat memasukkan nilai sesuai dengan tipe data yang ada kedalam varian. Sekarang, kita akan merombak ulang yang telah kita tulis.

.. code::

            enum Payment {
                Cash(f64),
                Credit(String, f64),
                Debit(String, f64),
            }

            fn pay(method: &Payment) {
                match method {
                    Payment::Cash(amount) => println!("Membayar dengan tunai sebesar {}", amount),
                    Payment::Credit(num, amount) => println!("Membayar dengan credit dengan nomor {} sebesar {}", num, amount),
                    Payment::Debit(num, amount) => println!("Membayar dengan debit dengan nomor {} sebesar {}", num, amount),
                }
            }

            fn main() {
                let cash = Payment::Cash(100000.0);
                let credit = Payment::Credit(String::from("123"), 50000.0);
                let debit = Payment::Debit(String::from("234"), 75000.0);

                pay(&cash);
                pay(&credit);
                pay(&debit)
            }
            

Lihatlah, kita dapat memasukkan nilai kedalam varian diatas. Kemudian, kita dapat mengecek nilai tersebut dengan menggunakan variabel didalam pattern matching. Variabel-variabel tersebut pada kode diatas adalah ``num`` dan ``amount``. Mereka mewakili nilai yang kalian berikan. Sekarang, output yang keluar adalah sebagai berikut:

.. code::

            Membayar dengan tunai sebesar 100000
            Membayar dengan credit dengan nomor 123 sebesar 50000
            Membayar dengan debit dengan nomor 234 sebesar 75000


Bagaimana? Sangat berguna bukan? Kita dapat menggunakan salah satu dari beberapa varian dibawah satu tipe utama, yang dalam kasus diatas adalah ``Payment``.


--------------------------------------------
Penggunaan Pattern Matching dalam assignment
--------------------------------------------


Kita dapat assign variabel secara kondisional dengan pattern matching seperti ini contohnya:

.. code::

            fn main() {
                let pay = Payment::Credit(String::from("555"), 100000.0);

                // Diskon bila memakai credit dan debit
                let calculate = match pay {
                    Payment::Cash(amount) => amount,
                    Payment::Credit(num, amount) => amount - ((amount * 50.0) / 100.0),
                    Payment::Debit(num, amount) => amount - ((amount * 40.0) / 100.0),
                };

                println!("{}", calculate);

            }
            

Nilai ``calculate`` akan tergantung pada varian yang kita gunakan.

Latihan
----------

TODO