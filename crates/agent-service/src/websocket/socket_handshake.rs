#![forbid(unsafe_code)]

//! Bounded first-message authentication for the parent-local socket.

use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use ocentra_family_identity_core::session_lifecycle_custody::authenticated_parent_local_bridge::AuthenticatedParentLocalBridgeSession;
use ocentra_schema::account_identity_parent_local_bridge::AccountIdentityParentLocalBridgeHandshake;

use crate::parent_local_bridge_admission::ParentLocalBridgeAdmission;

const TIMEOUT: Duration = Duration::from_secs(5);
const MAX_TEXT_BYTES: usize = 512;

pub(super) async fn authenticate_connection(
    socket: &mut WebSocket,
    admission: &ParentLocalBridgeAdmission,
) -> Option<AuthenticatedParentLocalBridgeSession> {
    let message = tokio::time::timeout(TIMEOUT, socket.recv())
        .await
        .ok()??
        .ok()?;
    let Message::Text(text) = message else {
        return None;
    };
    if text.len() > MAX_TEXT_BYTES {
        return None;
    }
    let handshake =
        serde_json::from_str::<AccountIdentityParentLocalBridgeHandshake>(&text).ok()?;
    admission.authenticate(&handshake).ok()
}
