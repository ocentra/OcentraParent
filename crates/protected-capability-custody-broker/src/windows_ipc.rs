mod io;
mod peer;
mod service;
mod service_accept;
mod service_control;

use crate::BrokerError;

pub(crate) fn run_service() -> Result<(), BrokerError> {
    service::run()
}
