# Browser Runtime Delivery Decision Proof

| Boundary | Route | Decision | Subscriber | Status |
| --- | --- | --- | --- | --- |
| browser runtime event chain | local-service | local-route-ready | browser-read-model | covered |
| browser action-intent status | local-in-process | local-route-ready | browser-action-intent-status | covered |
| browser action-intent handoff | local-in-process | local-route-ready | browser-action-intent-handoff | covered |
| browser runtime stream report | local-in-process | local-route-ready | browser-runtime-stream-report | covered |
| browser social-provider receipt status | local-in-process | local-route-ready | browser-social-provider-receipt-status | covered |
| browser social report-writer delivery status | local-in-process | local-route-ready | browser-social-report-writer-delivery-status | covered |
| browser social parent-notification delivery status | local-in-process | local-route-ready | browser-social-parent-notification-delivery-status | covered |
| browser social alert/report parent-surface status | local-in-process | local-route-ready | browser-social-alert-report-parent-surface-status | covered |
| browser external transport | external-transport | external-transport-route-manual-required | browser-intervention-command | manual-required |

The proof uses the reusable `ocentra-eventing` delivery decision API. External transport and relay delivery remain unimplemented, and the proof does not claim adapter dispatch, browser mutation, child intervention execution, final policy execution, or enforcement.
