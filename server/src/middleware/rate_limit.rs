use tower_governor::{
    GovernorLayer, 
    governor::GovernorConfigBuilder, 
    key_extractor::SmartIpKeyExtractor,
};
use governor::middleware::NoOpMiddleware;
use axum::body::Body;

use crate::auth::extractors::UserIdExtractor;

// Rate limiting layer for general endpoints
pub fn public_rate_limit() -> GovernorLayer<
    SmartIpKeyExtractor,
    NoOpMiddleware,
    Body,
> {
    let cfg = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    GovernorLayer::new(cfg)
}

// Rate limiter specifically for auth endpoints
pub fn auth_rate_limit() -> GovernorLayer<
    SmartIpKeyExtractor,
    NoOpMiddleware,
    Body,
> {
    let cfg = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap();

    GovernorLayer::new(cfg)
}

pub fn user_rate_limit() -> GovernorLayer<
    UserIdExtractor,
    governor::middleware::NoOpMiddleware,
    axum::body::Body,
> {
    let cfg = GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(20)
        .key_extractor(UserIdExtractor)
        .finish()
        .unwrap();

    GovernorLayer::new(cfg)
}