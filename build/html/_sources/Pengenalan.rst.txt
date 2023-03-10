""""""""""
Pengenalan
""""""""""


.. contents:: Overview
   :depth: 3
   
===================
Pengenalan Umum
===================
Rust merupakan bahasa pemrograman yang dikembangkan oleh Graydon Hoare dan disokong oleh `Mozilla Foundation`_ pada 2014 lalu. Rust memaksakan keamanan memory yang berarti, semua references pada Rust adalah valid, dan menunjuk pada memory yang benar-benar ada. Rust mencapainya tanpa penggunaan garbage collector.

Bagi pemula, bahkan developer berpengalaman sekalipun, Rust akan terasa agak menyulitkan di awal. Berikut adalah struktur kode Rust:

.. code-block::

		fn main(){
		// Entry Point
		}


===================
Instalasi dan Cargo
===================
----------------------
Instalasi Rust
----------------------
Cara Instalasi Rust yang paling umum adalah dengan menggunakan `rustup`_. Pada halaman web tersebut, OS kalian akan otomatis terdeteksi dan kalian dapat langsung mengikuti instruksi yang ada untuk menginstall Rust. Sebuah prompt akan bertanya pada anda tentang apa saja yang ingin anda install.

Bila kalian telah menginstall Rust lewat rustup, jalankanlah command berikut untuk mengecek instalasi dan versi dari Rust dan Cargo:

.. code::

	 cargo --version
 	 rustc --version

----------------
Cargo
----------------
Cargo merupakan package manager Rust. Cargo digunakan untuk mulai dari pembuatan project, kompilasi, hingga pemasangan package. 
       	- Pembuatan Package
       	- Kompilasi Program
    	- Cargo.toml
    	- Instalasi Package

Pembuatan Package
--------------------

Tanpa membuat pusing, package disini dapat kalian artikan sebagai project. Ini adalah cara untuk membuat sebuah project baru pada Rust. Untuk membuat sebuah package baru pada Rust, kita dapat memilih apakah package kita akan menjadi sebuah library, atau binary. By default, cargo akan membuat sebuah binary package. Karena itu, untuk membuat sebuah library, dibutuhkan flag ``--lib``.

.. code::

		cargo new --lib library_saya


Command diatas akan membuat sebuah package baru Rust dengan ``lib.rs`` didalamnya. Untuk membuat sebuah package binary, cukup buat sebuah project tanpa flag apapun, atau menggunakan ``--bin`` sebagai substitusi flag ``--lib`` diatas.

.. code::

		cargo new binary_saya
	

Project anda akan terstruktur seperti ini:

.. code::

		├── Cargo.toml
		└── src
    		 └── main.rs


Kompilasi Program
------------------

Untuk mengkompilasi seluruh project Rust, kita juga menggunakan cargo. Kita dapat membangun, mengecek error, ataupun langsung menjalankan sebuah project Rust bagi project binary.

Setelah menjalankan perintah ``cargo new binary_saya`` diatas, akan ada sebuah program hello world pada ``main.rs``. Kalian dapat menggunakannya untuk mengetes kompilasi Rust dengan menjalankan:

.. code::

		cargo run


Dan program akan langsung berjalan. Kalian juga dapat melakukan ``build`` tanpa menjalankannya langsung dengan command:

.. code::

		cargo build


Dan tergantung apakah build kalian merupakan ``release`` atau ``debug``, package kalian akan berada dalam folder ``target/``.

Yang terakhir, dikarenakan kompilasi program pada Rust cukup lama, bila ingin mengecek error, kita tidak harus membangun project kita berulang kali. Kita cukup menggunakan

.. code::

		cargo check


Untuk mengecek apakah ada error berlangsung.


Cargo.toml
--------------

File ``Cargo.toml`` merupakan sebuah file yang berisi semua detail tentang project kita mulai dari nama package, versi package, hingga nama author. Pada file ``Cargo.toml`` jugalah kita menuliskan nama-nama package dan versi package yang kita inginkan.


Instalasi Package
-------------------
Untuk menginstall package yang ingin kita gunakan pada sebuah project Rust, kita menuliskannya di `Cargo.toml`. Penulisannya ada dibawah tag `[dependencies]` seperti berikut:

.. code::

			toml
			[dependencies]
			serde = "1.0.140"


Dan package akan terinstall ketika kita menjalankan ``build`` atau ``run``. Yang kedua adalah, bila kita ingin menginstall package binary, maka kita dapat menggunakan command ``cargo install`` seperti berikut:

.. code::

			cargo install diesel_cli


.. _Mozilla Foundation: https://en.wikipedia.org/wiki/Mozilla_Foundation 

.. _rustup: https://rustup.rs/