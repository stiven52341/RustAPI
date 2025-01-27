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
pub async fn get_tipo_alimento(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(TipoAlimento, "SELECT * FROM public.tipo_alimento")
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

pub async fn insert_tipo_alimento(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<TipoAlimentoReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.tipo_alimento (descr,estado) VALUES ($1, $2) RETURNING id",
        alimentacion.descr,
        alimentacion.estado
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
        json!({"success": true, "data": row}).to_string(),
    ))
}

pub async fn update_tipo_alimento(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<TipoAlimentoUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.tipo_alimento set descr = $1, estado = $2 where id = $3";

    let row = sqlx::query(
        query
    )
    .bind(alimentacion_.descr)
    .bind(alimentacion_.estado)
    .bind(alimentacion_.id)
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
        json!({"success": true, "data": "[true]"}).to_string(),
    ))
}

#[derive(Deserialize)]
pub struct TipoAlimentoReq {
    descr: String,
    estado: bool
}

#[derive(Deserialize)]
pub struct TipoAlimentoUpdateReq {
    id: i16,
    descr: String,
    estado: bool
}

#[derive(Serialize)]
pub struct TipoAlimento {
    id: i16,
    descr: Option<String>,
    estado: Option<bool>
}

#[derive(Serialize)]
pub struct InsertRow {
    id: i32,
}
