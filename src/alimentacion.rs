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
pub async fn get_alimentaciones(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(Alimentacion, "SELECT * FROM public.alimentacion")
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

pub async fn insert_alimentacion(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<AlimentacionReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.alimentacion (id_cerdo,id_dieta,fecha) VALUES ($1, $2,$3) RETURNING id",
        alimentacion.id_cerdo,
        alimentacion.id_dieta,
        alimentacion.fecha
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

pub async fn update_alimentacion(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<AlimentacionUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.alimentacion set id_cerdo = ".to_owned() + &alimentacion_.id.to_string() + ", id_dieta = "+ &alimentacion_.id.to_string() +" where id = $3;";

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
pub struct AlimentacionReq {
    id_cerdo: Option<i32>,
    id_dieta: Option<i16>,
    fecha: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct AlimentacionUpdateReq {
    id: i32,
    id_cerdo: Option<i32>,
    id_dieta: Option<i16>
}

#[derive(Serialize)]
pub struct Alimentacion {
    id: i32,
    id_cerdo: Option<i32>,
    id_dieta: Option<i16>,
    fecha: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct InsertRow {
    id: i32,
}
