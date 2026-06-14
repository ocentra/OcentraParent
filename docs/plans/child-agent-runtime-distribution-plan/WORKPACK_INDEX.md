# Workpack Index

| Workpack                                                                                              | Purpose                                                              | Status  |
| ----------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------- |
| [01-child-agent-scope-and-route-boundary](workpacks/01-child-agent-scope-and-route-boundary.md)       | Canonical scope correction and setup-device-trust handoff contract.  | Planned |
| [02-child-windows-service-package](workpacks/02-child-windows-service-package.md)                     | Child Windows package, service lifecycle, and respawn proof.         | Planned |
| [03-child-macos-service-package](workpacks/03-child-macos-service-package.md)                         | Child macOS package, launchd lifecycle, and notarization proof.      | Planned |
| [04-child-linux-service-package](workpacks/04-child-linux-service-package.md)                         | Child Linux package, service manager lifecycle, and package proof.   | Planned |
| [05-child-android-agent-package](workpacks/05-child-android-agent-package.md)                         | Child Android package, install proof, and device-owner gap proof.    | Planned |
| [06-child-ios-agent-capability-package](workpacks/06-child-ios-agent-capability-package.md)           | Child iOS capability package and provisioning/manual-required proof. | Planned |
| [07-child-managed-service-respawn](workpacks/07-child-managed-service-respawn.md)                     | Managed respawn and restart-survival states by platform.             | Planned |
| [08-child-parent-authorized-uninstall](workpacks/08-child-parent-authorized-uninstall.md)             | Parent-authorized uninstall, revocation, and negative-case proof.    | Planned |
| [09-child-signing-store-device-owner-matrix](workpacks/09-child-signing-store-device-owner-matrix.md) | Signing, store, and device-owner matrix by platform.                 | Planned |
| [10-setup-device-trust-handoff](workpacks/10-setup-device-trust-handoff.md)                           | Setup-device-trust request/response contract.                        | Planned |
| [11-proof-ci-release-gate](workpacks/11-proof-ci-release-gate.md)                                     | Proof storage, route sync, CI gate, and PR-ready closure.            | Planned |

Child package work stays separate from parent client distribution and must not reuse parent proof pointers.
