use std::time::Instant;

use actix_web::{error, web, Error as AWError};
use chrono::prelude::*;
use rusqlite::{Error as SqlError, Row, Statement, params};
use uuid::Uuid;

use crate::games::GameType;
use crate::models::{FullGame, Game};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub struct Txn {
    conn: Connection,
}

// fn retrieve_mapping_all<V>(stmt: Statement, mapping: dyn Fn(&Row) -> V) -> Result<Vec<V>> {
//     stmt.query_map([], mapping)?.collect()
// }

// pub async fn blind_read<V, F>(pool: &Pool, conn_func: F) -> Result<V, Error>
// where
//     V: Send + 'static,
//     F: Fn(Connection) -> Result<V, Error> + Send + 'static
// {
//     let pool = pool.clone();
//     let conn = web::block(move || pool.get())
//         .await?
//         .map_err(error::ErrorInternalServerError)?;

//     web::block(|| conn_func(conn))
//         .await?
//         // .map_err(|err| match err {
//         //     SqlError::QueryReturnedNoRows => error::ErrorNotFound(err),
//         //     _ => error::ErrorInternalServerError(err),
//         // })
// }

pub async fn retrieve_games(pool: &Pool) -> Result<Vec<Game>, AWError> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        conn.prepare(
            "SELECT g.id, g.game_type, g.render, g.last_update
                FROM games g",
        )?
        .query_map([], |row| {
            Ok(Game {
                id: row.get(0)?,
                game_type: row.get(1)?,
                render: row.get(2)?,
                last_update: row.get(3)?,
            })
        })
        .and_then(Iterator::collect)
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

pub async fn retrieve_game_by_id(pool: &Pool, game_id: String) -> Result<FullGame, AWError> {
    let pool = pool.clone();
    let mut conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        let tx = conn.transaction()?;
        tx.query_row(
            "SELECT
                    g.id, g.game_type, g.player_one, g.player_two,
                    g.moves, g.render, g.winner, g.last_update
                FROM games g
                WHERE g.id=?",
            [&game_id],
            |row| {
                let moves_str: String = row.get(4)?;
                let moves_vec = moves_str
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                Ok(FullGame {
                    id: row.get(0)?,
                    game_type: row.get(1)?,
                    player_one: row.get(2)?,
                    player_two: row.get(3)?,
                    moves: moves_vec,
                    render: row.get(5)?,
                    winner: row.get(6)?,
                    last_update: row.get(7)?,
                })
            },
        )
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

pub async fn create_game(
    pool: &Pool,
    player_one: String,
    player_two: String,
    game_type: GameType,
) -> Result<FullGame, AWError> {
    let pool = pool.clone();
    let mut conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        let game = FullGame {
            id: Uuid::new_v4().to_string(),
            game_type: game_type.to_string(),
            player_one,
            player_two,
            moves: vec![],
            winner: None,
            render: game_type.default_render().to_string(),
            last_update: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        let txn = conn.transaction()?;
        txn.execute(
            "INSERT INTO games
        (id, game_type, player_one, player_two, moves, winner, render, last_update)
        VALUES
        (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                game.id,
                game.game_type,
                game.player_one,
                game.player_two,
                game.moves.join(" "),
                game.winner,
                game.render,
                game.last_update,
            ],
        ).and_then(|_|Ok(game))
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

//let connection = sqlite::open(":memory:").unwrap();
