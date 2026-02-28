use axum::Router;

use crate::app_resources::AppResources;

pub fn app_router(res: AppResources) -> Router {
    Router::new().with_state(res).route("/health", axum::routing::get(|| async { "OK" }))
}
