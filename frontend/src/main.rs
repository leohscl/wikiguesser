use crate::app::App;
use dotenv_codegen::dotenv;
pub mod app;
pub mod hamming;
// pub mod components;
pub mod entities;
pub mod service;
// pub mod routes;
// pub mod utils;

// Constants
const API_URL: &str = dotenv!("API_URL");

// use std::fs::File;
// use std::io::BufReader;
// use finalfusion::prelude::*;
// use finalfusion::similarity::WordSimilarity;
fn main() {
    // let fifu_file = "data/corrected_model_fr.fifu";
    // let mut reader = BufReader::new(File::open(&fifu_file).unwrap());
    // let embed: Embeddings<VocabWrap, StorageViewWrap> = Embeddings::read_embeddings(&mut reader).unwrap();
    // let results = embed.word_similarity("pain", 10).unwrap();
    // println!("results: {:?}", results);
    // let title = "Affaire Grégory".to_string();
//     let page_content = "L'affaire Grégory Villemin — ou affaire (du petit) Grégory, ou affaire
// Villemin — est une affaire criminelle française qui débute le 16 octobre 1984 en fin d'après-midi,
// lorsque Christine Villemin signale la disparition de Grégory, son fils de quatre ans, du domicile
// familial, situé à Lépanges-sur-Vologne. Le même jour, le corps sans vie de l'enfant est retrouvé
// vers 21 h 15 à près de sept kilomètres de là, dans la Vologne, une rivière des Vosges. La
// photographie du repêchage du corps de Grégory, les pieds, les mains et la tête liés par des
// cordelettes et un bonnet de laine rabattu sur le visage paraît dans la presse et marque d'emblée
// l'opinion publique. L'affaire attire rapidement de nombreux journalistes, français puis étrangers
// et, dès le surlendemain, fait la une de la presse nationale. En raison du rôle que les médias ont
// joué à cette occasion, une grande partie d'entre eux se verront reprocher tant leur traitement «
// feuilletonnesque » de l'affaire que leur manque d'objectivité et leur intrusion inadmissible à la
// fois dans la vie privée des intéressés et dans l'enquête judiciaire. Cet emballement médiatique a
// gravement nui à la sérénité et à l'objectivité des investigations. Certains des protagonistes ont
// reconnu plus tard les difficultés qu'ils avaient rencontrées à l'époque pour rester cantonnés dans
// leur rôle propre (selon le cas, enquêteur ou journaliste). ".to_string();
//     let page_props = PageProps{title, content:page_content};
    wasm_logger::init(wasm_logger::Config::default());
    // let article_props = ArticleProps{id:45288};
    yew::start_app::<App>();
    // yew::start_app_with_props::<App>(article_props);
    // log::info!("Hello world");
}


