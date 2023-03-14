use crate::FILE_MODEL;

use crate::models::words::WordModel;
use finalfusion::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn get_word_model() -> WordModel {
    println!("Reading file..");
    let fifu_file = FILE_MODEL;
    let mut reader = BufReader::new(File::open(&fifu_file).unwrap());
    let embed: Embeddings<VocabWrap, StorageViewWrap> =
        Embeddings::read_embeddings(&mut reader).unwrap();
    WordModel { embedding: embed }
}
