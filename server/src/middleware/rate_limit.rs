use tower_governor::{
    GovernorLayer, 
    governor::GovernorConfigBuilder, 
    key_extractor::SmartIpKeyExtractor,
};
use governor::middleware::NoOpMiddleware;
use axum::body::Body;

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