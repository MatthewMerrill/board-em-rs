use std::time::Instant;

use serde::{Serialize};

#[derive(Serialize)]
pub struct Game {
    pub id: String,
    pub game_type: String,
    pub render: String,
    pub last_update: String,
}

#[derive(Serialize)]
pub struct FullGame {
    pub id: String,
    pub game_type: String,
    pub player_one: String,
    pub player_two: String,
    pub moves: Vec<String>,
    pub winner: Option<String>,
    pub render: String,
    pub last_update: String,
}

// #[derive(Serialize)]
// pub struct Move {
//     pub id: String,
//     pub game_id: String,
//     pub player_id: String,
//     pub player_index: String,
//     pub render: String,
// }

// #[derive(Serialize)]
// pub struct Player {
//     pub id: String,
//     pub display_name: String,
// }

#[derive(Serialize)]
pub struct PlayerAuth {
    pub id: String,
    pub display_name: String,
    pub token: Option<String>,
    pub token_hash: Option<String>,
    pub token_expire: Option<String>,
}