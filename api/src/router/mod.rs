use axum::Router;
use utoipa::OpenApi;

use crate::app_resources::AppResources;
pub mod auth;
pub mod avatar;
pub mod extractors;
pub mod path_params;
pub mod user;
pub mod workspace;

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
        .nest("/workspace", workspace::workspace_router())
}

use self::auth::AuthApiDoc;
use self::avatar::AvatarApiDoc;
use self::user::UserApiDoc;
use self::workspace::WorkspaceApiDoc;
#[derive(OpenApi)]
#[openapi(
    nest(
         (path = "/avatar", api = AvatarApiDoc),
         (path = "/auth", api = AuthApiDoc),
         (path = "/user", api = UserApiDoc),
         (path = "/workspace", api = WorkspaceApiDoc),
    ),
)]
pub struct ApiV1Docs;
