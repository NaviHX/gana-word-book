use chrono;
use std::io::prelude::*;

pub fn add(dict_dir: &str, name: &str, romanji: &str, meaning: &str) {
    let mut dict = dict_dir.to_string();
    // let date = chrono::Utc::now().format("%Y-%m-%d");
    let date = chrono::Local::now().format("%Y-%m-%d");
    dict = dict + &date.to_string();
    let dict = std::path::Path::new(&dict);

    if !dict.exists() {
        std::fs::File::create(dict).expect("Cannot create dict file");
    }

    let mut dict = std::fs::OpenOptions::new()
        .append(true)
        .open(dict)
        .expect("Cannot open dict file");
    dict.write(format!("{},{},{}\n", name, romanji, meaning).as_bytes())
        .expect("Cannot write dict file");
}

pub fn list(dict_dir: &str, range: i32) {
    let mut dict = dict_dir.to_string();
    // let mut date = chrono::Utc::now();
    let mut date = chrono::Local::now();
    let mut wb = crate::word_bank::WordBank::new();

    match range >= 0 {
        true => {
            let date = date
                .checked_sub_signed(chrono::Duration::days(range as i64))
                .expect("date overflows")
                .format("%Y-%m-%d")
                .to_string();

            dict = dict + &date;
            let path = std::path::Path::new(&dict);
            match wb.read_from(path) {
                Ok(_) => {},
                Err(_) => {},
            }
        }
        false => {
            let r = -range;
            for _ in 0..=r {
                let path = format!("{}{}",dict,date.format("%Y-%m-%d").to_string());
                let path = std::path::Path::new(&path);

                date = match date.checked_sub_signed(chrono::Duration::days(1)) {
                    Some(d) => d,
                    None => { break; },
                };

                match wb.read_from(path) {
                    Ok(_) => {},
                    Err(_) => {},
                }
            }
        }
    }

    wb.list();
}
