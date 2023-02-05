static DEFAULT_NEWCULAR: &str = "      - N R K R N -
- - - B - - -
- - - B - - -
P - P - P - P
- - - - - - -
p - p - p - p
- - - b - - -
- - - b - - -
- n r k r n -";

pub enum GameType {
    Newcular,
}

impl GameType {
    pub fn to_string(&self) -> String {
        match self {
            Newcular => "newcular".to_string(),
        }
    }

    pub fn from_string(game_type: &str) -> Option<Self> {
        match game_type {
            "newcular" => Some(GameType::Newcular),
            _ => None,
        }
    }

    pub fn default_render(&self) -> &str {
        match self {
            Newcular => DEFAULT_NEWCULAR,
        }
    }
}
