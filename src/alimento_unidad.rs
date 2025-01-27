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
  use sqlx::{postgres::{types::PgTimeTz, PgPoolOptions}, PgPool,types::BigDecimal};
  use tokio::net::TcpListener;
pub async fn get_alimento_unidad(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(AlimentoUnidad, "SELECT * FROM public.alimento_unidad")
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

pub async fn insert_alimento_unidad(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<AlimentoUnidadReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.alimento_unidad VALUES ($1, $2, $3,$4,$5) RETURNING id_unidad",
        alimentacion.id_unidad,
        alimentacion.id_alimento,
        alimentacion.precio,
        alimentacion.disponible,
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
        json!({"success": true, "data": row.id_unidad}).to_string(),
    ))
}

pub async fn update_alimento_unidad(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<AlimentoUnidadUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.alimento_unidad set precio = $1, disponible = $2, estado = true where id_unidad = $3 and id_alimento = $4";

    let row = sqlx::query(
        query
    )
    .bind(alimentacion_.precio)
    .bind(alimentacion_.disponible)
    .bind(alimentacion_.id_unidad)
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
pub struct AlimentoUnidadReq {
    id_unidad: i16,
    id_alimento: i32,
    precio: f64,
    disponible: f64,
    estado: bool
}

#[derive(Deserialize)]
pub struct AlimentoUnidadUpdateReq {
    id_unidad: i16,
    id_alimento: i32,
    precio: f64,
    disponible: f64,
    estado: bool
}

#[derive(Serialize)]
pub struct AlimentoUnidad {
    id_unidad: i16,
    id_alimento: i32,
    precio: f64,
    disponible: f64,
    estado: Option<bool>
}

#[derive(Serialize)]
pub struct InsertRow {
    id_unidad: i32,
}
