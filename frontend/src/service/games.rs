use super::fetch::Fetch;
use crate::entities::interfaces::Article;
use crate::entities::interfaces::OngoingGame;
use crate::entities::interfaces::Status;
use crate::API_URL;

pub async fn get_game() -> Result<Article, Status> {
    let url = format!("{}/games/get_or_create/none", API_URL);
    let res_json = Fetch::get(url).await;
    log::info!("json: {:?}", res_json);
    let ongoing_game = match res_json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<OngoingGame>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }?;
    log::info!("game: {:?}", ongoing_game.game);
    Ok(ongoing_game.article)
}
