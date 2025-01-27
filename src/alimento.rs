use std::time::Duration;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
  };
  use chrono::NaiveDateTime;
  use serde::{Deserialize, Serialize};
  use serde_json::json;
  use sqlx::{postgres::{types::PgTimeTz, PgPoolOptions}, PgPool};
  use tokio::net::TcpListener;
pub async fn get_alimento(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(Alimento, "SELECT * FROM public.alimento")
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

pub async fn insert_alimento(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<AlimentoReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.alimento (nombre,id_tipo) VALUES ($1, $2) RETURNING id",
        alimentacion.nombre,
        alimentacion.id_tipo,
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
        json!({"success": true, "data": row.id}).to_string(),
    ))
}

pub async fn update_alimento(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<AlimentoUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.alimento set nombre = $1, id_tipo = $2 where id = $3";

    let row = sqlx::query(
        query
        
    )
    .bind(alimentacion_.nombre)
    .bind(alimentacion_.id_tipo)
    .bind(alimentacion_.id)
    .execute(&db_pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success": false, "message": e.to_string()}).to_string(),
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": [true]}).to_string(),
    ))
}

#[derive(Deserialize)]
pub struct AlimentoReq {
    nombre: Option<String>,
    id_tipo: Option<i16>
}

#[derive(Deserialize)]
pub struct AlimentoUpdateReq {
    id: i32,
    nombre: Option<String>,
    id_tipo: Option<i16>
}

#[derive(Serialize)]
pub struct Alimento {
    id: i32,
    nombre: Option<String>,
    id_tipo: Option<i16>
}

#[derive(Serialize)]
pub struct InsertRow {
    id: i32,
}
