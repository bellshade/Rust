
"""""""""""""""""""""""""
Struktur dan Implementasi
"""""""""""""""""""""""""

.. contents:: Overview
   :depth: 3


=========================
Struktur dan Implementasi
=========================


Sebuah ``struct`` atau structure digunakan untuk merepresentasikan tipe data kompleks yang kita definisikan sendiri. Pada Rust, kita dapat membuat sebuah tipe baru dengan menggunakan ``struct``. Kita dapat menambahkan method pada `struct` dengan menggunakan implementation block atau ``impl``.

---------
Struct
---------

``struct`` merupakan sebuah cara untuk mendefinisikan sebuah tipe data yang kompleks. Sebuah ``struct`` dapat memiliki beberapa field, yang masing-masing dapat memiliki tipe data yang berbeda. Sebagai contoh, kita akan membuat sebuah ``struct`` yang merepresentasikan seseorang. Kita akan menggunakan ``struct`` untuk menyimpan data-data yang berkaitan dengan orang tersebut.


.. code::

            struct Person {
                name: String,
                age: u8,
            }


Lalu kita dapat membuat sebuah variabel baru bertipe ``Person`` dengan cara berikut:

.. code::

            fn main() {
                let name = String::from("Fulan");
                let age = 27;
                let person = Person { name, age };
            }
            

**Derive Trait**
----------------

Derive Trait adalah sebuah cara untuk mendapatkan implementasi dari beberapa trait secara otomatis. Sebagai contoh, kita dapat menggunakan ``#[derive(Debug)]`` untuk mendapatkan implementasi dari trait ``Debug`` secara otomatis. Contoh penggunaan derive trait adalah sebagai berikut:

.. code::

            #[derive(Debug)]
            struct Person {
                name: String,
                age: u8,
            }


Sekarang, kita akan membahas beberapa derive trait berguna yang paling sering dipakai pada bahasa Rust.

Sebagai catatan, untuk memakai derive trait, seluruh field dalam ``struct`` harus juga telah mengimplementasikan ``trait`` yang kita derive tersebut. Karena itu, kita tidak akan bisa menggunakan ``trait`` ``Copy`` tanpa ``Clone`` bila ada ``String`` didalam field ``struct`` kita.

- **Debug**
-----------


``Debug`` merupakan trait yang sangat berguna untuk melakukan debugging. Ia akan mencetak output sesuai dengan bentuk asli dari tipe kita. Pada macro ``print!`` atau ``println!``, untuk dapat mencetak ``Debug``, kita harus menggunakan formatting berikut: ``:?``.

Contoh penggunaan:

.. code::

            #[derive(Debug)]
            struct Person {
                name: String,
                age: u8,
            }

            fn main() {
                let name = String::from("Fulan");
                let age = 27;
                let person = Person { name, age };
                println!("{:?}", person);
            }

            // OUTPUT: Person { name: "Fulan", age: 27 }
            

- **Clone**
-----------

Masih ingat dengan materi ownership sebelumnya? Dengan mengimplementasikan ``Clone``, kita dapat melakukan cloning pada tipe kita dengan method ``clone()`` seperti yang sudah pernah dijelaskan di materi ownership sebelumnya.

Contoh penggunaan:

.. code::

            #[derive(Clone, Debug)]
            struct Person {
                name: String,
                age: u8,
            }

            fn main() {
                let name = String::from("Fulan");
                let age = 27;
                let person = Person { name, age };
                let person2 = person.clone();
                println!("{:?}", person2);
            }


- **Copy**
-----------

Seperti ``Clone``, ``Copy`` juga merupakan trait yang berguna untuk melakukan cloning. Namun, ``Copy`` memiliki beberapa syarat yang harus dipenuhi agar dapat digunakan. Syarat-syarat tersebut adalah sebagai berikut:

  * Semua field dalam ``struct`` harus mengimplementasikan ``Copy`` (Contohnya, kita tidak akan bisa menggunakan ``Copy`` untuk tipe yang memiliki field bertipe ``String`` didalamnya.).
  * ``struct`` tidak boleh memiliki implementasi dari ``Drop`` trait.

Dengan ``Copy``, tipe kita akan melakukan copy secara otomatis ketika kita melakukan assignment, seperti yang telah dijelaskan tentang ``trait Copy`` pada artikel ownership sebelumnya. Saat men-derive ``Copy``, kita juga harus men-derive ``Clone``

Contoh penggunaan:

.. code::
 
            #[derive(Copy, Clone, Debug)]
            struct Location {
                lat: f64,
                lon: f64,
            }

            fn main() {
                let loc = Location { lat: 10.55555, lon: 20.22222 };
                let loc2 = loc; // Copy
                let loc3 = loc2; // Copy

                println!("{:?}", loc);
                println!("{:?}", loc2);
                println!("{:?}", loc3);
            }
            

Membuat format print untuk tipe kita
-------------------------------------

Kita dapat membuat format tertentu dan bagaimana kita akan mencetak tipe kita sendiri dengan menggunakan trait ``Display``. Untuk ``Display``, kita tidak dapat menggunakan derive trait. ``Display`` akan mengimplementasikan ``to_string()`` secara otomatis juga pada tipe kita.

Pneggunaan ``Display`` adalah sebagai berikut:

.. code::

            use std::fmt;

            #[Clone, Debug]
            struct Person {
                name: String,
                age: u8,
            }

            impl fmt::Display for Person {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "Seseorang bernama {} dan berumur {}", self.name, self.age)
                }
            }

            fn main() {
                let name = String::from("Fulan");
                let age = 27;
                let person = Person { name, age };
                println!("{}", person); // Seseorang bernama Fulan dan berumur 27
            }


``println!`` akan mencetak sesuai dengan yang telah kita definisikan pada implementasi `Display` kita, yang berada didalam macro ``write!``.

Membuat method untuk tipe kita
-------------------------------

Kita dapat membuat method untuk tipe kita sendiri dengan menggunakan ``impl`` block. Contoh penggunaan adalah sebagai berikut:

.. code::

            #[derive(Clone, Debug)]
            struct Person {
                name: String,
                age: u8,
            }

            impl Person {

                fn new(name: String, age: u8) -> Person {
                    Person { name, age }
                }

                fn say_hello(&self) {
                    println!("Halo, nama saya {} dan berumur {}", self.name, self.age);
                }

                fn birthday(&mut self) {
                    self.age += 1;
                }
            }

            fn main() {
                let name = String::from("Fulan");
                let age = 27;
                let person = Person::new(name, age);
                person.say_hello();

                println!("Saya ulang tahun!");
                person.birthday();

                println!("Umur saya sekarang adalah {}", person.age);
            }


Pada kode diatas, kita memiliki dua tipe method, yaitu method yang dapat langsung dipanggil dari tipenya, dan method yang harus dipanggil melalui variabel yang menyimpan tipe tersebut. Method yang dapat langsung dipanggil dari tipenya disebut dengan method ``associated function``. Method yang harus dipanggil melalui variabel yang menyimpan tipe tersebut disebut dengan method ``instance method``. Method `associated function` biasanya digunakan untuk membuat instance dari tipe tersebut, seperti yang terlihat pada method `new()` pada kode diatas. Ia dipanggil dengan menggunakan ``::`` setelah nama tipe, seperti yang terlihat pada ``Person::new(name, age)``. Namun, method `instance method` biasanya digunakan untuk melakukan operasi pada instance dari tipe tersebut, seperti yang terlihat pada method ``say_hello()`` dan ``birthday()`` pada kode diatas. Ia dipanggil dengan menggunakan ``.`` setelah variabel yang menyimpan tipe tersebut, seperti yang terlihat pada ``person.say_hello()`` dan ``person.birthday()``.


// TODO: Tambahkan iterator, operator overloading, etc.

# LATIHAN

// TODO
