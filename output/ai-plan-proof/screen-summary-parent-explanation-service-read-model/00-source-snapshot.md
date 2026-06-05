# Screen Summary Parent Explanation Service Read Model Proof

- Status: ok
- Proof kind: screen-summary-parent-explanation-service-read-model
- Service event: agent.activity.screen.read-model.reported
- Activity read-model kind: screen
- Activity surface state: ready
- Row id: screen-summary-parent-explanation-service-row
- Policy decision ref: screen-summary-parent-explanation-service-policy-decision
- Parent rule refs: screen-summary-parent-explanation-service-parent-rule
- Parent explanation refs: screen-summary-parent-explanation-service-explanation
- Local model runtime refs: screen-summary-parent-explanation-service-local-runtime
- Image deletion state: deleted
- Custody state: child-device-journal

Non-claims:
- This proof starts the real Rust service and requests the Activity Screen read model over WebSocket.
- It proves query-store/service read-model custody for parent explanation refs, not production portal rendering.
- It does not create new captures, rerun model inference, upload raw screenshots, or claim remote/API AI.
