use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use crate::models::{MouseCreate, MouseModel};
use crate::{mouse, AppState};

pub async fn get_all_mouses(State(poll): State<PgPool>) -> Result<Json<Vec<MouseModel>>, StatusCode> {

    let recs = match mouse::mouses(&poll).await {
        Ok(recs) => recs,
        Err(err) => {
            println!("Failed to get all users: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(recs))
}

pub async fn create_mouse(State(poll): State<PgPool>,
                          Json(payload): Json<MouseCreate>
                          ) ->
                            Result<Json<MouseModel>, StatusCode> {

    let mouse = MouseCreate{
        model: payload.model,
        brand: payload.brand,
        price: payload.price,
        color: payload.color,
    };


    match mouse::create_mouse(mouse, 1, &poll).await {
        Ok(_) => {
            let created_mouse = mouse::mouse_id(1, &poll).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Json(created_mouse))
        }
        Err(err) => {
            println!("Failed to create user: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}