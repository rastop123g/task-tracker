use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::ApiResult;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DBWorkspaceInvite {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl DBWorkspaceInvite {
    pub async fn create(
        workspace_id: &Uuid,
        user_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<DBWorkspaceInvite> {
        let workspace_invite = sqlx::query_as!(
            DBWorkspaceInvite,
            r#"
            INSERT INTO workspace_invitation (workspace_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (workspace_id, user_id) DO UPDATE SET
                deleted_at = NULL,
                updated_at = now()
            RETURNING *
        "#,
            workspace_id,
            user_id,
        )
        .fetch_one(db)
        .await?;
        Ok(workspace_invite)
    }

    pub async fn delete(
        workspace_id: &Uuid,
        user_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspaceInvite>> {
        let workspace_invite = sqlx::query_as!(
            DBWorkspaceInvite,
            r#"
            UPDATE workspace_invitation
            SET deleted_at = now()
            WHERE workspace_id = $1 AND user_id = $2
            RETURNING *
        "#,
            workspace_id,
            user_id,
        )
        .fetch_optional(db)
        .await?;
        Ok(workspace_invite)
    }

    pub async fn get(
        workspace_id: &Uuid,
        user_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspaceInvite>> {
        let workspace_invite = sqlx::query_as!(
            DBWorkspaceInvite,
            r#"
            SELECT *
            FROM workspace_invitation
            WHERE workspace_id = $1 AND user_id = $2
        "#,
            workspace_id,
            user_id,
        )
        .fetch_optional(db)
        .await?;
        Ok(workspace_invite)
    }

    pub async fn get_by_workspace_id(
        workspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBWorkspaceInvite>> {
        let workspace_invite = sqlx::query_as!(
            DBWorkspaceInvite,
            r#"
            SELECT *
            FROM workspace_invitation
            WHERE workspace_id = $1 AND deleted_at IS NULL
        "#,
            workspace_id,
        )
        .fetch_all(db)
        .await?;
        Ok(workspace_invite)
    }

    pub async fn get_by_user_id(
        user_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBWorkspaceInvite>> {
        let workspace_invite = sqlx::query_as!(
            DBWorkspaceInvite,
            r#"
            SELECT *
            FROM workspace_invitation
            WHERE user_id = $1 AND deleted_at IS NULL
        "#,
            user_id,
        )
        .fetch_all(db)
        .await?;
        Ok(workspace_invite)
    }
}
