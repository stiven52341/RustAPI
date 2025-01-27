use std::time::Duration;

mod alimentacion;
mod alimento;
mod alimento_unidad;
mod cerdo;
mod lotes;
mod tipolote;
mod tipo_alimento;
mod unidad;

use alimento::{get_alimento, insert_alimento, update_alimento};
use alimento_unidad::{get_alimento_unidad, insert_alimento_unidad, update_alimento_unidad};
use axum::{
  extract::{Path, State},
  http::StatusCode,
  routing::{get, patch,post},
  Json, Router,
};
use chrono::NaiveDateTime;
use lotes::{get_lotes, insert_lotes, update_lote};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::{types::PgTimeTz, PgPoolOptions}, PgPool};
use tipo_alimento::{get_tipo_alimento, insert_tipo_alimento};
use tipolote::{get_tipo_lote, insert_tipo_lote};
use tokio::net::TcpListener;
use unidad::{get_unidades, insert_unidad};

#[tokio::main]
async fn main() {
  //expose environment variables from .env file
  dotenvy::dotenv().expect("Unable to access .env file");

  //set variables from enviroment variables
  let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:13534".to_owned());
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

  //create our database pool
  let db_pool = PgPoolOptions::new()
    .max_connections(64)
    .acquire_timeout(Duration::from_secs(5))
    .connect(&database_url)
    .await
    .expect("can't connect to database");

  //create our tcp listener
  let listener = TcpListener::bind(server_address)
    .await
    .expect("Could not create tcp listener");

  println!("listening on {}", listener.local_addr().unwrap());

  // compose the routes
  let app = Router::new()
    .route("/", get(|| async { "Hello world" }))
    .route("/alimentacion", get(alimentacion::get_alimentaciones).post(alimentacion::insert_alimentacion))
    .route("/alimentacion/update", post(alimentacion::update_alimentacion))
    .route("/alimento",get(get_alimento).post(insert_alimento))
    .route("/alimento/update",post(update_alimento))
    .route("/alimento_unidad", get(get_alimento_unidad).post(insert_alimento_unidad))
    .route("/alimento_unidad/update", post(update_alimento_unidad))
    .route("/usuario/login", post(login))
    .route("/lotes", get(get_lotes).post(insert_lotes))
    .route("/lotes/update", post(update_lote))
    .route("/tipolote",get(get_tipo_lote).post(insert_tipo_lote))
    .route("/tipo-alimento", get(get_tipo_alimento).post(insert_tipo_alimento))
    .route("/unidades", get(get_unidades).post(insert_unidad))
    .with_state(db_pool);

  //serve the application
  axum::serve(listener, app)
    .await
    .expect("Error serving application");
}

pub async fn login(
  State(db_pool): State<PgPool>,
  Json(login): Json<LoginReq>
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let rows = sqlx::query_as!(Usuario, "SELECT * FROM public.usuario where username = $1 and password = $2",login.username, login.password)
      .fetch_all(&db_pool)
      .await
      .map_err(|e| {
          (
              StatusCode::INTERNAL_SERVER_ERROR,
              json!({"success": false, "message": e.to_string()}).to_string(),
          )
      })?;

  Ok((
      StatusCode::OK,
      json!({"success": true, "data": rows}).to_string(),
  ))
}

#[derive(Serialize)]
struct Usuario{
  id: i32,
  nombre: String,
  ocupacion: String,
  estado: Option<bool>,
  username: String,
  password: String
}

#[derive(Deserialize)]
struct LoginReq{
  username: String,
  password: String
}

