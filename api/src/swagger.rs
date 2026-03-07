use serde::Serialize;
use utoipa::{
    Modify, OpenApi,
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

#[derive(Debug, Serialize)]
struct AuthScheme;

impl Modify for AuthScheme {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "api_key",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

use crate::router::ApiV1Docs;
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Task Tracker API",
        version = "1.0.0",
        description = "API for Task Tracker",
    ),
    modifiers(&AuthScheme),
    security(
        ("api_key" = [])
    ),
    nest((path = "/api/v1", api = ApiV1Docs)),
)]
pub struct ApiDoc;
