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
pub async fn get_cerdo(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(Cerdo, "SELECT * FROM public.cerdo")
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

pub async fn insert_cerdo(
    State(db_pool): State<PgPool>,
    Json(alimentacion): Json<CerdoReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let row = sqlx::query_as!(
        InsertRow,
        "INSERT INTO public.cerdo (id_lote,fecha_registro,id_etapa_vida,id_estado) VALUES ($1, $2,$3,$4) RETURNING id",
        alimentacion.id_lote,
        alimentacion.fecha_registro,
        alimentacion.id_etapa_vida,
        alimentacion.id_estado,
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

pub async fn update_cerdo(
    State(db_pool): State<PgPool>,
    Json(alimentacion_): Json<CerdoUpdateReq>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let query = "update public.cerdo set id_lote = ".to_owned() +&alimentacion_.id_lote.to_string()+", id_etapa_vida="+&alimentacion_.id_etapa_vida.to_string()+",id_estado="+&alimentacion_.id_estado.to_string()+" where id = "+&alimentacion_.id.to_string()+";";

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
pub struct CerdoReq {
    id_lote: i16,
    fecha_registro: Option<NaiveDate>,
    id_etapa_vida: i16,
    id_estado: Option<i16>
}

#[derive(Deserialize)]
pub struct CerdoUpdateReq {
    id: i32,
    id_lote: i16,
    fecha_registro: Option<NaiveDate>,
    id_etapa_vida: i16,
    id_estado: bool
}

#[derive(Serialize)]
pub struct Cerdo {
    id: i32,
    id_lote: Option<i16>,
    fecha_registro: NaiveDate,
    id_etapa_vida: Option<i16>,
    id_estado: Option<i16>
}

#[derive(Serialize)]
pub struct InsertRow {
    id: i32,
}
