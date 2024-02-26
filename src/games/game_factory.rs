use crate::games::game_id::GameId;

//use super::game_html::{
//    morpion_html::MorpionHtml,
//    puissance4_html::Puissance4Html,
//    dames_html::DamesHtml
//};

pub struct GameFactory {
    pub id: String
}

impl GameFactory {
    pub fn get_games() -> Vec<GameId> {
        vec![
            GameId { id: "morpion".to_string(), name: "Morpion".to_string() },
            GameId { id: "puissance4".to_string(), name: "Puissance 4".to_string() },
            GameId { id: "lesdames".to_string(), name: "Les Dames".to_string() }
        ]
    }

    pub fn create_game_id(&self) -> Option<GameId> {
        GameFactory::get_games()
            .into_iter()
            .find(|game| game.id == self.id)
    }

    //pub fn create_game_html<T: Template>(&self) -> Option<&T> {
    //pub fn create_game_html<T: Template>(&self) -> T {
    //    let game_id = self.create_game_id().unwrap();

    //    //match self.id {
    //    //    1 => Some(&MorpionHtml::new(game_id)),
    //    //    2 => Some(&Puissance4Html::new(game_id)),
    //    //    3 => Some(&DamesHtml::new(game_id)),
    //    //    _ => None
    //    //}
    //    match self.id {
    //        1 => MorpionHtml::new(game_id),
    //        2 => Puissance4Html::new(game_id),
    //        3 => DamesHtml::new(game_id),
    //        _ => panic!("Test")
    //    }
    //}
}