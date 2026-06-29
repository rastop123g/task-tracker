use std::collections::HashMap;

use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{
    db::workspace_member::DBWorkspaceMember,
    error::{ApiError, ApiResult},
    router::extractors::req_ctx::Ctx,
};

const WORKSPACE_MEMBERS_TTL: u64 = 60 * 60 * 24 * 3; // 3 days

fn workspace_members_key(ws_id: &Uuid) -> String {
    format!("ws:workspace:{ws_id}:members")
}

fn workspace_nodes_key(ws_id: &Uuid) -> String {
    format!("ws:workspace:{ws_id}:nodes")
}

fn user_nodes_key(user_id: &Uuid) -> String {
    format!("ws:user:{user_id}:nodes")
}

fn user_workspaces_key(user_id: &Uuid) -> String {
    format!("ws:user:{user_id}:workspaces")
}

#[derive(Debug, Clone)]
pub struct WsRegistryService {
    ctx: Ctx,
}

impl WsRegistryService {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    fn node_id(&self) -> &str {
        &self.ctx.app.config.ws_node_id
    }

    async fn rconn(&self) -> ApiResult<bb8::PooledConnection<'_, RedisConnectionManager>> {
        Ok(self.ctx.app.redis.get().await?)
    }

    pub async fn get_workspace_members(&self, ws_id: &Uuid) -> ApiResult<Vec<Uuid>> {
        let mut conn = self.rconn().await?;
        let key = workspace_members_key(ws_id);

        let cached: Option<Vec<String>> = conn.smembers(&key).await?;
        if let Some(cached) = cached {
            if !cached.is_empty() {
                let ids = cached
                    .iter()
                    .filter_map(|s| s.parse::<Uuid>().ok())
                    .collect();
                return Ok(ids);
            }
        }

        let mut db_conn = self.ctx.app.db.acquire().await?;
        let ids = DBWorkspaceMember::get_member_ids(ws_id, &mut db_conn).await?;
        if !ids.is_empty() {
            let serialized: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
            let _: () = redis::pipe()
                .atomic()
                .del(&key)
                .sadd(&key, &serialized)
                .expire(&key, WORKSPACE_MEMBERS_TTL as i64)
                .query_async(&mut *conn)
                .await?;
        }
        Ok(ids)
    }

    pub async fn add_workspace_member(&self, ws_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let ws_key = workspace_members_key(ws_id);
        let user_key = user_workspaces_key(user_id);
        let ws_exists: bool = conn.exists(&ws_key).await?;
        let user_exists: bool = conn.exists(&user_key).await?;

        let mut pipe = redis::Pipeline::new();
        pipe.atomic();
        if ws_exists {
            pipe.cmd("SADD")
                .arg(&ws_key)
                .arg(user_id.to_string())
                .ignore();
            pipe.expire(&ws_key, WORKSPACE_MEMBERS_TTL as i64);
        }
        if user_exists {
            pipe.cmd("SADD")
                .arg(&user_key)
                .arg(ws_id.to_string())
                .ignore();
            pipe.expire(&user_key, WORKSPACE_MEMBERS_TTL as i64);
        }
        if ws_exists || user_exists {
            let _: () = pipe.query_async(&mut *conn).await?;
        }
        Ok(())
    }

    pub async fn remove_workspace_member(&self, ws_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let ws_key = workspace_members_key(ws_id);
        let user_key = user_workspaces_key(user_id);

        let mut pipe = redis::Pipeline::new();
        pipe.atomic();
        pipe.srem(&ws_key, user_id.to_string());
        pipe.srem(&user_key, ws_id.to_string());
        let _: () = pipe.query_async(&mut *conn).await?;
        Ok(())
    }

    pub async fn add_node_to_workspace(&self, ws_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn
            .sadd(workspace_nodes_key(ws_id), self.node_id())
            .await?;
        Ok(())
    }

    pub async fn add_node_to_workspaces(&self, ws_ids: &[Uuid]) -> ApiResult<()> {
        if ws_ids.is_empty() {
            return Ok(());
        }
        let mut conn = self.rconn().await?;
        let mut pipe = redis::Pipeline::new();
        pipe.atomic();
        for ws_id in ws_ids {
            pipe.cmd("SADD")
                .arg(workspace_nodes_key(ws_id))
                .arg(self.node_id())
                .ignore();
        }
        let _: () = pipe.query_async(&mut *conn).await?;
        Ok(())
    }

    pub async fn remove_node_from_workspace(&self, ws_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn
            .srem(workspace_nodes_key(ws_id), self.node_id())
            .await?;
        Ok(())
    }

    // TODO: create task for checking node health and remove from registry if not alive
    pub async fn get_workspace_nodes(&self, ws_id: &Uuid) -> ApiResult<Vec<String>> {
        let mut conn = self.rconn().await?;
        let nodes: Vec<String> = conn.smembers(workspace_nodes_key(ws_id)).await?;
        Ok(nodes)
    }

    pub async fn add_node_to_user(&self, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn.sadd(user_nodes_key(user_id), self.node_id()).await?;
        Ok(())
    }

    pub async fn remove_node_from_user(&self, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn.srem(user_nodes_key(user_id), self.node_id()).await?;
        Ok(())
    }

    pub async fn remove_node_from_workspaces(&self, ws_ids: &[Uuid]) -> ApiResult<()> {
        if ws_ids.is_empty() {
            return Ok(());
        }
        let mut conn = self.rconn().await?;
        let mut pipe = redis::Pipeline::new();
        pipe.atomic();
        for ws_id in ws_ids {
            pipe.cmd("SREM")
                .arg(workspace_nodes_key(ws_id))
                .arg(self.node_id())
                .ignore();
        }
        let _: () = pipe.query_async(&mut *conn).await?;
        Ok(())
    }

    pub async fn get_user_nodes(&self, user_id: &Uuid) -> ApiResult<Vec<String>> {
        let mut conn = self.rconn().await?;
        let nodes: Vec<String> = conn.smembers(user_nodes_key(user_id)).await?;
        Ok(nodes)
    }

    pub async fn get_nodes_by_users(
        &self,
        user_ids: &[Uuid],
    ) -> ApiResult<HashMap<Uuid, Vec<Uuid>>> {
        let mut conn = self.rconn().await?;
        let mut pipe = redis::pipe();
        for user_id in user_ids {
            pipe.cmd("SMEMBERS").arg(user_nodes_key(user_id));
        }
        let nodes: Vec<Vec<String>> = pipe.query_async(&mut *conn).await?;
        let mut map: HashMap<Uuid, Vec<Uuid>> = HashMap::with_capacity(user_ids.len());
        for (user_id, nodes) in user_ids.into_iter().zip(nodes) {
            let user_nodes = nodes
                .into_iter()
                .map(|s| s.parse::<Uuid>().map_err(|_| ApiError::InternalServerError))
                .collect::<ApiResult<_>>()?;
            map.insert(*user_id, user_nodes);
        }
        Ok(map)
    }

    pub async fn get_user_workspaces(&self, user_id: &Uuid) -> ApiResult<Vec<Uuid>> {
        let mut conn = self.rconn().await?;
        let key = user_workspaces_key(user_id);

        let cached: Option<Vec<String>> = conn.smembers(&key).await?;
        if let Some(cached) = cached {
            if !cached.is_empty() {
                let ids = cached
                    .iter()
                    .filter_map(|s| s.parse::<Uuid>().ok())
                    .collect();
                return Ok(ids);
            }
        }

        let mut db_conn = self.ctx.app.db.acquire().await?;
        let ids = DBWorkspaceMember::get_workspace_ids(user_id, &mut db_conn).await?;
        if !ids.is_empty() {
            let serialized: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
            let _: () = redis::pipe()
                .atomic()
                .del(&key)
                .sadd(&key, &serialized)
                .expire(&key, WORKSPACE_MEMBERS_TTL as i64)
                .query_async(&mut *conn)
                .await?;
        }
        Ok(ids)
    }
}
