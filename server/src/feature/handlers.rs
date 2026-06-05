use axum::extract::State;
use crate::{app_state::AppState, auth::model::AuthUser};

#[utoipa::path(
    get,
    path = "/feature",
    tag = "feature",
    responses(
        (status = 200, description = "Feature accessed successfully")
    )
)]
pub async fn get_feature(
    State(_app_state): State<AppState>,
) -> Result<&'static str, String> {
    Ok("feature response")
}

// Example of a protected route that requires authentication
// AuthUser in the parameters triggers the auth extractor
#[utoipa::path(
    get,
    path = "/feature/protected",
    tag = "feature",
    responses(
        (status = 200, description = "Protected feature accessed successfully"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearerAuth" = [])
    ),
)]
pub async fn get_protected_feature(
    State(_app_state): State<AppState>,
    AuthUser { id }: AuthUser,
) -> Result<String, String> {
    Ok(format!("user's id is: {}", id))
}