use chrono::{Duration, Utc};

pub fn generate_token(user_id: &str, secret: &str, duration_minutes: i64) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(duration_minutes))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = crate::auth::model::Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
}