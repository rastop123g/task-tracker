use axum::Router;
use utoipa::OpenApi;

use crate::app_resources::AppResources;
pub mod extractors;
pub mod auth;
pub mod avatar;
pub mod user;

pub fn app_router(res: AppResources) -> Router<AppResources> {
    Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .nest("/api/v1", apiv1_router(res))
}

pub fn apiv1_router(res: AppResources) -> Router<AppResources> {
    Router::new()
        .nest("/auth", auth::auth_router())
        .nest("/avatar", avatar::avatar_router(res))
        .nest("/user", user::user_router())
}

use self::auth::AuthApiDoc;
use self::avatar::AvatarApiDoc;
use self::user::UserApiDoc;
#[derive(OpenApi)]
#[openapi(
    nest(
         (path = "/avatar", api = AvatarApiDoc),
         (path = "/auth", api = AuthApiDoc),
         (path = "/user", api = UserApiDoc),
    ),
)]
pub struct ApiV1Docs;
