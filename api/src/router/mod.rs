use axum::Router;

use crate::app_resources::AppResources;
pub mod auth;

pub fn app_router(res: AppResources) -> Router<AppResources> {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .nest("/api/v1", apiv1_router(res))
}

pub fn apiv1_router(res: AppResources) -> Router<AppResources> {
    Router::new().nest("/auth", auth::auth_router())
}
