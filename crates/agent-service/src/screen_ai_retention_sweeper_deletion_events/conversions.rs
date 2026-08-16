use crate::screen_ai_service_event_subscription::ObservedAtText;

use super::ScreenRetentionObservedAt;

impl From<ObservedAtText> for ScreenRetentionObservedAt {
    fn from(value: ObservedAtText) -> Self {
        Self(value.0)
    }
}
