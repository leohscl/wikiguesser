use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use finalfusion::prelude::*;
use finalfusion::similarity::WordSimilarity;
use crate::FILE_MODEL;

#[derive(Serialize)]
pub struct IString {
    pub str: String,
}

#[derive(Serialize)]
pub struct WordResult {
    word: String,
    results: Vec<IString>,
}
impl WordResult {
    pub fn query(word: &str) -> Result<WordResult, diesel::result::Error> {
        // let article = Article::get(id, connection)?;
        let fifu_file = FILE_MODEL;
        let mut reader = BufReader::new(File::open(&fifu_file).unwrap());
        let embed: Embeddings<VocabWrap, StorageViewWrap> = Embeddings::read_embeddings(&mut reader).unwrap();
        //TODO(leo): handle error !
        let results = embed.word_similarity(word, 1000).expect("Word query failed");
        // iterate through text of the article
        // println!("results: {:?}", results);
        //TODO(leo): check that we can discard score
        let word_res = results.iter().map(|similarity_res| {
            let str = similarity_res.word().to_string();
            IString{str}
        }).collect();
        Ok(WordResult{word:word.to_string(), results: word_res})
    }
}
