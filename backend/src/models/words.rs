// use distance::hamming;
use crate::NUM_WORD_RESULTS;
use finalfusion::prelude::*;
use finalfusion::similarity::WordSimilarity;
use finalfusion::vocab::Vocab;
use serde::Deserialize;
use serde::Serialize;
use word_frequency::read_freq_csv;

pub struct WordModel {
    pub embedding: Embeddings<VocabWrap, StorageViewWrap>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IString {
    pub str: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WordResult {
    pub word: String,
    pub close_words: Vec<IString>,
    pub variants: Vec<IString>,
    pub frequency: Option<f64>,
}

impl WordResult {
    pub fn query(
        word: &str,
        embed: &Embeddings<VocabWrap, StorageViewWrap>,
    ) -> Result<Option<WordResult>, diesel::result::Error> {
        let hash_freq =
            read_freq_csv("data/Lexique383.csv").expect("The hashmap should be read properly");
        let frequency = hash_freq.get(&word.to_lowercase()).cloned();
        // skip quering if word is a number
        if let Ok(num) = word.parse::<i32>() {
            let close_words: Vec<_> = (1..500)
                .flat_map(|n| [num + n, num - n].into_iter())
                .map(|n| IString { str: n.to_string() })
                .collect();
            let word_res = WordResult {
                word: word.to_string(),
                close_words,
                variants: vec![],
                frequency: None,
            };
            Ok(Some(word_res))
        } else {
            let opt_results = embed.word_similarity(word, NUM_WORD_RESULTS);
            // iterate through text of the article
            if let Some(results) = opt_results {
                let word_res = results
                    .iter()
                    .map(|similarity_res| {
                        let str = similarity_res.word().to_string();
                        IString { str }
                    })
                    .collect();
                let variants = get_variants(word, &word_res);
                Ok(Some(WordResult {
                    word: word.to_string(),
                    close_words: word_res,
                    variants,
                    frequency,
                }))
            } else {
                Ok(None)
            }
        }
    }

    pub fn check(word: &str, embed: &Embeddings<VocabWrap, StorageViewWrap>) -> bool {
        let words_vocab = embed.vocab().words();
        words_vocab.iter().find(|w| w == &word).is_some()
    }

    pub fn query_multiple(
        words: &Vec<String>,
        embed: &Embeddings<VocabWrap, StorageViewWrap>,
    ) -> Result<Vec<Option<WordResult>>, diesel::result::Error> {
        words.iter().map(|word| Self::query(word, embed)).collect()
    }
}

fn get_variants(word: &str, word_res: &Vec<IString>) -> Vec<IString> {
    let iter_opt_variants = word_res
        .iter()
        .take(50)
        .flat_map(|istr| match same_root(istr, word) {
            true => [
                Some(IString {
                    str: istr.str.clone(),
                }),
                get_ligature_variants(&istr.str),
            ],
            false => [None, None],
        });
    std::iter::once(get_ligature_variants(word))
        .chain(iter_opt_variants)
        .filter_map(|variant| variant)
        .collect()
}

fn get_ligature_variants(word: &str) -> Option<IString> {
    if word.contains("oe") {
        Some(IString {
            str: str::replace(word, "oe", "œ"),
        })
    } else {
        None
    }
}

fn same_root(icandidate: &IString, word: &str) -> bool {
    let candidate = &icandidate.str;
    let l_candidate = candidate.len();
    let l_word = word.len();
    let distance = hamming_with_normal_size(candidate, word);
    let distance_f64 = distance as f64;
    let min_dist = std::cmp::max_by(l_candidate as f64, l_word as f64, |a, b| {
        a.partial_cmp(b).unwrap()
    });
    match min_dist {
        x if x == 0.0 => false,
        _ => {
            let normalized_dist = distance_f64 / min_dist;
            // println!("normalized_dist: {}", normalized_dist);
            normalized_dist < 0.4
        }
    }
}
fn hamming_with_normal_size(candidate: &str, word: &str) -> usize {
    let l_candidate = candidate.len();
    let l_word = word.len();
    // let mut word = word.clone();
    // TODO(leo: handle errors)
    let (candidate_cmp, word_cmp) = if l_candidate < l_word {
        let padding = l_word - l_candidate;
        let candidate_cmp = candidate
            .chars()
            .chain(std::iter::repeat(' ').take(padding))
            .collect::<String>();
        (candidate_cmp, word.to_string())
    } else {
        let padding = l_candidate - l_word;
        let word_cmp: String = word
            .chars()
            .chain(std::iter::repeat(' ').take(padding))
            .collect();
        (candidate.to_string(), word_cmp)
    };
    // println!("Compared strings: {}, {}", candidate_cmp, word_cmp);
    // println!("lenghts: {},{}", candidate_cmp.len(), word_cmp.len());
    let res_distance = hamming(&candidate_cmp, &word_cmp);
    // println!("res_distance: {:?}", res_distance);
    res_distance.unwrap()
}
fn hamming(candidate: &str, word: &str) -> Result<usize, String> {
    match candidate.chars().count() == candidate.chars().count() {
        true => {
            let dist = candidate
                .chars()
                .zip(word.chars())
                .map(|(c_candidate, c_word)| {
                    // println!("Compared chars: {}, {}", c_candidate, c_word);
                    match c_candidate == c_word {
                        false => 1,
                        true => 0,
                    }
                })
                .sum();
            Ok(dist)
        }
        false => Err("Error !".to_string()),
    }
}
