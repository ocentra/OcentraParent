# Decisions

Status: locked architecture decisions for the parent client distribution route.

| ID       | Decision                                                                                                | Why it matters                                                          | Follow-up                                                 |
| -------- | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------- |
| PCRD-001 | Parent client distribution is parent-only: web portal, desktop, Android, and iOS.                       | Keeps parent and child runtime distribution separate.                   | Child agent distribution gets its own plan.               |
| PCRD-002 | Setup handoff is not package proof.                                                                     | Install state and artifact proof are different claims.                  | Use the setup handoff contract for boundary mapping only. |
| PCRD-003 | Parent web portal is a parent client and must have build, route, auth, cache, and env separation proof. | Hosted parent portal is a distribution target, not just a page.         | Proof must include route smoke and no-child-data checks.  |
| PCRD-004 | Parent desktop shell/package is not product readiness.                                                  | Launch smoke does not prove policy, child, billing, or setup readiness. | Desktop proof stays scoped to the shell/package boundary. |
| PCRD-005 | Parent Android and iOS scaffold states are manual-required until real device/build/store proof exists.  | Stops scaffold from being mistaken for parity.                          | Proof must show device or store state explicitly.         |
| PCRD-006 | Signing, notarization, and store claims must be explicit per artifact.                                  | Every platform has a different trust and distribution state.            | Artifact matrix and signing matrix must stay visible.     |
| PCRD-007 | Route bridge behavior must stay separate from setup and package claims.                                 | Local-service and LAN bridge behavior is not a packaging claim.         | Keep bridge proof separate from setup proof.              |
| PCRD-008 | Child-agent runtime/package distribution belongs to a separate plan.                                    | Prevents child claims from leaking into parent client distribution.     | Route child packaging work to the child plan.             |
