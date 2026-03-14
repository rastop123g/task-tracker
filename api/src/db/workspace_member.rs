use crate::error::ApiResult;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DBWorkspaceMember {
    pub user_id: uuid::Uuid,
    pub workspace_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
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
