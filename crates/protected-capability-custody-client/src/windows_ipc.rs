use crate::admission::AuthenticatedBrokerSession;
use crate::ClientError;

#[path = "windows_ipc_connect.rs"]
pub(crate) mod connect_impl;
#[path = "windows_ipc_io.rs"]
pub(crate) mod io;
#[path = "windows_ipc_io_read.rs"]
pub(crate) mod io_read;
#[path = "windows_ipc_io_write.rs"]
pub(crate) mod io_write;
#[path = "windows_ipc_peer.rs"]
pub(crate) mod peer;
#[path = "windows_ipc_session.rs"]
pub(crate) mod session;

pub(crate) fn connect() -> Result<AuthenticatedBrokerSession, ClientError> {
    connect_impl::connect()
}
