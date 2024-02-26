#[derive(Clone)]
pub struct GameId {
    pub id: String,
    pub name: String,
}

impl GameId {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}