use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    db::common::{DBColor, DBStatusCategory},
    error::ApiResult,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DBStatus {
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

impl DBNewWorkspaceStatus {
    pub async fn create_many(
        data: &Vec<DBNewWorkspaceStatus>,
        worspace_id: &Uuid,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<DBStatus>> {
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
        let res: Result<Vec<DBStatus>, sqlx::Error> =
            statuses.iter().map(FromRow::from_row).collect();
        Ok(res?)
    }
}
