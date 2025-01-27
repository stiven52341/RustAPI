use std::time::Duration;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
  };
  use chrono::NaiveDate;
  use serde::{Deserialize, Serialize};
  use serde_json::json;
  use sqlx::{postgres::{types::PgTimeTz, PgPoolOptions}, PgPool};
  use tokio::net::TcpListener;
pub async fn get_tipo_lote(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(TipoLote, "SELECT * FROM public.tipo_lote")
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

pub async fn insert_tipo_lote(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<TipoLoteReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.tipo_lote (descr,estado) VALUES ($1, $2) RETURNING id",
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

pub async fn update_tipo_lote(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<TipoLoteUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.lote set descr = ".to_owned() + &alimentacion_.descr.to_string() + ", estado = "+ &alimentacion_.estado.to_string() +" where id = "+&alimentacion_.id.to_string()+";";

    let row = sqlx::query_as(
        &query,
        
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
        StatusCode::OK,
        json!({"success": true, "data": row}).to_string(),
    ))
}

#[derive(Deserialize)]
pub struct TipoLoteReq {
    descr: String,
    estado: bool
}

#[derive(Deserialize)]
pub struct TipoLoteUpdateReq {
    id: i16,
    descr: String,
    estado: bool
}

#[derive(Serialize)]
pub struct TipoLote {
    id: i16,
    descr: Option<String>,
    estado: Option<bool>
}

#[derive(Serialize)]
pub struct InsertRow {
    id: i32,
}
