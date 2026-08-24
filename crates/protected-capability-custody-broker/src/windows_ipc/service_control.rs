use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use windows_service::service::ServiceControl;
use windows_service::service_control_handler::{
    self, ServiceControlHandlerResult, ServiceStatusHandle,
};

use crate::BrokerError;

pub(super) fn register(stopping: &Arc<AtomicBool>) -> Result<ServiceStatusHandle, BrokerError> {
    let handler_stopping = Arc::clone(stopping);
    service_control_handler::register(crate::BROKER_SERVICE_NAME, move |control_event| {
        handle_control(control_event, &handler_stopping)
    })
    .map_err(map_service_error)
}

fn handle_control(
    control_event: ServiceControl,
    stopping: &AtomicBool,
) -> ServiceControlHandlerResult {
    match control_event {
        ServiceControl::Stop | ServiceControl::Shutdown => {
            stopping.store(true, Ordering::Release);
            ServiceControlHandlerResult::NoError
        }
        ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
        _ => ServiceControlHandlerResult::NotImplemented,
    }
}

fn map_service_error<E>(_error: E) -> BrokerError {
    BrokerError::Transport
}
