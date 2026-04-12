use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::ApiResult;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DBWorkspaceMember {
    pub user_id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DBWorkspaceMemberWithUser {
    pub user_id: Uuid,
    pub workspace_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub user_name: String,
    pub user_avatar: Option<String>,
    pub user_avatar_preview: Option<String>,
    pub user_email: String,
    pub user_confirmed: bool,
    pub user_created_at: DateTime<Utc>,
    pub user_updated_at: DateTime<Utc>,
    pub user_deleted_at: Option<DateTime<Utc>>,
}

impl DBWorkspaceMember {
    pub async fn create(
        user_id: &uuid::Uuid,
        workspace_id: &uuid::Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<DBWorkspaceMember> {
        let res = sqlx::query_as!(
            DBWorkspaceMember,
            r#"
                INSERT INTO workspace_member (user_id, workspace_id)
                VALUES ($1, $2)
                ON CONFLICT (user_id, workspace_id) DO UPDATE SET
                    deleted_at = NULL,
                    updated_at = now()
                RETURNING *
            "#,
            user_id,
            workspace_id
        )
        .fetch_one(db)
        .await?;
        Ok(res)
    }

    pub async fn delete(
        user_id: &uuid::Uuid,
        workspace_id: &uuid::Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspaceMember>> {
        let res = sqlx::query_as!(
            DBWorkspaceMember,
            r#"
                UPDATE workspace_member
                SET deleted_at = now()
                WHERE user_id = $1 AND workspace_id = $2
                RETURNING *
            "#,
            user_id,
            workspace_id
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }

    pub async fn get(
        user_id: &uuid::Uuid,
        workspace_id: &uuid::Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspaceMember>> {
        let res = sqlx::query_as!(
            DBWorkspaceMember,
            r#"
                SELECT *
                FROM workspace_member
                WHERE user_id = $1 AND workspace_id = $2
            "#,
            user_id,
            workspace_id
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }

    pub async fn get_by_user_id(
        user_id: &uuid::Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBWorkspaceMember>> {
        let res = sqlx::query_as!(
            DBWorkspaceMember,
            r#"
                SELECT *
                FROM workspace_member
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(db)
        .await?;
        Ok(res)
    }

    pub async fn get_member_ids(
        workspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<Uuid>> {
        let res = sqlx::query_scalar!(
            r#"
                SELECT user_id
                FROM workspace_member
                WHERE workspace_id = $1 AND deleted_at IS NULL
            "#,
            workspace_id
        )
        .fetch_all(db)
        .await?;
        Ok(res)
    }

    pub async fn get_workspace_ids(
        user_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<Uuid>> {
        let res = sqlx::query_scalar!(
            r#"
                SELECT workspace_id
                FROM workspace_member
                WHERE user_id = $1 AND deleted_at IS NULL
            "#,
            user_id
        )
        .fetch_all(db)
        .await?;
        Ok(res)
    }

    pub async fn get_by_workspace_id(
        workspace_id: &uuid::Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBWorkspaceMember>> {
        let res = sqlx::query_as!(
            DBWorkspaceMember,
            r#"
                SELECT *
                FROM workspace_member
                WHERE workspace_id = $1
            "#,
            workspace_id
        )
        .fetch_all(db)
        .await?;
        Ok(res)
    }
}

impl DBWorkspaceMemberWithUser {
    pub async fn get_list(
        workspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBWorkspaceMemberWithUser>> {
        let res = sqlx::query_as!(
            DBWorkspaceMemberWithUser,
            r#"
                SELECT 
                    wm.*,
                    u.name as user_name,
                    u.avatar as user_avatar,
                    u.avatar_preview as user_avatar_preview,
                    u.email as user_email,
                    u.confirmed as user_confirmed,
                    u.created_at as user_created_at,
                    u.updated_at as user_updated_at,
                    u.deleted_at as user_deleted_at
                FROM workspace_member wm
                JOIN app_user u ON u.id = wm.user_id
                WHERE wm.workspace_id = $1
            "#,
            workspace_id
        )
        .fetch_all(db)
        .await?;
        Ok(res)
    }

    pub async fn get(user_id: &Uuid, workspace_id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<Option<DBWorkspaceMemberWithUser>> {
        let res = sqlx::query_as!(
            DBWorkspaceMemberWithUser,
            r#"
                SELECT 
                    wm.*,
                    u.name as user_name,
                    u.avatar as user_avatar,
                    u.avatar_preview as user_avatar_preview,
                    u.email as user_email,
                    u.confirmed as user_confirmed,
                    u.created_at as user_created_at,
                    u.updated_at as user_updated_at,
                    u.deleted_at as user_deleted_at
                FROM workspace_member wm
                JOIN app_user u ON u.id = wm.user_id
                WHERE wm.workspace_id = $1 AND wm.user_id = $2
            "#,
            workspace_id,
            user_id
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
}
