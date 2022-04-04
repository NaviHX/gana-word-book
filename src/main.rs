mod func;
mod word_bank;

extern crate csv;
use clap::{Parser, Subcommand};
use shellexpand;
use std::env::var;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use func::{add, list};

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    #[clap(subcommand)]
    sub: Subs,
}

#[derive(Subcommand)]
enum Subs {
    /// add a new word to dict
    Add {
        word: String,
        romanji: String,
        meaning: String,
    },

    /// print word list from a range of time
    List {
        /// Range can be positive or negative.
        /// When it is negative, word_book lists the dicts between today and <range> days before.
        /// Otherwise, word_book only lists the dict of <range> days before.
        /// If range is 0, word_book shows today's word list.
        #[clap(default_value_t = 0)]
        range: i32,
    },
}

fn main() {
    let config_path = format!("{}/.wordbook", var("HOME").unwrap()).to_string();
    let config_path = std::path::Path::new(&config_path);

    if !config_path.exists() {
        println!("It seems that you're trying word_book for the first time.");
        println!("Let's config the path which word_book stores words in! Don't forget the slash at the end of the path.");
        print!("I want to store here: ");
        io::stdout().flush().unwrap();

        let mut dir = String::new();
        io::stdin()
            .read_line(&mut dir)
            .expect("Cannot read your input");

        let mut config_file = File::create(config_path).expect("Cannot create config file");
        config_file
            .write(dir.as_bytes())
            .expect("Cannot write config file");
    }

    let mut dict_dir = String::new();
    File::open(config_path)
        .expect("Cannot open config file")
        .read_to_string(&mut dict_dir)
        .expect("Cannot read config file");
    dict_dir = shellexpand::env(&dict_dir)
        .expect("Cannot parse the path")
        .to_string()
        .trim()
        .to_string();

    let args = Args::parse();

    match args.sub {
        Subs::Add {
            word,
            romanji,
            meaning,
        } => {
            println!("Add a new word:");
            println!("word: {}", word);
            println!("romanji: {}", romanji);
            println!("meaning: {}", meaning);

            add(&dict_dir, &word, &romanji, &meaning);
        }
        Subs::List { range } => {
            list(&dict_dir, range.clone());
        }
    }
}
