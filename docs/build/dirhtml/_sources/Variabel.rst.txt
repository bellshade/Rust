"""""""""
Variabel
"""""""""

.. contents:: Overview
   :depth: 3

==========
Variabel
==========

Variabel merupakan tempat penyimpanan data sementara. Pada Bahasa Rust, semua variabel secara default bersifat immutable atau tidak dapat dirubah. Untuk membuatnya menjadi mutable, dibutuhkan keyword ``mut``. Rust memiliki tiga cara untuk mendeklarasikan sebuah variabel.


-------------------- 
Constant Variables
--------------------

Constant Variables atau variabel konstan merupakan sebuah variabel yang tidak akan dapat diubah nilainya. Ia akan tetap, sampai kapan pun. Biasanya, konstanta digunakan untuk variabel global yang tidak memiliki kemungkinan untuk dapat berubah. Pendeklarasian variabel konstan diawali dengan tanda ``const``, diikuti dengan tipe data yang digunakan.

Contoh dari sebuah variabel konstan adalah sebagai berikut:

.. code::
			
			const PI: f32 = 3.14;

			fn main() {
    			// Kode
			}

------------------
Static Variables
------------------

Static Variables atau variabel statis merupakan tipe variabel yang sangat mirip dengan konstanta. Namun, Static variable masih memiliki kemungkinan untuk diubah, atau ``mutable``. Static variable memiliki lifetime ``'static`` dan masih dapat diubah menggunakan block ``unsafe``. Static variable ditandai dengan keyword ``static``.

.. code::

		// Mutable Static Variable
		static mut COUNTER: u32 = 0;

		// Immutable static variable
		static VAR: u32 = 0;

		fn main() {
     		// Kode
		}

-----------------
Local Variables
-----------------

Local variables atau variabel lokal merupakan tipe variabel yang paling umum. Local variable ini aman untuk dimutate menggunakan ``mut`` tanpa perlu menggunakan block ``unsafe``. Penggunaan local variable ini hanya dalam scopenya saja dan tidak dapat digunakan secara global. Local variable ditandai dengan keyword ``let``, dan tidak membutuhkan tipe data didefinisikan secara eksplisit.


.. code::

			fn main() {
    		// Mutable
    		let mut x = 5;
    		x += 10;
    		println!("{}", x);

    		// Immutable
    		let a = 10;
    		println!("{}", a);
			}


Latihan
---------

Pada bab 2 ini, kalian dapat melihat file ``lib.rs`` didalam folder ``src``. Didalam sana, ada sebuah test dengan variabel berikut:

.. code::

			let a = todo!();


Gantilah macro ``todo!`` tersebut dengan nilai yang sesuai pada macro ``assert_eq!``, kemudian jalankan ``cargo test`` untuk mengetes apakah jawaban kalian sudah benar atau belum.

Contoh cara mengerjakan:

..  code::

			// Sebelum

			let a = todo!();

			assert_eq!(20, a + 5);

			// Sesudah

			let a = 15;

			assert_eq!(20, a + 5);

			// cargo test

			running 1 test

	
			test tests::start ... ok

			test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

