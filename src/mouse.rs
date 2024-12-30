
use serde::de::Error;
use sqlx::Row;
use crate::models::{MouseCreate, MouseModel, MouseUpdate};

pub async fn create_mouse(mouse: MouseCreate,x:i32, poll: &sqlx::PgPool ) -> Result<(), Box<dyn Error>>{
    let query = "INSERT INTO mouse (id, model, brand, price, color) VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(x)
        .bind(&mouse.model)
        .bind(&mouse.brand)
        .bind(&mouse.price)
        .bind(&mouse.color)
        .execute(poll)
        .await?;

    Ok(())

}

pub async fn update_mouse(mouse: MouseUpdate,x:i32, pool: &sqlx::PgPool ) -> Result<(), Box<dyn Error>>{
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

pub async fn mouse_id(id:i32, pool: &sqlx::PgPool) -> Result<MouseModel, Box<dyn Error>>{
    let mouse = sqlx::query("SELECT * FROM mouse WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(mouse)
}

pub async fn mouses(poll: &sqlx::PgPool) -> Result<Vec<Option<MouseModel>>,  Box<dyn Error>>{
    let mouses = sqlx::query("SELECT * FROM mouse")
        .fetch_all(poll)
        .await?;
    let list_of_mouses = mouses.iter().map(|row| {
        let mouse = MouseModel {
            id: row.get(0),
            model: row.get(1),
            brand: row.get(2),
            price: row.get(3),
            color: row.get(4),
        };
        Some(mouse)
    }).collect();

    Ok(list_of_mouses)
}
