use std::error::Error;

pub struct WordBank {
    words: Vec<Word>,
}

struct Word {
    word: String,
    romanji: String,
    meaning: String,
}

impl WordBank {

    /// create a word bank with nothing
    pub fn new() -> WordBank {
        WordBank {
            words: vec![],
        }
    }

    /// read a file and append to the word bank
    pub fn read_from(&mut self,file_path: &std::path::Path) -> Result<(),Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(file_path).expect("Cannot read dict file");
        let mut line_number = 0;
        
        for result in rdr.records() {
            match result {
                Err(_) => eprintln!("Line {} corrupted", line_number),
                Ok(rec) => {
                    if rec.len() < 3 {
                        eprintln!("Line {} corrupted", line_number);
                    } else {
                        self.words.push(Word {
                            word: rec[0].to_string(),
                            romanji: rec[1].to_string(),
                            meaning: rec[2].to_string(),
                        })
                    }
                }
            }

            line_number = line_number + 1;
        }

        Ok(())
    }

    /// list all words in stdin
    pub fn list(&self) {
        println!("WORD\tROMANJI\tMEAN");
        for w in &self.words {
            println!("{}\t{}\t{}",w.word,w.romanji,w.meaning);
        }
    }

}