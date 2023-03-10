""""""""""
Generic
""""""""""


..  contents:: Overview
    :depth: 3

============
Generic
============

Generics atau Generic Type merupakan tipe yang dapat digunakan untuk berbagai tipe data. Dengan menggunakan generics, kita dapat membuat sebuah fungsi yang dapat digunakan untuk berbagai tipe data, tanpa harus membuat fungsi yang sama untuk berbagai tipe data - yang membuat kode kita lebih reusable. Compiler akan melakukan *monomorphization* pada fungsi yang menggunakan generics, sehingga fungsi yang menggunakan generics akan menjadi fungsi yang spesifik untuk tipe data tertentu.

----------------------
Membuat Generic Struct
----------------------


Kita akan membuat sebuah struct ``Point`` yang memiliki dua field, ``x`` dan ``y``. Kita akan membuat ``Point`` menjadi generic, sehingga kita dapat membuat ``Point`` dengan tipe data yang berbeda-beda.

.. code::

            #[derive(Debug)]
            struct Point<T> {
                x: T,
                y: T,
            }
            

``T`` diatas merupakan tipe placeholder atau generic parameter, yang berfungsi seperti parameter sebuah fungsi yaitu mewakili data yang akan dimasukkan. Namun, generic parameter berfungsi untuk mewakili tipe yang akan dimasukkan, bukan sebuah nilai.

Sekarang, kita akan membuat instance dari ``Point`` dengan tipe data yang berbeda-beda.

.. code::

            fn main() {
                let integer = Point { x: 5, y: 10 };
                let float = Point { x: 1.0, y: 4.0 };
            }


Kemudian kita gunakan ``println!`` untuk mencetak ``Point`` yang kita buat.

.. code::

            fn main() {
                let integer = Point { x: 5, y: 10 };
                let float = Point { x: 1.0, y: 4.0 };

                println!("integer point = {:?}", integer);
                println!("float point = {:?}", float);
            }


Output:

.. code::

            integer point = Point { x: 5, y: 10 }
            float point = Point { x: 1.0, y: 4.0 }


Generics akan secara otomatis menentukan tipe data yang kita masukkan seperti integer pada variabel ``integer`` dan float pada variabel ``float``.

Kita juga dapat membuat instance ``Point`` seperti ini untuk mendefinisikan tipe yang ingin kita pakai secara lebih eksplisit:

.. code::

            let integer: Point<i32> = Point { x: 5, y: 10 };
            let float: Point<f64> = Point { x: 1.0, y: 4.0 };


Selalu ingat bahwa ``T`` hanya bisa dimasukkan satu tipe yang sama. Jika kita ingin membuat ``Point`` yang memiliki dua field dengan tipe data yang berbeda, kita bisa membuat struct seperti ini:

.. code::

            #[derive(Debug)]
            struct Point<T, U> {
                x: T,
                y: U,
            }


Dan kemudian membuat instancenya seperti ini:
    
.. code::

            let both_integer = Point { x: 5, y: 10 };
            let both_float = Point { x: 1.0, y: 4.0 };
            let integer_and_float = Point { x: 5, y: 4.0 };


Kita juga dapat menggunakan generics pada ``enum``

.. code::

            enum Enum<T> {
                One(T),
                Two,
            }
            
-------------------------------------
Trait Constraint dan Generic Function
-------------------------------------


Selain pada ``struct`` dan ``enum``, kita juga dapat menggunakan menggunakan generics pada fungsi. Kita akan membuat sebuah fungsi untuk membandingkan parameter ``a`` dan ``b`` dimana ``a`` dan ``b`` adalah sebuah generics ``T``.

.. code::

            fn compare<T>(a: T, b: T) -> T {
                if a > b {
                    return a;
                }
                b
            }

            fn main() {
                let a = 5;
                let b = 10;
                let comp = compare(a, b);
                println!("{}", comp);
            }
            
Pada kode diatas, yang akan terjadi adalah sebuah error seperti berikut:

.. code::

            error[E0369]: binary operation `>` cannot be applied to type `T`
              --> test.rs:22:10
               |
            22 |     if a > b {
               |        - ^ - T
               |        |
               |        T
               |
            help: consider restricting type parameter `T`
               |
            21 | fn compare<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
               |             ++++++++++++++++++++++
            

Error diatas menunjukkan kalau operator ``>`` tidak bisa digunakan untuk tipe ``T``. Hal ini terjadi karena compiler tidak dapat mengetahui apakah ``T`` memiliki implementasi operator ``>`` atau tidak. Untuk mengatasi ini, kita dapat menggunakan trait constraint untuk membatasi tipe ``T`` yang dapat digunakan pada fungsi ``compare`` seperti yang ditunjukan pada ``help: consider restricting type parameter **T**``.

.. code::

            use std::cmp::PartialOrd;

            fn compare<T: PartialOrd>(a: T, b: T) -> T {
                if a > b {
                    return a;
                }
                b
            }

            fn main() {
                let a = 5;
                let b = 10;
                let comp = compare(a, b);
                println!("{}", comp);
            }
            

Dan outputnya adalah ``10`` dari variabel ``b``.

Trait constraint ini berfungsi untuk membatasi suatu tipe generics. Logikanya adalah, hanya tipe yang sudah mengimplementasikan constraint tersebutlah yang dapat masuk ke dalam fungsi tersebut. Dengan demikian, kita dapat memastikan bahwa fungsi tersebut akan berjalan dengan baik. Pada kode diatas, ``T`` hanya bisa dimasukkan oleh tipe yang sudah mengimplementasikan trait ``PartialOrd``. Variabel ``a`` dan ``b`` yang berupa `i32` telah mengimplementasikan `PartialOrd` sehingga fungsi bekerja dengan sangat baik. Bila kita memasukkan tipe yang tidak mengimplementasikan ``PartialOrd`` seperti ``String``, maka fungsi tersebut akan error.

-----------------------------------
Generics Struct dan Implementation
-----------------------------------


Disini kita akan membuat sebuah trait ``Animal`` dan beberapa struct yang mengimplementasikan ``Animal``. Kemudian kita akan membuat satu struct lagi bernama ``Pet`` dimana ``Pet`` akan menerima ``T`` dimana ``T`` merupakan ``Animal``, lalu membuat method di dalam ``Pet`` yang akan memakai ``T`` untuk melakukan sesuatu.

.. code::

            trait Animal {
                fn name(&self) -> String;
            }

            struct Cat {
                name: String,
            }

            struct Dog {
                name: String,
            }

            struct Cow {
                name: String,
            }

            impl Animal for Cat {
                fn name(&self) -> String {
                    self.name.clone()
                }
            }

            impl Animal for Dog {
                fn name(&self) -> String {
                    self.name.clone()
                }
            }

            impl Animal for Cow {
                fn name(&self) -> String {
                    self.name.clone()
                }
            }

            struct Pet<T: Animal> {
                animal: T,
            }

            impl <T: Animal> Pet<T> {
                fn new(animal: T) -> Pet<T> {
                    Pet { animal }
                }

                fn name(&self) -> String {
                    self.animal.name()
                }

                fn pat(&self) {
                    println!("Mengelus {}", self.name());
                }
            }

            fn main() {
                let cat = Cat { name: String::from("Kitty") };
                let dog = Dog { name: String::from("Doggy") };
                let cow = Cow { name: String::from("Cowy") };

                let cat_pet = Pet::new(cat);
                let dog_pet = Pet::new(dog);
                let cow_pet = Pet::new(cow);

                println!("Cat name: {}", cat_pet.name());
                println!("Dog name: {}", dog_pet.name());
                println!("Cow name: {}", cow_pet.name());

                cat_pet.pat();
                dog_pet.pat();
                cow_pet.pat();
            }
            

Pada kode di atas, terlihat bahwa kita dapat memasukkan tipe apapun yang mengimplementasikan ``Animal``. Karena Rust memiliki trait-based generics, jadi manipulasi yang terjadi kepada tipe yang mengimplementasikan trait tersebut ada pada methodnya, yang didefinisikan didalam traitnya seperti method ``name()`` yang ada pada trait ``Animal`` yang kita pakai berulang kali di dalam implementasi ``Pet``. Karena itulah ada **constraint** atau batasan yaitu trait, dimana method yang dipakai harus berada dalam trait yang kita pakai sebagai constraint. Dengan begitu, kita dapat melakukan operasi yang bertujuan sama, namun implementasinya berbeda pada setiap tipe. Dengan demikian, kita dapat membuat sebuah fungsi yang dapat digunakan untuk tipe yang berbeda-beda, namun memiliki implementasi yang sama yang pada kode di atas, merupakan method dari ``Pet``.

# LATIHAN

TODO


