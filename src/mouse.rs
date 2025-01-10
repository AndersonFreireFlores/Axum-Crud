
use sqlx::{PgPool, Row};
use crate::models::{MouseCreate, MouseModel, MouseUpdate};
use std::error::Error as StdError;

pub async fn create_mouse(mouse: MouseCreate, x: i32, pool: &PgPool) -> Result<(),  Box<dyn StdError + Send + Sync>>{
    let query = "INSERT INTO mouse (id, model, brand, price, color) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(x)
        .bind(&mouse.model)
        .bind(&mouse.brand)
        .bind(&mouse.price)
        .bind(&mouse.color)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_mouse(mouse: MouseUpdate, x: i32, pool: &PgPool) ->  Result<(), Box<dyn StdError + Send + Sync>> {
    let query = "INSERT INTO mouse (id, model, brand, price, color) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(x)
        .bind(&mouse.model)
        .bind(&mouse.brand)
        .bind(&mouse.price)
        .bind(&mouse.color)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn mouse_id(id: i32, pool: &PgPool) -> Result<MouseModel, Box<dyn StdError>> {
    let row = sqlx::query("SELECT * FROM mouse WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let mouse = MouseModel {
        id: row.get(0),
        model: row.get(1),
        brand: row.get(2),
        price: row.get(3),
        color: row.get(4),
    };

    Ok(mouse)
}

pub async fn mouses(pool: &PgPool) -> Result<Vec<MouseModel>, Box<dyn StdError>> {
    let rows = sqlx::query("SELECT * FROM mouse")
        .fetch_all(pool)
        .await?;

    let mouses: Vec<MouseModel> = rows.into_iter().filter_map(|row| {
        Some(MouseModel {
            id: row.get(0),
            model: row.get(1),
            brand: row.get(2),
            price: row.get(3),
            color: row.get(4),
        })
    }).collect();

    Ok(mouses)
}