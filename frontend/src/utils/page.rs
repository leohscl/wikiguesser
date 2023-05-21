use crate::components::guessing_page::Prereveal;
use crate::components::hidden_field::Category;
use crate::entities::hidden_text::HiddenText;
use crate::entities::interfaces::{Article, WordResult};
use crate::entities::interfaces::{GameEngine, StringAndPos};
use crate::utils::reveal_strength::initialize_revealed_vector;
use crate::utils::reveal_strength::RevealStrength;
use wiki_process::wiki_parse::create_string_vector;

#[derive(Clone, PartialEq)]
pub struct Page {
    pub title: HiddenText,
    pub content: HiddenText,
    pub input: String,
}

impl Page {
    pub fn reveal_with_engine(&mut self, word: &str, result_engine: &Vec<StringAndPos>) -> bool {
        let title_fully_revealed = self.title.reveal_with_engine(word, result_engine);
        self.content.reveal_with_engine(word, result_engine);
        title_fully_revealed
    }
    pub fn reveal(&mut self, word_res: &WordResult) -> bool {
        let title_fully_revealed = self.title.reveal(word_res);
        self.content.reveal(word_res);
        title_fully_revealed
    }
    pub fn reveal_all(&mut self) {
        self.title.reveal_all();
        self.content.reveal_all();
    }
}

pub fn page_from_json(
    article: Article,
    prereveal: Prereveal,
    game_engine: Option<GameEngine>,
) -> Page {
    let title = String::from(article.title + " ");
    let content = String::from(article.content + " ");
    let title_vec = create_string_vector(&title);
    let content_vec = create_string_vector(&content.clone());
    let protected_words = get_protected(&game_engine, &title_vec);
    let revealed_title = initialize_revealed_vector(&title_vec, Prereveal::Base, None, None);
    let revealed_content =
        initialize_revealed_vector(&content_vec, prereveal, game_engine, Some(&protected_words));
    let title_vec_len = title_vec.len();
    let content_vec_len = content_vec.len();
    let hidden_title = HiddenText {
        is_title: true,
        text: title_vec,
        revealed: revealed_title,
        new_revelations: vec![RevealStrength::NotRevealed; title_vec_len],
        categories: vec![Category::Normal; title_vec_len],
        fully_revealed: false,
    };

    let importance_content: Vec<Category> = content_vec
        .iter()
        .map(|word| {
            if protected_words.contains(word) {
                Category::Important
            } else {
                Category::Normal
            }
        })
        .collect();

    let hidden_content = HiddenText {
        is_title: false,
        text: content_vec,
        revealed: revealed_content,
        new_revelations: vec![RevealStrength::NotRevealed; content_vec_len],
        categories: importance_content,
        fully_revealed: false,
    };
    Page {
        title: hidden_title,
        content: hidden_content,
        input: "".to_string(),
    }
}

fn get_protected(opt_game_engine: &Option<GameEngine>, title_vec: &Vec<String>) -> Vec<String> {
    let common = super::similar_word::COMMON_WORDS;
    let top_num = 200;
    let game_engine = opt_game_engine
        .clone()
        .expect("There should be a game engine !");
    let close_words_title: Vec<String> = game_engine
        .list_results_title
        .into_iter()
        .map(|opt_word_res| {
            if let Some(word_res) = opt_word_res {
                word_res
                    .close_words
                    .into_iter()
                    .take(top_num)
                    .map(|istr| istr.str.clone())
                    .collect()
            } else {
                Vec::new()
            }
        })
        .flatten()
        .collect();
    let mut all_protected: Vec<String> = title_vec
        .clone()
        .into_iter()
        .chain(close_words_title.into_iter())
        .collect();
    all_protected.retain(|protected| {
        let prot: &str = &*protected;
        !common.contains(&prot)
    });
    // log::info!("all protected: {:?}", all_protected);
    all_protected
}
