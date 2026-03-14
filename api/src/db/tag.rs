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
}
