# Panduan Berkontribusi

## Kontributor

Kami sangat senang dan berterima kasih bila anda ikut berkontribusi dalam repositori ini.
Semua boleh ikut berkontribusi walaupun hal kecil dengan pengecualian sebagai berikut:

- Hasil pekerjaan kamu adalah buatan kamu sendiri dan tidak ada hak cipta dari orang lain.
  - Jika ditemukan kesamaan, maka tidak akan kami `merge`.
- Hasil kerja kamu akan berlisensi [MIT](LICENSE) ketika permintaan pull kamu sudah di merged.
- Hasil kerja kamu wajib mengikuti standar dan style koding dari kami.
- Penggunaan nama file bersifat `snake_case` dan berlaku juga untuk variable dan identifier.
- Menggunakan output `println!`.
- Menghindari penggunaan library pada koding (jika dibutuhkan silahkan diskusi di [issue](https://github.com/bellshade/Rust/issues)).

## Apa Itu Algoritma?

Algoritma adalah langkah-langkah untuk menyelesaikan suatu pekerjaan di mana terdiri dari 3 bagian utama, yaitu:

- Input/masukan, sebelum menjalankan sebuah algoritma maka hal yang pertama harus dilakukan adalah menerima masukan, input dapat berasal dari pengguna ataupun dari langkah sebelumnya.
- Proses, bagian utama dari algoritma yang melakukan pengolahan input yang akan menghasilkan output.
- Output/keluaran, output adalah hasil dari bagian proses, output ini juga bisa digunakan untuk langkah selanjutnya (jika masih ada).

Algoritma harus dikemas sedemikian rupa sehingga memudahkan pembaca untuk memasukkannya ke dalam program yang lebih besar.

Algoritma harus:

- Memiliki nama kelas dan fungsi intuitif yang memperjelas tujuannya bagi pembaca
- Menggunakan konvensi penamaan Javascript dan nama variabel intuitif untuk memudahkan pemahaman
- Fleksibel untuk mengambil nilai input yang berbeda
- Memiliki docstrings dengan penjelasan yang jelas dan/atau URL ke materi sumber
- Berisi doctests yang menguji nilai input yang valid dan salah
- Kembalikan semua hasil perhitungan alih-alih mencetak atau memplotnya

``mod.rs`` harus berisi seperti :
```Rust
mod algoritma_saya;

pub use self::algoritma_saya::algoritma_saya;
```

``algoritma_saya.rs`` berisi testing hasil fungsi algoritma yang kamu buat sebagai contoh :
```Rust
pub fn algoritma_saya() {
  // contoh algoritma
}

#[cfg(test)]
mod test {
  #[test]
  fn testing_saya() {
    // isi dari testing
  }
}
```

Running dengan cara
- ``cargo test``
- ``cargo fmt``
- ``cargo clippy -all -- -D warnings

## Pull Request

### Pull Request Yang Baik
- Lakukan fork pada repositori kami
- setelah melakukan fork, kamu dibebaskan untuk mengubah atau menambah algoritma
  - Untuk pull reqquest merubah diusahakan kamu menerapkan algoritma yang lebih baik dan lebih mudah
- Setelah merubah, menambah, atau perbaikan dokumentasi, usahakn kamu membuat branch baru

```bash
git checkout -b <nama_branch>
git add .
git commit -m "add: menambahkan algoritma terbaru"
```

- Lakukan push ke branch kamu dn kemudian open pull request

### Pesan commit

Pesan / message commit harus mengikuti conventional commit. Kami menggunakan bot label agar tidak susah dalam labeling.
Berikut adalah jenis - jenis pesan commit.

- `fix` : untuk memperbaiki bug (label `bug`)
- `feat`: untuk menambhkan algoritma terbaru
- `docs`: untuk menambahahkan dokumentasi

Referensi:
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)

### Contoh penggunaan

```bash
git commit -m "docs: menambahkan dokumentasi"
```

```bash
git commit -m  "feat: menambahkan algoritma terbaru"
```

Pull request `merged` jika:

- Mengikuti standar dan arahan dari `CONTRIBUTING.md`
- Lulus test dan cek dari beberapa test yang sudah kami siapkan

## Tambahan

- Jika ada kendala atau masalah dalam pull request, kamu bisa laporkan masalah pada [issue](https://github.com/bellshade/Rust/issues)
- Jika ada test yang tidak lewat atau gagal, kami akan mengecek kembali perubahan.

Untuk pull request kami sarankan untuk menjelaskan secara detail yang kamu ubah atau tambahkan, dan bersikap sopan, serta selalu berterima kasih, itu salah satu bentuk tata krama yang baik terhadap sesama contributor dan programmer lainnya.terima kasih sudah berkontribusi di **Rust**.
