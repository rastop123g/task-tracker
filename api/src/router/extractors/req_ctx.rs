use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
};

use crate::{app_resources::AppResources, error::ApiError};

//TODO: Контекст вообще не нужен надо бы убрать
#[derive(Debug, Clone)]
pub struct Ctx {
    pub app: AppResources,
}

impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
    AppResources: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        //check cached
        if let Some(ctx) = parts.extensions.get::<Ctx>().cloned() {
            return Ok(ctx);
        }
        let State(app): State<AppResources> = State::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;

        let ctx = Ctx {
            app: app.clone(),
        };
        parts.extensions.insert(ctx.clone());
        Ok(ctx)
    }
}

impl Ctx {
    pub fn new(app: AppResources) -> Self {
        Self { app }
    }

    pub fn user_service(&self) -> crate::services::user_service::UserService {
        crate::services::user_service::UserService::new(self.clone())
    }

    pub fn workspace_service(&self) -> crate::services::workspace_service::WorkspaceService {
        crate::services::workspace_service::WorkspaceService::new(self.clone())
    }

    pub fn workspace_invite_service(
        &self,
    ) -> crate::services::workspace_invite_service::WorkspaceInviteService {
        crate::services::workspace_invite_service::WorkspaceInviteService::new(self.clone())
    }

    pub fn auth_service(&self) -> crate::services::auth_service::AuthService {
        crate::services::auth_service::AuthService::new(self.clone())
    }

    pub fn workspace_member_service(
        &self,
    ) -> crate::services::workspace_member_service::WorkspaceMemberService {
        crate::services::workspace_member_service::WorkspaceMemberService::new(self.clone())
    }

    pub fn workspace_statuses_service(
        &self,
    ) -> crate::services::workspace_statuses_service::WorkspaceStatusesService {
        crate::services::workspace_statuses_service::WorkspaceStatusesService::new(self.clone())
    }

    pub fn tag_service(&self) -> crate::services::tag_service::TagService {
        crate::services::tag_service::TagService::new(self.clone())
    }

    pub fn ws_registry_service(
        &self,
    ) -> crate::services::ws_registry_service::WsRegistryService {
        crate::services::ws_registry_service::WsRegistryService::new(self.clone())
    }
}
