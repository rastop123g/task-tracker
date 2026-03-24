use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    db::common::{DBColor, DBStatusCategory},
    error::ApiResult,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DBWorkspaceStatus {
    pub id: Uuid,
    pub name: String,
    pub category: DBStatusCategory,
    pub workspace_id: Uuid,
    pub color: DBColor,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct DBNewWorkspaceStatus {
    pub name: String,
    pub category: DBStatusCategory,
    pub color: DBColor,
}

#[derive(Debug, Clone)]
pub struct DBUpdateWorkspaceStatus {
    pub name: Option<String>,
    pub category: Option<DBStatusCategory>,
    pub color: Option<DBColor>,
}

impl DBNewWorkspaceStatus {
    pub async fn create_many(
        data: &Vec<DBNewWorkspaceStatus>,
        worspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBWorkspaceStatus>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        let mut qb = sqlx::QueryBuilder::new(
            "INSERT INTO task_status (workspace_id, name, category, color)",
        );
        qb.push_values(data, |mut b, status| {
            b.push_bind(worspace_id);
            b.push_bind(&status.name);
            b.push_bind(status.category);
            b.push_bind(status.color);
        });
        qb.push(
            r#"
            RETURNING
                id,
                name,
                category,
                workspace_id,
                color,
                created_at,
                updated_at,
                deleted_at
        "#,
        );
        let statuses = qb.build().fetch_all(db).await?;
        let res: Result<Vec<DBWorkspaceStatus>, sqlx::Error> =
            statuses.iter().map(FromRow::from_row).collect();
        Ok(res?)
    }

    pub async fn create(
        &self,
        workspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<DBWorkspaceStatus> {
        let res = sqlx::query_as!(
            DBWorkspaceStatus,
            r#"
                INSERT INTO task_status (workspace_id, name, category, color)
                VALUES ($1, $2, $3, $4)
                RETURNING
                    id,
                    name,
                    category as "category: DBStatusCategory",
                    workspace_id,
                    color as "color: DBColor",
                    created_at,
                    updated_at,
                    deleted_at
            "#,
            workspace_id,
            self.name,
            self.category as DBStatusCategory,
            self.color as DBColor,
        )
        .fetch_one(db)
        .await?;
        Ok(res)
    }
}

impl DBWorkspaceStatus {
    pub async fn get_by_workspace_id(
        workspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBWorkspaceStatus>> {
        let res = sqlx::query_as!(
            DBWorkspaceStatus,
            r#"
                SELECT
                    id,
                    name,
                    category as "category: DBStatusCategory",
                    workspace_id,
                    color as "color: DBColor",
                    created_at,
                    updated_at,
                    deleted_at
                FROM task_status
                WHERE workspace_id = $1
            "#,
            workspace_id
        )
        .fetch_all(db)
        .await?;
        Ok(res)
    }

    pub async fn get_by_id(
        id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspaceStatus>> {
        let status = sqlx::query_as!(
            DBWorkspaceStatus,
            r#"
                SELECT
                    id,
                    name,
                    category as "category: DBStatusCategory",
                    workspace_id,
                    color as "color: DBColor",
                    created_at,
                    updated_at,
                    deleted_at
                FROM task_status
                WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(db)
        .await?;
        Ok(status)
    }

    pub async fn delete(
        id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspaceStatus>> {
        let status = sqlx::query_as!(
            DBWorkspaceStatus,
            r#"
                UPDATE task_status
                SET deleted_at = now()
                WHERE id = $1
                RETURNING 
                    id,
                    name,
                    category as "category: DBStatusCategory",
                    workspace_id,
                    color as "color: DBColor",
                    created_at,
                    updated_at,
                    deleted_at
            "#,
            id,
        )
        .fetch_optional(db)
        .await?;
        Ok(status)
    }
}

impl DBUpdateWorkspaceStatus {
    pub async fn update(
        &self,
        id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBWorkspaceStatus>> {
        let status = sqlx::query_as!(
            DBWorkspaceStatus,
            r#"
                UPDATE task_status
                SET
                    name = COALESCE($1, name),
                    category = COALESCE($2, category),
                    color = COALESCE($3, color),
                    updated_at = now()
                WHERE id = $4
                RETURNING 
                    id,
                    name,
                    category as "category: DBStatusCategory",
                    workspace_id,
                    color as "color: DBColor",
                    created_at,
                    updated_at,
                    deleted_at
            "#,
            self.name,
            self.category as Option<DBStatusCategory>,
            self.color as Option<DBColor>,
            id,
        )
        .fetch_optional(db)
        .await?;
        Ok(status)
    }
}
