use axum::extract::State;
use crate::app_state::AppState;

pub async fn get_feature(
    State(_app_state): State<AppState>,
) -> Result<&'static str, String> {
    Ok("feature response")
}