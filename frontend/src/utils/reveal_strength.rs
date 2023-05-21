use crate::components::guessing_page::Prereveal;
use crate::entities::interfaces::WordResult;
use crate::entities::interfaces::{GameEngine, StringAndPos};
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RevealStrength {
    Revealed,
    VeryClose(StringAndPos),
    Close(StringAndPos),
    Distant(StringAndPos),
    NotRevealed,
}

pub fn initialize_revealed_vector(
    vec_text: &Vec<String>,
    prereveal: Prereveal,
    game_engine: Option<GameEngine>,
    protected: Option<&Vec<String>>,
) -> Vec<RevealStrength> {
    // We need some info on frequency, and on protected status
    if let Some(game_eng) = game_engine {
        let mut hash_res = HashMap::new();
        for word_res in game_eng
            .list_results_content
            .clone()
            .into_iter()
            .filter_map(|x| x)
        {
            hash_res.insert(word_res.word.clone(), word_res);
        }
        vec_text
            .iter()
            .map(|word| match word.chars().count() <= 1 {
                true => RevealStrength::Revealed,
                false => {
                    if let Some(word_res) = hash_res.get(word) {
                        get_reveal_status(word_res, prereveal.clone(), protected)
                    } else {
                        RevealStrength::NotRevealed
                    }
                }
            })
            .collect()
    } else {
        vec_text
            .iter()
            .map(|str| match str.chars().count() <= 1 {
                true => RevealStrength::Revealed,
                false => RevealStrength::NotRevealed,
            })
            .collect()
    }
}

fn _initialize_revealed_vector(vec_text: &Vec<String>) -> Vec<RevealStrength> {
    //TODO(léo): handle all pre_revealed words ?
    let determinants = vec!["le", "la", "les", "un", "une", "des"];
    let pronoms = vec!["ce", "ces", "de", "du"];
    let avoir_conj = vec!["eu", "aura", "a"];
    let etre_conj = vec!["était", "sera", "est"];
    let conjonction_coord = vec!["et", "en"];
    let pre_revealed: Vec<_> = [
        determinants,
        pronoms,
        avoir_conj,
        etre_conj,
        conjonction_coord,
    ]
    .concat();
    vec_text
        .iter()
        .map(|str| match str.chars().count() <= 1 {
            true => RevealStrength::Revealed,
            false => {
                if let Some(_) = pre_revealed
                    .iter()
                    .position(|candidate| candidate.to_lowercase() == str.to_lowercase())
                {
                    RevealStrength::Revealed
                } else {
                    RevealStrength::NotRevealed
                }
            }
        })
        .collect()
}

fn get_reveal_status(
    word_res: &WordResult,
    prereveal: Prereveal,
    opt_protected: Option<&Vec<String>>,
) -> RevealStrength {
    let word = word_res.word.clone();
    let opt_frequency = word_res.frequency;
    // log::info!("word: {}, frequency: {:?}", word, opt_frequency);
    // check if word is protected, too close to the title
    if let Some(protected) = opt_protected {
        if protected.contains(&word) {
            // log::info!("protected_word: {}", word);
            return RevealStrength::NotRevealed;
        }
    }
    // else resume as normal
    if let Some(frequency) = opt_frequency {
        match prereveal {
            Prereveal::Under(freq_threshold) => {
                if frequency < freq_threshold {
                    RevealStrength::Revealed
                } else {
                    RevealStrength::NotRevealed
                }
            }
            Prereveal::Over(freq_threshold) => {
                if frequency > freq_threshold {
                    RevealStrength::Revealed
                } else {
                    RevealStrength::NotRevealed
                }
            }
            Prereveal::OverAndHintUnder(over, hint_under) => {
                if frequency > over {
                    RevealStrength::Revealed
                } else if frequency < hint_under {
                    let mut rng = rand::thread_rng();
                    let random_rank = rng.gen_range(6..20);
                    let hint = word_res.close_words[random_rank].str.clone();
                    let string_pos = StringAndPos { str: hint, pos: 10 };
                    RevealStrength::Close(string_pos)
                } else {
                    RevealStrength::NotRevealed
                }
            }
            _ => RevealStrength::NotRevealed,
        }
    } else {
        RevealStrength::NotRevealed
    }
}
