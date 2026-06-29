use std::collections::{HashMap, HashSet};

use bytes::Bytes;
use uuid::Uuid;

use crate::{
    entity::user::UserEntity, protocol::websocket::{WsOutgoingMsg, ws_outgoing::user::{AddMemberEvent, RemoveMemberEvent, RevokeInviteEvent, UpdateUserEvent, UserInviteEvent}}, router::extractors::req_ctx::Ctx, websocket::{LocalFanOutMessage, global_fan_out::{GlobalFanOutMessage, send_to_user, send_to_workspace}}
};

pub async fn upd_user(ev: UpdateUserEvent, app: &Ctx) -> anyhow::Result<()> {
    let workspaces = app
        .ws_registry_service()
        .get_user_workspaces(&ev.user_id)
        .await?;
    let users = app
        .workspace_member_service()
        .get_user_ids_by_workspaces(&workspaces)
        .await?;
    let users_nodes = app.ws_registry_service().get_nodes_by_users(&users).await?;
    let mut inverted: HashMap<Uuid, HashSet<Uuid>> = Default::default();
    for (user_id, nodes) in users_nodes {
        for node_id in nodes {
            inverted.entry(node_id).or_default().insert(user_id);
        }
    }
    let str_message = serde_json::to_string(&WsOutgoingMsg::UpdateUser(ev.clone()))?;
    for (node_id, user_ids) in inverted {
        let msg =
            LocalFanOutMessage::UsersMessage(user_ids.into_iter().collect(), str_message.clone());
        let json = serde_json::to_string(&msg)?;
        let subject = format!("node.{node_id}.events");
        app.app.nats.client.publish(subject, json.into()).await?;
    }
    Ok(())
}

pub async fn add_member(ev: AddMemberEvent, app: &Ctx) -> anyhow::Result<()> {
    let user_nodes = app.ws_registry_service().get_user_nodes(&ev.user_id).await?;
    let workspace_nodes = app.ws_registry_service().get_workspace_nodes(&ev.workspace_id).await?;
    let mut all_nodes:HashSet<String> = Default::default();
    for node_id in user_nodes {
        all_nodes.insert(node_id);
    }
    for node_id in workspace_nodes {
        all_nodes.insert(node_id);
    }
    let local = serde_json::to_string(&LocalFanOutMessage::AddUserToWorkspace(ev))?;
    let payload = Bytes::from(local);
    for node_id in all_nodes {
        app.app.nats.client.publish(format!("node.{node_id}.events"), payload.clone()).await?;
    }
    Ok(())
}

pub async fn remove_member(ev: RemoveMemberEvent, app: &Ctx) -> anyhow::Result<()> {
    let user_nodes = app.ws_registry_service().get_user_nodes(&ev.user_id).await?;
    let workspace_nodes = app.ws_registry_service().get_workspace_nodes(&ev.workspace_id).await?;
    let mut all_nodes:HashSet<String> = Default::default();
    for node_id in user_nodes {
        all_nodes.insert(node_id);
    }
    for node_id in workspace_nodes {
        all_nodes.insert(node_id);
    }
    let local = serde_json::to_string(&LocalFanOutMessage::RemoveUserFromWorkspace(ev))?;
    let payload = Bytes::from(local);
    for node_id in all_nodes {
        app.app.nats.client.publish(format!("node.{node_id}.events"), payload.clone()).await?;
    }
    Ok(())
}

pub async fn invite(ev: UserInviteEvent, app: &Ctx) -> anyhow::Result<()> {
    let msg = serde_json::to_string(&WsOutgoingMsg::UserInvite(ev.clone()))?;
    send_to_workspace(&ev.workspace_id, &msg, app).await?;
    send_to_user(&ev.user_id, &msg, app).await?;
    Ok(())
}

pub async fn revoke_invite(ev: RevokeInviteEvent, app: &Ctx) -> anyhow::Result<()> {
    let msg = serde_json::to_string(&WsOutgoingMsg::RevokeInvite(ev.clone()))?;
    send_to_workspace(&ev.workspace_id, &msg, app).await?;
    send_to_user(&ev.user_id, &msg, app).await?;
    Ok(())
}

impl From<&UserEntity> for GlobalFanOutMessage {
    fn from(user: &UserEntity) -> Self {
        Self::UpdateUser(UpdateUserEvent {
            user_id: user.id.clone(),
            name: user.name.clone(),
        })
    }
}

impl From<&AddMemberEvent> for GlobalFanOutMessage {
    fn from(ev: &AddMemberEvent) -> Self {
        Self::AddMember(ev.clone())
    }
}

impl From<&RemoveMemberEvent> for GlobalFanOutMessage {
    fn from(ev: &RemoveMemberEvent) -> Self {
        Self::RemoveMember(ev.clone())
    }
}

impl From<&UserInviteEvent> for GlobalFanOutMessage {
    fn from(ev: &UserInviteEvent) -> Self {
        Self::UserInvite(ev.clone())
    }
}

impl From<&RevokeInviteEvent> for GlobalFanOutMessage {
    fn from(ev: &RevokeInviteEvent) -> Self {
        Self::RevokeInvite(ev.clone())
    }
}
