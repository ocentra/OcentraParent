use std::{future::pending, sync::Arc};

use futures::{stream::FuturesUnordered, StreamExt};
use tokio::sync::watch;

use crate::EventingError;

tokio::task_local! {
    static ACTIVE_HANDLER_SCOPE: Arc<HandlerScopeState>;
}

#[derive(Clone, Default)]
pub(super) struct HandlerScopeChain {
    scopes: Vec<Arc<HandlerScopeState>>,
}

impl HandlerScopeChain {
    pub(super) fn append(&self) -> HandlerScopeBinding {
        let state = Arc::new(HandlerScopeState::new());
        // CLONE-JUSTIFICATION: a child scope keeps the complete immutable
        // ancestor chain through nested handler publication.
        let mut scopes = self.scopes.clone();
        scopes.push(Arc::clone(&state));
        HandlerScopeBinding {
            chain: Self { scopes },
            guard: HandlerScopeGuard { state },
        }
    }

    pub(super) fn ensure_active(&self) -> Result<(), EventingError> {
        if self.scopes.iter().all(|scope| scope.is_active()) {
            return Ok(());
        }
        Err(EventingError::CausalDispatchCancelled)
    }

    pub(super) fn ensure_current_handler_task(&self) -> Result<(), EventingError> {
        let Some(expected) = self.scopes.last() else {
            return Err(EventingError::CausalPublicationOutsideHandlerTask);
        };
        let is_current = ACTIVE_HANDLER_SCOPE
            .try_with(|active| Arc::ptr_eq(active, expected))
            .unwrap_or(false);
        if is_current {
            return Ok(());
        }
        Err(EventingError::CausalPublicationOutsideHandlerTask)
    }

    pub(super) async fn cancelled(&self) {
        if self.scopes.is_empty() {
            return pending().await;
        }
        let mut notifications = FuturesUnordered::new();
        for scope in &self.scopes {
            let mut cancellation = scope.cancelled.subscribe();
            notifications.push(async move {
                let _ = cancellation.wait_for(|cancelled| *cancelled).await;
            });
        }
        let _ = notifications.next().await;
    }
}

pub(super) struct HandlerScopeBinding {
    pub(super) chain: HandlerScopeChain,
    pub(super) guard: HandlerScopeGuard,
}

pub(super) struct HandlerScopeGuard {
    state: Arc<HandlerScopeState>,
}

impl HandlerScopeGuard {
    pub(super) async fn run<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future,
    {
        // CLONE-JUSTIFICATION: task-local identity must match the exact private
        // handler scope while the handler future is being polled.
        ACTIVE_HANDLER_SCOPE
            .scope(Arc::clone(&self.state), future)
            .await
    }

    pub(super) fn cancel(&self) {
        self.state.cancel();
    }
}

impl Drop for HandlerScopeGuard {
    fn drop(&mut self) {
        self.state.cancel();
    }
}

struct HandlerScopeState {
    // BRAND-INVARIANT: cancellation is private and monotonic from false to true.
    cancelled: watch::Sender<bool>,
}

impl HandlerScopeState {
    fn new() -> Self {
        let (cancelled, _) = watch::channel(false);
        Self { cancelled }
    }

    fn is_active(&self) -> bool {
        !*self.cancelled.borrow()
    }

    fn cancel(&self) {
        self.cancelled.send_replace(true);
    }
}
