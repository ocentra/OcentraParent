mod io;
#[path = "windows_ipc/peer.rs"]
mod peer;
#[path = "windows_ipc/service.rs"]
mod service;
mod service_accept;
mod service_control;
mod service_status;

use crate::BrokerError;

pub(crate) fn run_service() -> Result<(), BrokerError> {
    service::run()
}
