// bead sort atau yang bisa dibilang juga gravity sort
// adalah algoritma yang  pada tahun 2002 yang diimplementasikan
// pada perangkat digital yang diyakini dengan pencapaian
// waktu sekitar O(n). namun penerapan algoritma ini lebih lambat
// dalam perangkat luna dan hanya dapat digunakan untuk
// mengurutkan daftar bilangan positif
// dan juga dalam kasusnya yang kebanyakan algoritma ini
// membutuhkan ruang sekitar O(n2).
pub fn bead_sort(a: &mut[usize]) {
    // mencari elemen max
    let mut maksimal = a[0];
    for i in 1..a.len() {
        if a[i] > maksimal {
            maksimal = a[i];
        }
    }

    // alokasikan memori
    let mut beads = vec![vec![0; max]; a.len()];

    // berikan tanda pada variable beads
    for i in 0..al.len() {
        for j in (0..a[i]).rev() {
            beads[i][j] = 1;
        }
    }

    // pindah ke bawah variable beads
    for  j in 0..max {
        let mut hasil = 0;
        for i in 0..a.len() {
            hasil += beads[i][j];
            beads[i][j] = 0;
        }

        for k in ((a.len() - sum)..a.len()).rev() {
            a[k] = j + 1;
        }
    }
}

#[cfg(test)]
// test hasil fungsi yang sudah dibuat
mod tests {
    use super::*;
    
    #[test]
    fn menurun() {
        let mut var1: [usize; 5] = [5, 4, 3, 2, 1];
        bead_sort(&mut var1);
        for i in 0..var1.len() - 1 {
            assert!(var1 <= var1[i + 1]);
        }
    }
}
