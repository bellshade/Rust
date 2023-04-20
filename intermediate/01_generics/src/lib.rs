trait MakhlukHidup {
    fn makan(&self) -> String;
}

struct Orang {
    nama: String,
}

struct Kucing {
    nama: String,
}

fn makan_makhluk<T: MakhlukHidup>(makhluk: T) -> String {
    makhluk.makan()
}

impl MakhlukHidup for Orang {
}

impl MakhlukHidup for Kucing {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start() {
        let orang = Orang {
            nama: String::from("Reimu"),
        };
        let kucing = Kucing {
            nama: String::from("Chen"),
        };

        let orang_makan = makan_makhluk(orang);
        let kucing_makan = makan_makhluk(kucing);

        assert_eq!(orang_makan, "Reimu makan nasi");
        assert_eq!(kucing_makan, "Chen makan whiskas");
    }
}
