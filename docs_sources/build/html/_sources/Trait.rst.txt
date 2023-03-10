""""""""
Trait
""""""""

......
Trait
......

.. contents:: Overview
   :depth: 3



Trait pada bahasa Rust merupakan sebuah cara kita mencapai polymorphism. Dengan trait, kita dapat mendefiniskan perlakuan yang sama, untuk tipe-tipe yang berbeda. Trait pada Rust mirip dengan interface pada bahasa pemrograman lainnya, namun lebih powerful.

Pada ``trait``, kita dapat mendefinisikan method-method yang harus diimplementasikan oleh sebuah tipe, kemudian memanggil method-method tersebut pada tipe tersebut. Sebagai contoh, kita dapat mendefinisikan sebuah trait `Animal` yang memiliki method ``make_sound()``, kemudian kita dapat membuat sebuah struct ``Cat`` yang mengimplementasikan ``Animal``, dan kita dapat memanggil method ``make_sound()`` pada ``Cat``.

Kita akan mencoba membuat struct ``Cat`` yang mengimplementasikan trait ``Animal``. Pertama, kita akan mendefinisikan trait ``Animal`` dan struct ``Cat`` terlebih dahulu.

.. code::

            trait Animal {
                fn make_sound(&self);
            }

            struct Cat {
                name: String,
            }


Lalu kita akan mengimplementasikan trait ``Animal`` untuk struct ``Cat``.

.. code::

            impl Animal for Cat {
                fn make_sound(&self) {
                    println!("Meow!");
                }
            }

            
Dan terakhir, kita akan membuat sebuah instance dari struct ``Cat`` dan memanggil method ``make_sound()`` pada instance tersebut.

.. code::

            fn main() {
                let cat = Cat { name: String::from("Bacing") };
                cat.make_sound();
            }


Dengan begini, kita telah berhasil membuat sebuah trait dan mengimplementasikannya pada sebuah struct. Method ``make_sound()`` pada trait ``Animal`` dapat kita panggil pada instance dari struct ``Cat``.

Kalian pasti bertanya-tanya, kenapa kita tidak langsung saja membuat method ``make_sound()`` pada struct ``Cat``? Nah, memang tidak berarti untuk membuat satu ``trait`` untuk satu tipe saja. Karena itu, kita akan membuat satu tipe lagi yang akan mengimplementasikan ``Animal``.

.. code::

            struct Dog {
                name: String,
            }

            impl Animal for Dog {
                fn make_sound(&self) {
                    println!("Woof!");
                }
            }


Diatas, kita telah membuat struct ``Dog`` dan mengimplementasikan ``Animal`` untuk ``Dog``. Sekarang, kita akan membuat sebuah fungsi yang menerima sebuah tipe yang mengimplementasikan ``Animal`` sebagai parameter.


.. code::

            fn make_animal_sound(animal: &dyn Animal) {
                animal.make_sound();
            }
            


Kita akan membuat sebuah instance dari struct ``Cat`` dan ``Dog``, kemudian kita akan memanggil fungsi `make_animal_sound()` pada kedua instance tersebut.


.. code::

            fn main() {
                let cat = Cat { name: String::from("Bacing") };
                let dog = Dog { name: String::from("Baguk") };

                make_animal_sound(&cat);
                make_animal_sound(&dog);
            }


Dan hasilnya akan dikeluarkan sesuai dengan masing-masing implementasi dari method ``make_sound``. Untuk misalnya fungsi yang melakukan hal yang sama untuk banyak tipe, kita tidak perlu membuat banyak fungsi untuk masing-masing tipe. Cukup satu dimana perbedaan yang muncul akan tergantung dengan implementasi didalam tipe itu sendiri.

Keyword ``dyn`` diatas harus digunakan bila kita memakai ``trait`` sebagai parameter. Namun, ada cara yang lebih baik selain menggunakan keyword ``dyn`` terutama bila kita menginginkan untuk memakai lebih dari satu trait, yaitu dengan generics.


.. code::

            fn make_animal_sound<T: Animal>(animal: T) {
                animal.make_sound();
            }
            


Yang mana akan kita bahas dengan lebih lanjut di bagian advanced.

# LATIHAN

// TODO