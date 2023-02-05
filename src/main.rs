pub mod db;
pub mod games;
pub mod models;

use actix_web::{
    error::HttpError, get, middleware, post, web, App, HttpResponse, HttpServer, Responder,
};
use db::Pool;
use log::{debug, error, info, log_enabled, Level};
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

use crate::games::GameType;

#[derive(Serialize)]
pub struct ErrorReport {
    pub handle: Option<String>,
    pub external: Option<String>,
}

#[get("/games")]
async fn get_games(db: web::Data<Pool>) -> impl Responder {
    match db::retrieve_games(&db).await {
        Ok(games) => HttpResponse::Ok().json(games),
        Err(e) => {
            let uuid = Uuid::new_v4();
            error!("Err {}: {}", uuid, e);
            HttpResponse::InternalServerError().json(ErrorReport {
                handle: Some(uuid.to_string()),
                external: None,
            })
        }
    }
}

#[get("/games/{game_id:[a-z0-9-]+}")]
async fn get_game_by_id(db: web::Data<Pool>, path: web::Path<(String,)>) -> impl Responder {
    match db::retrieve_game_by_id(&db, path.0.clone()).await {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(e) => {
            let uuid = Uuid::new_v4();
            error!("Err {}: {}", uuid, e);
            HttpResponse::InternalServerError().json(ErrorReport {
                handle: Some(uuid.to_string()),
                external: None,
            })
        }
    }
}

#[derive(Deserialize)]
struct CreateReq {
    player_one: String,
    player_two: String,
    game_type: String,
    bearer_token: String,
}

#[post("/games")]
async fn create_game(db: web::Data<Pool>, create_msg: web::Json<CreateReq>) -> impl Responder {
    let game_type = match GameType::from_string(create_msg.game_type.as_str()) {
        Some(game_type) => game_type,
        None => {
            return HttpResponse::BadRequest().json(ErrorReport {
                handle: Some("".to_string()),
                external: Some("Invalid game_type".to_string()),
            })
        }
    };
    match db::create_game(
        &db,
        create_msg.player_one.clone(),
        create_msg.player_two.clone(),
        game_type,
    )
    .await
    {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(e) => {
            let uuid = Uuid::new_v4();
            error!("Err {}: {}", uuid, e);
            HttpResponse::InternalServerError().json(ErrorReport {
                handle: Some(uuid.to_string()),
                external: None,
            })
        }
    }
}

// #[get("/games/:game_id([a-z0-9-]+)/validMoves")]

// #[get("/games/:game_id([a-z0-9-]+)/moves/:move_id(\\d+)")]

// #[post("/games/:game_id([a-z0-9-]+)/moves/:move_id(\\d+)")]

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // connect to SQLite DB
    let manager = SqliteConnectionManager::file("foobar.db");
    let pool = Pool::builder().max_size(4).build(manager).unwrap();

    info!("starting HTTP server at http://localhost:8080");

    // start HTTP server
    HttpServer::new(move || {
        App::new()
            // store db pool as Data object
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(get_games)
            .service(get_game_by_id)

        // .service(web::resource("/asyncio_weather").route(web::get().to(asyncio_weather)))
        // .service(web::resource("/parallel_weather").route(web::get().to(parallel_weather)))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
