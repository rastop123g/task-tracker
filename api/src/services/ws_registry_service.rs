use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{
    db::workspace_member::DBWorkspaceMember,
    error::ApiResult,
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
        let serialized: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        let _: () = redis::pipe()
            .atomic()
            .del(&key)
            .sadd(&key, &serialized)
            .expire(&key, WORKSPACE_MEMBERS_TTL as i64)
            .query_async(&mut *conn)
            .await?;
        Ok(ids)
    }

    pub async fn invalidate_workspace_members(&self, ws_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn.del(workspace_members_key(ws_id)).await?;
        Ok(())
    }

    pub async fn add_workspace_member(&self, ws_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let key = workspace_members_key(ws_id);
        let _: () = conn.sadd(&key, user_id.to_string()).await?;
        let _: () = conn.expire(&key, WORKSPACE_MEMBERS_TTL as i64).await?;
        Ok(())
    }

    pub async fn remove_workspace_member(&self, ws_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let key = workspace_members_key(ws_id);
        let _: () = conn.srem(&key, user_id.to_string()).await?;
        Ok(())
    }

    pub async fn add_node_to_workspace(&self, ws_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn
            .sadd(workspace_nodes_key(ws_id), self.node_id())
            .await?;
        Ok(())
    }

    pub async fn remove_node_from_workspace(&self, ws_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn
            .srem(workspace_nodes_key(ws_id), self.node_id())
            .await?;
        Ok(())
    }

    pub async fn get_workspace_nodes(&self, ws_id: &Uuid) -> ApiResult<Vec<String>> {
        let mut conn = self.rconn().await?;
        let nodes: Vec<String> = conn.smembers(workspace_nodes_key(ws_id)).await?;
        Ok(nodes)
    }

    pub async fn add_node_to_user(&self, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn
            .sadd(user_nodes_key(user_id), self.node_id())
            .await?;
        Ok(())
    }

    pub async fn remove_node_from_user(&self, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.rconn().await?;
        let _: () = conn
            .srem(user_nodes_key(user_id), self.node_id())
            .await?;
        Ok(())
    }

    pub async fn get_user_nodes(&self, user_id: &Uuid) -> ApiResult<Vec<String>> {
        let mut conn = self.rconn().await?;
        let nodes: Vec<String> = conn.smembers(user_nodes_key(user_id)).await?;
        Ok(nodes)
    }
}
