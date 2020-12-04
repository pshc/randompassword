//! See https://xkcd.com/936/

use std::io::prelude::*;
use flate2::read::GzDecoder;
use rand::prelude::SliceRandom;

const WORDS_GZ: &'static [u8] = include_bytes!("words.txt.gz");

fn main() {
    let mut word_txt = String::new();
    {
        let mut decoder = GzDecoder::new(WORDS_GZ);
        decoder.read_to_string(&mut word_txt).unwrap();
    }
    let words: Vec<&str> = word_txt.lines().collect();
    let mut rng = rand::thread_rng();
    for i in 0..4 {
        if i > 0 {
            print!(" ");
        }
        print!("{}", words.choose(&mut rng).unwrap());
    }
    println!("");
}
