# Tracking Full Product UI Runtime Preflight Manual Runbook

- generatedAt: 2026-06-08T12:30:00.000Z
- status: manual_required
- This runbook is not product-ready proof. It names the product UI runtime artifacts still missing.

## retention-settings-production-write-result

Acceptance criteria:
- Parent retention settings write is executed through the production runtime path.
- The parent product UI renders the completed write result with source evidence and service status.
- The artifact includes the command result, persisted setting revision, screenshot, and no-product-ready claim boundary.

Manual commands:
- cmd /c npm run dev:agent
- cmd /c npm run dev:portal
- manual: capture production retention settings product UI write result artifact under product-parent-child-ui-runtime

Required artifacts:
- output/tracking-plan-proof/product-parent-child-ui-runtime/04-retention-settings-production-write-result.png

## rendered-child-device-check-in

Acceptance criteria:
- Child runtime receives and renders the check-in request on the actual child surface.
- The child check-in response is captured with parent receipt and runtime observation refs.
- The artifact includes child UI screenshot, delivery envelope, response payload, and device/runtime log reference.

Manual commands:
- manual: run child-device check-in request against rendered child runtime UI
- manual: capture child UI screenshot and parent receipt under product-parent-child-ui-runtime

Required artifacts:
- output/tracking-plan-proof/product-parent-child-ui-runtime/05-child-device-rendered-check-in-runtime.png

## rendered-child-device-location-consent

Acceptance criteria:
- Child runtime renders the location consent state through the actual child surface.
- Consent decision evidence is tied to parent-visible tracking status and source refs.
- The artifact includes child UI screenshot, consent payload, parent receipt, and runtime log reference.

Manual commands:
- manual: run child location consent flow on rendered child runtime UI
- manual: capture consent screenshot, payload, parent receipt, and logs under product-parent-child-ui-runtime

Required artifacts:
- output/tracking-plan-proof/product-parent-child-ui-runtime/06-child-device-rendered-location-consent-runtime.png

## child-device-safe-help-response

Acceptance criteria:
- Child runtime renders safe/help response UI and sends a response through the real runtime path.
- Parent product UI receives and displays the safe/help response with source evidence refs.
- The artifact includes child UI screenshot, response payload, parent receipt, and runtime log reference.

Manual commands:
- manual: run child safe/help response flow on rendered child runtime UI
- manual: capture child response screenshot, parent receipt, and logs under product-parent-child-ui-runtime

Required artifacts:
- output/tracking-plan-proof/product-parent-child-ui-runtime/07-child-device-safe-help-response-runtime.png

