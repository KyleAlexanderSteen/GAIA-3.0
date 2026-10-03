//! Earth query. Not a live feed.

use axum::{extract::Query, Json};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct EarthQuery {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

pub async fn earth(Query(query): Query<EarthQuery>) -> Json<Value> {
    Json(json!({
        "lat": query.lat,
        "lon": query.lon,
        "variable": "temperature",
        "live": false,
        "reason": "no NOAA reading"
    }))
}

pub async fn submit_v1() -> Json<Value> {
    Json(json!({ "intent_id": "local-1", "signed": false }))
}
