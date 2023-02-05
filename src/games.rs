use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GameType {
    pub name: String,
    pub webhook: String,
}

#[derive(Serialize)]
pub struct GameSummary {
    pub valid_moves: Vec<String>,
    pub render: String,
    pub winner: Option<i8>,
}

static registry_string : &str = "[{\"name\":\"newcular\",\"webhook\":\"http://localhost:8181/gameType/newcular/summary/\"}";

static registry: HashMap<String, GameType> =
    serde_json::from_str::<Vec<GameType>>(registry_string)
        .unwrap().iter().map(|&game|(game.name, game)).collect();

impl GameType {
    pub fn from_string(game_type: &str) -> Option<&GameType> {
        registry.get(game_type)
    }

    pub fn summary(&self, moves: Vec<String>) -> GameSummary {
        
    }
}
