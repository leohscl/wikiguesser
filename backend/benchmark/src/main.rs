use clap::{arg, Command};
use std::fs::File;
use std::io::BufReader;
use finalfusion::prelude::*;
use finalfusion::similarity::WordSimilarity;

fn main() {
    let matches = Command::new("MyApp")
        .version("1.0")
        .arg(arg!(--num_res <VALUE>).required(true).value_parser(clap::value_parser!(usize)))
        .arg(arg!(--num_runs <VALUE>).required(true).value_parser(clap::value_parser!(usize)))
        .get_matches();

    let num_res = matches.get_one::<usize>("num_res").expect("required");
    let num_runs = matches.get_one::<usize>("num_runs").expect("required");
    query("pain", *num_res, *num_runs);
}

pub fn query(word: &str, num_res: usize, num_runs: usize) {
    let fifu_file = "../data/wiki_new_model_fr.fifu";
    let mut reader = BufReader::new(File::open(&fifu_file).unwrap());
    let embed: Embeddings<VocabWrap, StorageViewWrap> = Embeddings::read_embeddings(&mut reader).unwrap();
    for _ in 0..num_runs {
        let results = embed.word_similarity(word, num_res).expect("Word query failed");
        println!("results: {:?}", results);
    }
}
