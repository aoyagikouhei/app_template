use axum::response::IntoResponse;
use serde_json::json;
use tower_sessions::Session;

use crate::{
    controllers::{ResponseType, make_response},
    error::ApiError,
};

pub async fn execute(session: Session) -> impl IntoResponse {
    make_response(inner(session).await)
}

async fn inner(session: Session) -> Result<ResponseType, ApiError> {
    session.clear().await;
    Ok(ResponseType::new_data("success", json!({})))
}
