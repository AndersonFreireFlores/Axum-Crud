use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Mouse{
    pub id: u64,
    pub model: String,
    pub brand: String,
    pub price: f64,
    pub color: String,
}