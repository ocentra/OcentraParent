# ocentra-parent-agent-maintenance

Updater and maintenance tooling for the installed Ocentra Parent agent.

## Owns

- Signed update manifest verification.
- Updater binary and maintenance command tooling.
- Release/update helper logic for installed agent lifecycle.

## Must Not Own

- Child safety decisions.
- Capture, AI, policy, or enforcement behavior.
- Billing entitlement logic.
- Silent install/update claims without release proof.

## Flow

```mermaid
flowchart LR
  Manifest["signed update manifest"]
  Updater["agent updater"]
  MSI["agent installer package"]
  Service["installed agent service"]
  Manifest --> Updater --> MSI --> Service
```

## Connected Docs

- [Release installer expectations](../../docs/expectations/release-installer.md)
- [Production hardening expectations](../../docs/expectations/roadmap-v8-production-hardening.md)
- [Release and update architecture](../../docs/architecture/release-update.md)

## Gaps To Fill

- Production signing, release channel, rollback, and support runbooks.
- Platform-specific update strategy outside Windows.
- Parent-visible update status and failure recovery.
