# Child Permission Matrix

## Purpose

Define which child-side permissions and disclosures are required before a child install can be called trusted or policy-ready.

## Matrix columns

- Windows
- macOS
- Linux
- Android
- iOS
- Status
- Parent-visible setup state
- Child-visible disclosure
- Manual-required proof
- Owning plan

## Permission rows

- screen capture
- browser URL evidence
- app/game usage
- network/domain metadata
- location
- notifications
- remote view
- remote control
- background service
- startup/login item
- tamper/uninstall resistance
- device owner / managed profile
- Accessibility
- UsageStats
- VPN/DNS
- FamilyControls / DeviceActivity / ManagedSettings

## Permission states

- `notRequired`
- `required`
- `granted`
- `denied`
- `missing`
- `unsupported`
- `manualRequired`
- `degraded`
- `revoked`

## Disclosure states

- `notRequired`
- `requiredNotShown`
- `shown`
- `accepted`
- `declined`
- `platformPromptShown`
- `manualRequired`

## Rule

Sensitive child-side features must not be treated as ready unless the permission/disclosure state is visible to the parent and the child-facing disclosure is represented honestly.
