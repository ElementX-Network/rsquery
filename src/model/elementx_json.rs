use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Map {
    pub world: String,
    pub display: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Player {
    pub current: u16,
    pub max: u16
}

#[derive(Clone, Debug, Deserialize)]
pub struct Game {
    pub room: String,
    pub name: String,
    pub private: bool,
    pub phase: String,
    pub map: Map,
    pub player: Player
}