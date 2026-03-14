use uuid::Uuid;

use crate::error::ApiResult;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DBWorkspace {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub admin_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct DBNewWorkspace {
    pub name: String,
    pub admin_id: Uuid,
}

#[derive(Debug, Clone, Default)]
pub struct DBUpdateWorkspace {
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub admin_id: Option<Uuid>,
}

impl DBWorkspace {
    pub async fn get(id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<Option<Self>> {
        let res = sqlx::query_as!(
            DBWorkspace,
            r#"
                SELECT *
                FROM app_workspace
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }

    pub async fn delete(id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<bool> {
        let res = sqlx::query!(
            r#"
                UPDATE app_workspace
                SET deleted_at = now()
                WHERE id = $1
            "#,
            id
        )
        .execute(db)
        .await?;
        Ok(res.rows_affected() == 1)
    }

    pub async fn reset_avatar(
        &self,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspace>> {
        let res = sqlx::query_as!(
            DBWorkspace,
            r#"
                UPDATE app_workspace
                SET avatar = NULL
                WHERE id = $1
                RETURNING *
            "#,
            self.id
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }

    pub async fn update_avatar(
        &self,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspace>> {
        let res = sqlx::query_as!(
            DBWorkspace,
            r#"
                UPDATE app_workspace
                SET avatar = $1
                WHERE id = $2
                RETURNING *
            "#,
            self.avatar,
            self.id
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
}

impl DBNewWorkspace {
    pub async fn create(&self, db: &mut sqlx::PgConnection) -> ApiResult<DBWorkspace> {
        let res = sqlx::query_as!(
            DBWorkspace,
            r#"
                INSERT INTO app_workspace (name, admin_id)
                VALUES ($1, $2)
                RETURNING *
            "#,
            self.name,
            self.admin_id
        )
        .fetch_one(db)
        .await?;
        Ok(res)
    }
}

impl DBUpdateWorkspace {
    pub async fn update(
        &self,
        id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspace>> {
        let res = sqlx::query_as!(
            DBWorkspace,
            r#"
                UPDATE app_workspace
                SET name = COALESCE($2, name),
                    avatar = COALESCE($3, avatar),
                    admin_id = COALESCE($4, admin_id)
                WHERE id = $1
                RETURNING *
            "#,
            id,
            self.name,
            self.avatar,
            self.admin_id
        )
        .fetch_optional(db)
        .await?;
        Ok(res)
    }
}
