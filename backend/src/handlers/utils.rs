use crate::FILE_MODEL;

use finalfusion::prelude::*;
use std::fs::File;
use std::io::BufReader;
use crate::models::words::WordModel;

pub fn get_word_model() -> WordModel {
    println!("Reading file..");
    let fifu_file = FILE_MODEL;
    let mut reader = BufReader::new(File::open(&fifu_file).unwrap());
    let embed: Embeddings<VocabWrap, StorageViewWrap> = Embeddings::read_embeddings(&mut reader).unwrap();
    WordModel{embedding: embed}
}
