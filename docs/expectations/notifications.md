# Notification Feature Expectations

Notification features should reduce parent anxiety, not create noise.

## Expected Deliverables

- Notification contract.
- Alert reason codes.
- Provider adapter boundary.
- Delivery status.
- Retry/failure handling.
- Quiet hours.
- Parent preference controls.

## Acceptance

- Notifications reference evidence and policy reason.
- Provider failure is visible and retryable.
- Parents can tune frequency.
- Sensitive details are minimized in push, WhatsApp, email, or SMS bodies.
- Notification history is auditable.

## Non-Goals

- Do not send alerts from raw unclassified noise.
- Do not hardcode one provider into core policy logic.
- Do not expose sensitive evidence in third-party notification previews unless explicitly approved.

## Done Signal

The system can create, deliver, and audit one notification type through a provider boundary with clear parent controls and safe failure behavior.
