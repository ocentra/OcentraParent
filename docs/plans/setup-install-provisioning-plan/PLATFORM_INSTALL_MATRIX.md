# Platform Install Matrix

## Purpose

Track parent and child install state separately per platform. Installed, downloaded, and launchable are not the same as trusted or ready.

## Parent platform matrix

| Platform | Parent bootstrap scope | Expected visible states |
| --- | --- | --- |
| Windows | Bootstrap installer, code entry, full parent package, update channel | `available`, `previewOnly`, `manualRequired`, `unsupported`, `downloadReady`, `downloadFailed`, `installStarted`, `installSucceeded`, `installFailed`, `updateRequired`, `rollbackAvailable`, `notImplemented` |
| macOS | Bootstrap installer, code entry, full parent package, signing/notarization handoff | same as Windows plus Mac-specific signing/notarization gaps |
| Linux | Bootstrap installer, code entry, package selection, service handoff | same as Windows plus distro/service-manager gaps |
| Android parent | Parent app bootstrap, code entry, package preview, permission visibility | same as Windows plus Play/permission/manual-required gaps |
| iOS parent | Parent app bootstrap, code entry, TestFlight/App Store handoff | same as Windows plus Apple entitlement/gateway gaps |
| Web-only fallback | Account, download, support, status, and bootstrap routing | `available`, `previewOnly`, `manualRequired`, `unsupported`, `blocked`, `notImplemented` |

## Child platform matrix

| Platform | Child bootstrap scope | Expected visible states |
| --- | --- | --- |
| Windows child agent | Child bootstrap installer, permissions, service, signed hello | `notInstalled`, `downloaded`, `installed`, `serviceRegistered`, `serviceRunning`, `serviceStopped`, `permissionMissing`, `permissionDenied`, `permissionGranted`, `paired`, `trusted`, `policyReady`, `degraded`, `manualRequired`, `unsupported`, `revoked`, `reinstallRequired` |
| macOS child agent | Child bootstrap installer, permissions, launchd handoff | same as Windows plus notarization and privacy permission gaps |
| Linux child agent | Child bootstrap installer, systemd handoff, distro gates | same as Windows plus distro/service-manager gaps |
| Android child agent | Child bootstrap installer, foreground service, managed-device proof | same as Windows plus Device Owner/Accessibility/VPN/DNS gaps |
| iOS child agent | Child bootstrap installer, entitlement and device proof | same as Windows plus Family Controls/DeviceActivity/Network Extension gaps |
| Unsupported/manual-only | No real child agent runtime yet | `manualRequired`, `unsupported`, `notImplemented` |

## Rule

The install matrix must show what is real, what is preview-only, what is manual-required, and what is not implemented. A download button alone never proves production readiness.
