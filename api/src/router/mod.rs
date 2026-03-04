use axum::Router;
use utoipa::OpenApi;

use crate::app_resources::AppResources;
pub mod auth;
pub mod avatar;

pub fn app_router(res: AppResources) -> Router<AppResources> {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .nest("/api/v1", apiv1_router(res))
}

pub fn apiv1_router(res: AppResources) -> Router<AppResources> {
    Router::new()
        .nest("/auth", auth::auth_router())
        .nest("/avatar", avatar::avatar_router(res))
}


use self::avatar::AvatarApiDoc;
use self::auth::AuthApiDoc;
#[derive(OpenApi)]
#[openapi(
    nest(
         (path = "/avatar", api = AvatarApiDoc),
         (path = "/auth", api = AuthApiDoc),
    ),
)]
pub struct ApiV1Docs;
