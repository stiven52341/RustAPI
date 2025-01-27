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
pub async fn get_unidades(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(Unidad, "SELECT * FROM public.unidad")
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

pub async fn insert_unidad(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<UnidadReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.unidad (nombre,estado) VALUES ($1, $2) RETURNING id",
        alimentacion.nombre,
        alimentacion.estado,

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

pub async fn update_unidad(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<UnidadUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.unidad set nombre = $1, estado = $2 where id = $3;";

    let row = sqlx::query(
        query
    )
    .bind(alimentacion_.nombre)
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
pub struct UnidadReq {
    nombre: String,
    estado: bool
}

#[derive(Deserialize)]
pub struct UnidadUpdateReq {
    id: i16,
    nombre: String,
    estado: bool
}

#[derive(Serialize)]
pub struct Unidad {
    id: i16,
    nombre: Option<String>,
    estado: Option<bool>
}

#[derive(Serialize)]
pub struct InsertRow {
    id: i32,
}
