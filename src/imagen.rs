use std::time::Duration;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
  };
  use chrono::NaiveDateTime;
  use chrono::NaiveDate;
  use serde::{Deserialize, Serialize};
  use serde_json::json;
  use sqlx::{postgres::{types::PgTimeTz, PgPoolOptions}, PgPool};
  use tokio::net::TcpListener;
pub async fn get_imagen(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(Imagen, "SELECT * FROM public.articulo_imagen")
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

pub async fn insert_imagen(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<ImagenReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.articulo_imagen (id_alimento,imagen) values ($1,$2) RETURNING id_alimento as id",
        alimentacion.id_alimento,
        alimentacion.imagen
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": 1}).to_string(),
    ))
}

pub async fn update_imagen(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<ImagenUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.articulo_imagen set imagen = $1 where id_alimento = $2;";

    let row = sqlx::query(
        &query
    )
    .bind(alimentacion_.imagen)
    .bind(alimentacion_.id_alimento)
    .execute(&db_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    });

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": [true]}).to_string(),
    ))
}

#[derive(Deserialize)]
pub struct ImagenReq {
    id_alimento: i32,
    imagen: String
}

#[derive(Deserialize)]
pub struct ImagenUpdateReq {
    id_alimento: i32,
    imagen: String
}

#[derive(Serialize)]
pub struct Imagen {
    id_alimento: i32,
    imagen: Option<String>
}

#[derive(Serialize)]
pub struct InsertRow {
    id: i32,
}
