use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize,Serialize, sqlx::FromRow)]
pub struct MouseModel{
    pub id: i64,
    pub model: String,
    pub brand: String,
    pub price: f64,
    pub color: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseCreate{
    pub model: String,
    pub brand: String,
    pub price: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MouseUpdate{
    pub model: Option<String>,
    pub brand: Option<String>,
    pub price: Option<f64>,
    pub color: Option<String>,
}


