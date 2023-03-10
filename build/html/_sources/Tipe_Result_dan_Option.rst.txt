
""""""""""""""""""""""
Tipe Result dan Option
""""""""""""""""""""""

.. contents:: Overview
   :depth: 3

======================
Result dan Option Type
======================

Pada Rust, kita tidak melakukan error handling secara konvensional dengan try-catch seperti bahasa lainnya, melainkan menggunakan sebuah tipe yang diambil dari bahasa pemrograman fungsional, yaitu `Result` type. Untuk `Option` type, Rust menggunakannya sebagai tipe yang merepresentasikan value yang dapat kosong atau `null`. Rust menerapkan `null safety` atau keamanan dari `null` dengan menggunakan `Option`.

-----------
Result Type
-----------

Pada Rust, Result type diimplementasikan menggunakan ``enum`` Deklarasi ``Result`` type pada standard library ``std::result`` adalah sebagai berikut:

.. code::

        enum Result<T, E> {
           Ok(T),
           Err(E),
        }
    

``T`` dan ``E`` diatas merupakan tipe generic dimana kita dapat memasukkan tipe apapun kedalam sana dimana ``T`` merupakan tipe dari nilai yang akan dikembalikan dan ``E`` merupakan tipe dari error yang dapat muncul. Untuk generics akan kita bahas lebih dalam nanti. Sekarang, kita berfokus terlebih dahulu pada ``Result`` type.

Sekarang, kita akan melihat contoh penggunaan ``Result``. Misalnya, kita memiliki file ``tes.txt`` dengan isi teks "Halo", lalu kita ingin membukanya.

.. code::

            use std::fs::File;
            use std::io::Read;

            fn main() {
                let mut file = match File::open("tes.txt") {
                    Ok(f) => f,
                    Err(_) => panic!("File tidak dapat dibuka"),
                };
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => {},
                    Err(e) => panic!("Tidak dapat membaca konten {}", e)
                };
                println!("{}", contents);
            }

Nah, seperti yang pernah dijelaskan, kita bisa memakai ``match`` untuk ``enum`` adan karena ``Result`` type merupakan ``enum``, kita bisa me-handle error dengan menggunakan ``match``. `Ok` akan me-wrap atau membungkus value yang ingin kita dapatkan, sedangkan ``Err`` akan membungkus value error. Untuk ignore atau mengabaikan variabel yang di-wrap dalam varian, kita dapat menggunakan underscore ``_``. Pada kode diatas, bila file ``tes.txt`` ditemukan, maka variabel ``f`` yang merupakan tipe ``File`` akan di-assign ke ``file``. Sedangkan bila file tidak ditemukan, maka program akan terhenti diakrenakan ``panic``. Untuk baris ``match file.read_to_string(&mut contents)``, kita menggunakan underscore didalam ``Ok`` dan kemudian tidak mengembalikan apa-apa, kita hanya mengisinya dengan kurung kurawal kosong ``{}`` dikarenakan method ``read_to_string`` akan mengisi variabel lain sehingga nilai dari ``read_to_string`` itu sendiri tidak dibutuhkan. Kita juga dapat menggunakan error yang terjadi dengan memakai variabel yang di-wrap dalam ``Err`` seperti ``e`` diatas.

Cara diatas sangatlah **awkward** dan terlalu berputar-putar. Seharusnya kita tidak perlu menggunakan `match` dan kemudian memasang ``panic`` seperti itu. Error handling dengan ``match`` harusnya dilakukan disaat kita harus me-handle error dengan lebih detail lagi, seperti mengembalikan value errornya dari fungsi, fallback ketika error terjadi, dan sebagainya. Untuk mengatasi hal seperti diatas, Rust memiliki beberapa method yang lebih praktis.


Unwrap
-------


``unwrap`` adalah sebuah method Rust dimana kita langsung mengambil variabel yang di-wrap didalam ``Ok`` pada ``Result`` type dan akan terjadi ``panic`` bila ``Err`` terjadi. Kita sebaiknya memakai ``unwrap`` bila sudah yakin bahwa error tidak akan terjadi dan pasti value ``Ok`` yang akan dikembalikan. Namun sebagai contoh, kita akan memakai unwrap untuk kode diatas.

.. code::

            use std::fs::File;
            use std::io::Read;

            fn main() {
                let mut file = File::open("tes.txt").unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                println!("{}", contents);
            }


Nah, kode ini hampir ekuivalen dengan kode diatas. Namun, ada cara method lain yang akan membuat kode sependek menggunakan ``unwrap``, namun ekuivalen dengan kode diatas.


Expect
-------

Dengan menggunakan method ``expect``, kita dapat menuliskan error kita sendiri bila ``panic`` terjadi. `expect` tidaklah berbeda dengan ``unwrap``, ia akan langsung mengambil value dalam ``Ok``, atau ``panic``. Namun, kita dapat menuliskan error kita sendiri.

.. code::

            use std::fs::File;
            use std::io::Read;

            fn main() {
                let mut file = File::open("tes.txt").expect("Tidak dapat membuka file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("Tidak dapat membaca file");
                println!("{}", contents);
            }


Kode ini ekuivalen dengan kode yang menggunakan ``match`` diatas. Bila ``panic`` terjadi, maka seperti kita menggunakan macro ``panic!``, pesan yang akan keluar adalah pesan yang kita tuliskan sendiri.

``?`` Operator
--------------

Rust memiliki sebuah operator khusus yang bisa digunakan untuk me-handle error dengan lebih praktis dan aman, yaitu dengan operator ``?``. Namun, untuk menggunakan operator ``?``, return type dari sebuah fungsi haruslah ``Result`` atau ``Option`` type, dan bagi ``Result``, error yang ada pada ``Result`` type yang dikembalikan harus mengimplementasikan ``From<T>`` yang akan kita bahas lebih lanjut nanti.

.. code::

            use std::fs::File;
            use std::io::{self, Read};

            fn main() -> io::Result<()> {
                let mut file = File::open("tes.txt")?;
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                println!("{}", contents);

                // Agar sesuai return type
                Ok(())
            }


Kode diatas merupakan contoh penggunaan ``?`` operator. ``?`` operator akan mengembalikan ``Err`` bila error terjadi, dan akan me-assign value yang berada dalam ``Ok`` kepada variabel yang bersangkutan dengan operator ``?``. Jadi ia berfungsi sama seperti semua method diatas, namun tidak ``panic`` bila error terjadi, tapi mengembalikan error tersebut. Untuk assignment value, ia akan assign seperti biasa bila sukses sehingga membuatnya ekuivalen dengan hal seperti ini:

.. code::

            let mut file = match File::open("tes.txt") {
                Ok(f) => f,
                Err(e) => return Err(e),
            };


------------
Option Type
------------


``Option`` type adalah enum, yang digunakan untuk null-safety dimana kita memakainya bila suatu value dapat memiliki kemungkinan untuk kosong. Definisi ``Option`` adalah sebagai berikut:

.. code::

            pub enum Option<T> {
                None,
                Some(T),
            }


Varian ``Some(T)`` merupakan varian yang berisi value bila value tersebut ada, dimana ``T`` adalah tipe generic yang mana value apapun dapat masuk kesana sedangkan ``None`` merupakan varian bila value tersebut kosong. Namun dengan `Option` type, value tersebut bukanlah ``null``, tapi ``enum`` ``Option`` tersebut.

Penggunaan ``Option`` type juga sama seperti ``Result`` type. Kita dapat menggunakan ``match``, ``unwrap``, ``expect``, maupun operator ``?`` untuk me-handle ``None``.

.. code::

            fn count_word(sentence: String, word: &str) -> Option<i32> {
                let mut count = 0;
                let vec = sentence.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

                for i in vec {
                    if i == word {
                        count += 1;
                    }
                }
                if count == 0 {
                    None
                }
                else {
                    Some(count)
                }
            }

            fn main() {
                let sentence = String::from("Aku sedang coding bahasa Rust dan bahasa Rust ini sangat keren");
                let count = count_word(sentence, "Rust");
                match count {
                    None => println!("Tidak ditemukan kata tersebut di kalimat tersebut"),
                    Some(value) => println!("Ditemukan {} kata tersebut di kalimat tersebut", value)
                };
            }



Kode diatas merupakan sebuah contoh penggunaan ``Option`` type dimana ``count_word`` merupakan fungsi yang menghitung jumlah kata tertentu didalam sebuah kalimat. Bila kata ditemukan, maka jumlah kata akan dikembalikan dalam varian ``Some`` dan bila tidak, varian ``None`` akan dikembalikan dari fungsi ``count_word``. Lalu pada fungsi ``main``, tergantung pada kondisi apakah varian yang dikembalikan adalah ``None`` atau ``Some``, output ``Tidak ditemukan kata tersebut di kalimat tersebut`` atau ``Ditemukan <jumlah kata> kata tersebut di kalimat tersebut`` akan dicetak.


Unwrap Or
---------

``unwrap_or`` merupakan sebuah method untuk melakukan ``unwrap`` terhadap ``Option`` type, namun tidak akan ``panic`` bila value yang di-unwrap ternyata ``None``, melainkan akan lari ke value default yang didefinisikan.

.. code::
           
            fn default_example(opt: Option<&str>) {
                println!("{}", opt.unwrap_or("Ini adalah value default"));
            }

            fn main() {
                default_example(Some("Test"));
                default_example(None);
            }


# Latihan

TODO


