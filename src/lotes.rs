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
pub async fn get_lotes(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(Lote, "SELECT * FROM public.lote")
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

pub async fn insert_lotes(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<LoteReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.lote (nombre,id_tipo,cantidad_cerdos,fecha_creacion,estado) VALUES ($1, $2,$3,now(),true) RETURNING id",
        alimentacion.nombre,
        alimentacion.id_tipo,
        alimentacion.cantidad_cerdos

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

pub async fn update_lote(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<LoteUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.lote set nombre = $1, id_tipo = $2, fecha_creacion = $3, cantidad_cerdos = $4, estado = $5 where id = $6;";

    let row = sqlx::query(
        query
    )
    .bind(alimentacion_.nombre)
    .bind(alimentacion_.id_tipo)
    .bind(alimentacion_.fecha_creacion)
    .bind(alimentacion_.cantidad_cerdos)
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
        json!({"success": true, "message": "[true]"}).to_string()
    ))
}

#[derive(Deserialize)]
pub struct LoteReq {
    nombre: String,
    id_tipo: i16,
    cantidad_cerdos: i32,
    fecha_creacion: Option<NaiveDate>,
    estado: bool
}

#[derive(Deserialize)]
pub struct LoteUpdateReq {
    id: i16,
    nombre: String,
    id_tipo: i16,
    cantidad_cerdos: i32,
    fecha_creacion: NaiveDate,
    estado: bool
}

#[derive(Serialize)]
pub struct Lote {
    id: i16,
    nombre: Option<String>,
    id_tipo: Option<i16>,
    cantidad_cerdos: Option<i32>,
    fecha_creacion: Option<NaiveDate>,
    estado: Option<bool>
}

#[derive(Serialize)]
pub struct InsertRow {
    id: i32,
}
