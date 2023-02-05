CREATE TABLE IF NOT EXISTS players (
    id TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    token_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS games (
    id TEXT PRIMARY KEY,
    game_type TEXT NOT NULL,
    player_one TEXT NOT NULL,
    player_two TEXT NOT NULL,
    moves TEXT NOT NULL,
    render TEXT NOT NULL,
    winner TEXT,
    last_update TEXT NOT NULL,

    FOREIGN KEY (player_one) REFERENCES players(id),
    FOREIGN KEY (player_two) REFERENCES players(id)
);