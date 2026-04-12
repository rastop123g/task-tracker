use std::collections::HashSet;
use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const CHANNEL_CAPACITY: usize = 256;

#[derive(Debug, Clone)]
pub enum WsOutChannelMessage {
    Send(String),
    Pong,
}

#[derive(Debug)]
struct WsSession {
    user_id: Uuid,
    workspace_ids: HashSet<Uuid>,
    tx: Sender<WsOutChannelMessage>,
    // Для отключения ws сессии
    cancel: CancellationToken,
}

#[derive(Debug)]
pub struct WsSessionMap {
    sessions: DashMap<Uuid, WsSession>,
    user_sessions: DashMap<Uuid, HashSet<Uuid>>,
    workspace_sessions: DashMap<Uuid, HashSet<Uuid>>,
}

impl WsSessionMap {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            sessions: DashMap::new(),
            user_sessions: DashMap::new(),
            workspace_sessions: DashMap::new(),
        })
    }

    pub fn channel_capacity() -> usize {
        CHANNEL_CAPACITY
    }

    pub fn add_session(
        &self,
        user_id: Uuid,
        workspace_ids: Vec<Uuid>,
        tx: Sender<WsOutChannelMessage>,
        cancel: CancellationToken,
    ) -> Uuid {
        let session_id = Uuid::new_v4();
        let workspace_ids: HashSet<Uuid> = workspace_ids.into_iter().collect();
        let session = WsSession {
            user_id,
            workspace_ids: workspace_ids.clone(),
            tx,
            cancel,
        };

        self.user_sessions
            .entry(user_id)
            .or_default()
            .insert(session_id);

        for ws_id in &workspace_ids {
            self.workspace_sessions
                .entry(*ws_id)
                .or_default()
                .insert(session_id);
        }

        self.sessions.insert(session_id, session);
        session_id
    }

    pub fn remove_session(&self, session_id: &Uuid) -> Option<(Uuid, HashSet<Uuid>)> {
        let (_, session) = self.sessions.remove(session_id)?;

        if let Some(mut ids) = self.user_sessions.get_mut(&session.user_id) {
            ids.remove(session_id);
            if ids.is_empty() {
                drop(ids);
                self.user_sessions.remove(&session.user_id);
            }
        }

        for ws_id in &session.workspace_ids {
            if let Some(mut ids) = self.workspace_sessions.get_mut(ws_id) {
                ids.remove(session_id);
                if ids.is_empty() {
                    drop(ids);
                    self.workspace_sessions.remove(ws_id);
                }
            }
        }

        Some((session.user_id, session.workspace_ids))
    }

    pub fn add_user_to_workspace(&self, user_id: &Uuid, ws_id: &Uuid) {
        let target_session_ids: Vec<Uuid> = self
            .user_sessions
            .get(user_id)
            .map(|ids| ids.value().iter().copied().collect())
            .unwrap_or_default();

        for sid in target_session_ids {
            if let Some(mut session) = self.sessions.get_mut(&sid) {
                if session.workspace_ids.insert(*ws_id) {
                    self.workspace_sessions
                        .entry(*ws_id)
                        .or_default()
                        .insert(sid);
                }
            }
        }
    }

    pub fn remove_user_from_workspace(&self, user_id: &Uuid, ws_id: &Uuid) {
        let target_session_ids: Vec<Uuid> = self
            .user_sessions
            .get(user_id)
            .map(|ids| ids.value().iter().copied().collect())
            .unwrap_or_default();

        for sid in target_session_ids {
            let removed = self
                .sessions
                .get_mut(&sid)
                .is_some_and(|mut s| s.workspace_ids.remove(ws_id));

            if removed {
                if let Some(mut ids) = self.workspace_sessions.get_mut(ws_id) {
                    ids.remove(&sid);
                    if ids.is_empty() {
                        drop(ids);
                        self.workspace_sessions.remove(ws_id);
                    }
                }
            }
        }
    }

    pub fn send_to_user(&self, user_id: &Uuid, msg: &str) {
        let senders: Vec<(Sender<WsOutChannelMessage>, CancellationToken)> = self
            .user_sessions
            .get(user_id)
            .map(|ids| {
                ids.value()
                    .iter()
                    .filter_map(|sid| self.sessions.get(sid).map(|s| (s.tx.clone(), s.cancel.clone())))
                    .collect()
            })
            .unwrap_or_default();

        for (tx, cancel) in senders {
            match tx.try_send(WsOutChannelMessage::Send(msg.to_owned())) {
                Ok(_) => {}
                Err(e) => match e {
                    tokio::sync::mpsc::error::TrySendError::Full(_) => {
                        tracing::warn!("Session channel is full, dropping session");
                        cancel.cancel();
                    }
                    _ => {}
                },
            }
        }
    }

    pub fn send_to_workspace(&self, ws_id: &Uuid, msg: &str) {
        let senders: Vec<(Sender<WsOutChannelMessage>, CancellationToken)> = self
            .workspace_sessions
            .get(ws_id)
            .map(|ids| {
                ids.value()
                    .iter()
                    .filter_map(|sid| self.sessions.get(sid).map(|s| (s.tx.clone(), s.cancel.clone())))
                    .collect()
            })
            .unwrap_or_default();

        for (tx, cancel) in senders {
            match tx.try_send(WsOutChannelMessage::Send(msg.to_owned())) {
                Ok(_) => {}
                Err(e) => match e {
                    tokio::sync::mpsc::error::TrySendError::Full(_) => {
                        tracing::warn!("Session channel is full, dropping session");
                        cancel.cancel();
                    }
                    _ => {}
                },
            }
        }
    }

    pub fn has_user_sessions(&self, user_id: &Uuid) -> bool {
        self.user_sessions
            .get(user_id)
            .map(|ids| !ids.is_empty())
            .unwrap_or(false)
    }

    pub fn has_workspace_sessions(&self, ws_id: &Uuid) -> bool {
        self.workspace_sessions
            .get(ws_id)
            .map(|ids| !ids.is_empty())
            .unwrap_or(false)
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::sync::mpsc;
    use tokio::time::timeout;

    use super::*;

    fn msg() -> String {
        r#"{"type":"Pong","data":null}"#.to_owned()
    }

    fn make_channel() -> (Sender<WsOutChannelMessage>, mpsc::Receiver<WsOutChannelMessage>) {
        mpsc::channel(WsSessionMap::channel_capacity())
    }

    fn helper_add_session(map: &WsSessionMap, user_id: Uuid, ws_ids: Vec<Uuid>) -> Uuid {
        let (tx, _) = make_channel();
        let cancel = CancellationToken::new();
        map.add_session(user_id, ws_ids, tx, cancel)
    }

    #[tokio::test]
    async fn add_and_remove_session() {
        let map = WsSessionMap::new();
        let user_id = Uuid::new_v4();
        let ws_id = Uuid::new_v4();

        let sid = helper_add_session(&map, user_id, vec![ws_id]);

        assert!(map.has_user_sessions(&user_id));
        assert!(map.has_workspace_sessions(&ws_id));
        assert_eq!(map.session_count(), 1);

        let (returned_user, returned_ws) = map.remove_session(&sid).unwrap();
        assert_eq!(returned_user, user_id);
        assert!(returned_ws.contains(&ws_id));

        assert!(!map.has_user_sessions(&user_id));
        assert!(!map.has_workspace_sessions(&ws_id));
        assert_eq!(map.session_count(), 0);
    }

    #[tokio::test]
    async fn multiple_sessions_same_user() {
        let map = WsSessionMap::new();
        let user_id = Uuid::new_v4();
        let ws1 = Uuid::new_v4();
        let ws2 = Uuid::new_v4();

        let s1 = helper_add_session(&map, user_id, vec![ws1]);
        let s2 = helper_add_session(&map, user_id, vec![ws2]);

        assert_eq!(map.session_count(), 2);
        assert!(map.has_workspace_sessions(&ws1));
        assert!(map.has_workspace_sessions(&ws2));

        map.remove_session(&s1).unwrap();
        assert!(map.has_user_sessions(&user_id));
        assert!(!map.has_workspace_sessions(&ws1));
        assert!(map.has_workspace_sessions(&ws2));

        map.remove_session(&s2).unwrap();
        assert!(!map.has_user_sessions(&user_id));
        assert!(!map.has_workspace_sessions(&ws2));
    }

    #[tokio::test]
    async fn add_remove_user_from_workspace() {
        let map = WsSessionMap::new();
        let user_id = Uuid::new_v4();
        let ws1 = Uuid::new_v4();
        let ws2 = Uuid::new_v4();

        let s1 = helper_add_session(&map, user_id, vec![ws1]);

        map.add_user_to_workspace(&user_id, &ws2);
        assert!(map.has_workspace_sessions(&ws2));

        map.remove_user_from_workspace(&user_id, &ws2);
        assert!(!map.has_workspace_sessions(&ws2));
        assert!(map.has_workspace_sessions(&ws1));

        map.remove_session(&s1).unwrap();
        assert!(!map.has_workspace_sessions(&ws1));
    }

    #[tokio::test]
    async fn send_to_user_receives_message() {
        let map = WsSessionMap::new();
        let user_id = Uuid::new_v4();
        let ws_id = Uuid::new_v4();

        let (tx, mut rx) = make_channel();
        let cancel = CancellationToken::new();
        map.add_session(user_id, Vec::from([ws_id]), tx, cancel);

        map.send_to_user(&user_id, &msg());

        let received = timeout(Duration::from_millis(100), rx.recv()).await;
        assert!(received.is_ok());
    }

    #[tokio::test]
    async fn send_to_workspace_receives_message() {
        let map = WsSessionMap::new();
        let user_id = Uuid::new_v4();
        let ws_id = Uuid::new_v4();

        let (tx, mut rx) = make_channel();
        let cancel = CancellationToken::new();
        map.add_session(user_id, Vec::from([ws_id]), tx, cancel);

        map.send_to_workspace(&ws_id, &msg());

        let received = timeout(Duration::from_millis(100), rx.recv()).await;
        assert!(received.is_ok());
    }

    #[tokio::test]
    async fn send_to_user_not_on_channel() {
        let map = WsSessionMap::new();
        let other_user = Uuid::new_v4();
        let (tx, mut rx) = make_channel();
        let cancel = CancellationToken::new();
        map.add_session(other_user, Vec::new(), tx, cancel);

        map.send_to_user(&Uuid::new_v4(), &msg());

        let received = timeout(Duration::from_millis(50), rx.recv()).await;
        assert!(received.is_err());
    }

    #[tokio::test]
    async fn deadlock_send_vs_concurrent_remove() {
        let map = WsSessionMap::new();
        let user_id = Uuid::new_v4();
        let ws_id = Uuid::new_v4();

        let mut session_ids = Vec::new();
        for _ in 0..20 {
            let sid = helper_add_session(&map, user_id, vec![ws_id]);
            session_ids.push(sid);
        }

        let map_send = map.clone();
        let send_handle = tokio::spawn(async move {
            for _ in 0..100 {
                map_send.send_to_user(&user_id, &msg());
                map_send.send_to_workspace(&ws_id, &msg());
            }
        });

        let map_remove = map.clone();
        let remove_handle = tokio::spawn(async move {
            for sid in session_ids {
                map_remove.remove_session(&sid);
                tokio::task::yield_now().await;
            }
        });

        let result = timeout(
            Duration::from_secs(5),
            async { tokio::join!(send_handle, remove_handle) },
        )
        .await;
        assert!(result.is_ok(), "deadlock detected: timeout");
    }

    #[tokio::test]
    async fn deadlock_add_remove_workspace_vs_send() {
        let map = WsSessionMap::new();
        let user_id = Uuid::new_v4();
        let ws_id = Uuid::new_v4();

        let mut session_ids = Vec::new();
        for _ in 0..20 {
            let sid = helper_add_session(&map, user_id, vec![]);
            session_ids.push(sid);
        }

        let map_ws = map.clone();
        let ws_handle = tokio::spawn(async move {
            for _ in 0..200 {
                map_ws.add_user_to_workspace(&user_id, &ws_id);
                tokio::task::yield_now().await;
                map_ws.remove_user_from_workspace(&user_id, &ws_id);
                tokio::task::yield_now().await;
            }
        });

        let map_send = map.clone();
        let send_handle = tokio::spawn(async move {
            for _ in 0..200 {
                map_send.send_to_user(&user_id, &msg());
                map_send.send_to_workspace(&ws_id, &msg());
            }
        });

        let result = timeout(
            Duration::from_secs(5),
            async { tokio::join!(ws_handle, send_handle) },
        )
        .await;
        assert!(result.is_ok(), "deadlock detected: timeout");
    }

    #[tokio::test]
    async fn deadlock_concurrent_add_session_and_send() {
        let map = WsSessionMap::new();
        let user_id = Uuid::new_v4();
        let ws_id = Uuid::new_v4();

        let map_add = map.clone();
        let add_handle = tokio::spawn(async move {
            for _ in 0..100 {
                helper_add_session(&map_add, user_id, vec![ws_id]);
                tokio::task::yield_now().await;
            }
        });

        let map_send = map.clone();
        let send_handle = tokio::spawn(async move {
            for _ in 0..200 {
                map_send.send_to_user(&user_id, &msg());
            }
        });

        let result = timeout(
            Duration::from_secs(5),
            async { tokio::join!(add_handle, send_handle) },
        )
        .await;
        assert!(result.is_ok(), "deadlock detected: timeout");
    }
}
