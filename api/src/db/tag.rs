use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{db::common::DBColor, error::ApiResult};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DBTag {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub color: DBColor,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct DBNewWorkspaceTag {
    pub name: String,
    pub color: DBColor,
}

#[derive(Debug, Clone)]
pub struct DBUpdateWorkspaceTag {
    pub name: Option<String>,
    pub color: Option<DBColor>,
}

impl DBNewWorkspaceTag {
    pub async fn create_many(
        data: &Vec<DBNewWorkspaceTag>,
        worspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBTag>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }
        let mut qb =
            sqlx::QueryBuilder::new("INSERT INTO workspace_tag (workspace_id, name, color)");
        qb.push_values(data, |mut b, tag| {
            b.push_bind(worspace_id);
            b.push_bind(&tag.name);
            b.push_bind(tag.color);
        });
        qb.push(
            r#"
            RETURNING
                id,
                workspace_id,
                name,
                color,
                created_at,
                updated_at,
                deleted_at
        "#,
        );
        let tags_res = qb.build().fetch_all(db).await;
        tracing::debug!("DBNewWorkspaceTag::create_many {:?}", tags_res);
        let tags = tags_res?;
        let res: Result<Vec<DBTag>, sqlx::Error> = tags.iter().map(FromRow::from_row).collect();
        tracing::debug!("DBNewWorkspaceTag::create_many {:?}", res);
        Ok(res?)
    }

    pub async fn create(
        &self,
        worspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<DBTag> {
        let res = sqlx::query_as!(
            DBTag,
            r#"
            INSERT INTO workspace_tag (workspace_id, name, color)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                workspace_id,
                name,
                color as "color: DBColor",
                created_at,
                updated_at,
                deleted_at
        "#,
            worspace_id,
            self.name,
            self.color as DBColor,
        )
        .fetch_one(db)
        .await;
        Ok(res?)
    }
}

impl DBUpdateWorkspaceTag {
    pub async fn update(
        &self,
        tag_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<DBTag>> {
        let res = sqlx::query_as!(
            DBTag,
            r#"
            UPDATE workspace_tag
            SET name = COALESCE($2, name), color = COALESCE($3, color), updated_at = NOW()
            WHERE id = $1
            RETURNING
                id,
                workspace_id,
                name,
                color as "color: DBColor",
                created_at,
                updated_at,
                deleted_at
        "#,
            tag_id,
            self.name,
            self.color as Option<DBColor>,
        )
        .fetch_optional(db)
        .await;
        Ok(res?)
    }
}

impl DBTag {
    pub async fn get_by_workspace_id(
        workspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBTag>> {
        let res = sqlx::query_as!(
            DBTag,
            r#"
                SELECT
                    id,
                    workspace_id,
                    name,
                    color as "color: DBColor",
                    created_at,
                    updated_at,
                    deleted_at
                FROM workspace_tag
                WHERE workspace_id = $1
                ORDER BY name ASC
            "#,
            workspace_id,
        )
        .fetch_all(db)
        .await;
        Ok(res?)
    }

    pub async fn get_by_id(id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<Option<DBTag>> {
        let res = sqlx::query_as!(
            DBTag,
            r#"
                SELECT
                    id,
                    workspace_id,
                    name,
                    color as "color: DBColor",
                    created_at,
                    updated_at,
                    deleted_at
                FROM workspace_tag
                WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(db)
        .await;
        Ok(res?)
    }

    pub async fn delete(id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<Option<DBTag>> {
        let res = sqlx::query_as!(
            DBTag,
            r#"
                UPDATE workspace_tag
                SET deleted_at = now()
                WHERE id = $1
                RETURNING
                    id,
                    workspace_id,
                    name,
                    color as "color: DBColor",
                    created_at,
                    updated_at,
                    deleted_at
            "#,
            id,
        )
        .fetch_optional(db)
        .await;
        Ok(res?)
    }
}
