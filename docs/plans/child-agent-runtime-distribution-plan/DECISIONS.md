# Decisions

Status: locked architecture decisions for the child-agent distribution route.

| ID       | Decision                                                                               | Why it matters                                                               | Follow-up                                                |
| -------- | -------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- | -------------------------------------------------------- |
| CARD-001 | Child agent distribution is separate from parent client distribution.                  | Prevents parent packaging from absorbing child runtime claims.               | Keep the parent plan and child plan split.               |
| CARD-002 | Package proof is not setup proof.                                                      | Install state and package state are different claims.                        | Use setup-device-trust handoff for the boundary only.    |
| CARD-003 | Windows, macOS, and Linux rows may claim respawn only where the platform proves it.    | Stops generic "installed" language from becoming service persistence claims. | Keep respawn proof separate per platform.                |
| CARD-004 | Android rows must separate package, install, device-owner, and managed-profile claims. | Android support differs sharply by custody state.                            | Matrix rows must show the exact supported state.         |
| CARD-005 | iOS rows must stay honest about provisioning and service limits.                       | iOS distribution cannot inherit desktop-style service claims.                | Manual-required and capability-only states stay visible. |
| CARD-006 | Signing, store, and device-owner claims must be explicit per artifact.                 | Every platform has a different trust and distribution state.                 | Artifact matrix and signing matrix must stay visible.    |
| CARD-007 | Parent-authorized uninstall is a custody claim, not stealth persistence.               | Prevents anti-tamper language from becoming hidden malware semantics.        | Keep uninstall negative cases explicit.                  |
