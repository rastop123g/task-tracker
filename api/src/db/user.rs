use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::ApiResult;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirmed: bool,
    pub avatar: Option<String>,
    pub avatar_preview: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub avatar_preview: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub avatar_preview: Option<String>,
}

impl NewUser {
    pub async fn create(&self, db: &mut sqlx::PgConnection) -> ApiResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO app_user (name, email, password, avatar, avatar_preview)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
            self.name,
            self.email,
            self.password,
            self.avatar,
            self.avatar_preview,
        )
        .fetch_one(db)
        .await?;
        Ok(user)
    }
}

impl UpdateUser {
    pub async fn update(&self, id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE app_user
            SET
                name = COALESCE($1, name),
                email = COALESCE($2, email),
                password = COALESCE($3, password),
                avatar = COALESCE($4, avatar),
                avatar_preview = COALESCE($5, avatar_preview)
            WHERE id = $6
            RETURNING *
        "#,
            self.name,
            self.email,
            self.password,
            self.avatar,
            self.avatar_preview,
            id,
        )
        .fetch_optional(db)
        .await?;
        Ok(user)
    }
}

impl User {
    pub async fn get(id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM app_user
            WHERE id = $1
        "#,
            id,
        )
        .fetch_optional(db)
        .await?;
        Ok(user)
    }

    pub async fn confirm_email(id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE app_user
            SET confirmed = true
            WHERE id = $1
            RETURNING *
        "#,
            id,
        )
        .fetch_optional(db)
        .await?;
        Ok(user)
    }

    pub async fn check_id(id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<bool> {
        let user = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM app_user WHERE id = $1)",
            id,
        )
        .fetch_optional(db)
        .await?;
        Ok(user.is_some())
    }

    pub async fn delete(id: &Uuid, db: &mut sqlx::PgConnection) -> ApiResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE app_user
            SET deleted_at = now()
            WHERE id = $1
            RETURNING *
        "#,
            id,
        )
        .fetch_optional(db)
        .await?;
        Ok(user)
    }

    pub async fn check_credentials(
        email: &str,
        password: &str,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Vec<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM app_user
            WHERE email = $1 AND password = $2
        "#,
            email,
            password,
        )
        .fetch_all(db)
        .await?;
        Ok(user)
    }

    pub async fn get_by_email(email: &str, db: &mut sqlx::PgConnection) -> ApiResult<Vec<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM app_user
            WHERE email = $1
        "#,
            email,
        )
        .fetch_all(db)
        .await?;
        Ok(user)
    }
}


