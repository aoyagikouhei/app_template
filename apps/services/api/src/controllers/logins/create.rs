use std::{sync::Arc, time::Duration};

use axum::{Json, extract::State, response::IntoResponse};
use google_login::{Oauth2Client, request_user_info};
use tower_sessions::Session;

use crate::{
    AppState,
    controllers::{ResponseType, SESSION_PKCE_VERIFIER, SESSION_USER_INFO, make_response},
    error::ApiError,
};

pub async fn execute(
    State(state): State<Arc<AppState>>,
    session: Session,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    make_response(inner(&state.oauth2_client, payload, session).await)
}

async fn inner(
    oauth2_client: &Oauth2Client,
    payload: serde_json::Value,
    session: Session,
) -> Result<ResponseType, ApiError> {
    let Some(pkce) = session.remove::<String>(SESSION_PKCE_VERIFIER).await? else {
        return Err(ApiError::Invalid("PKCE verifier not found".to_string()));
    };
    let code = payload
        .get("code")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ApiError::Invalid("Code not found".to_string()))?;
    
    let _state = payload
        .get("state")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let res = oauth2_client
        .token(&pkce, code, Duration::from_secs(5))
        .await?;
    let user_info = request_user_info(&res.access_token, Duration::from_secs(5)).await?;
    session.insert(SESSION_USER_INFO, user_info).await?;
    Ok(ResponseType::new_no_data("success"))
}
