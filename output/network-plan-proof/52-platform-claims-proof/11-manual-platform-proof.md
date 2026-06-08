# Network Platform Claim Manual Proof

Row: 52 Platform claim manifest proof

Fixture target rows:

- Windows Firewall: fixture Windows OS scope ref, adapter authorization ref, capability proof ref, target/rule capability refs, audit ref.
- Windows WFP: fixture target/provider/layer refs, administrator permission, driver signing/package, provider registration, layer capability, lab result, audit ref.
- Android VpnService: fixture package/service/device refs, VpnService declaration, user consent, package identity, virtual interface, traffic observation, Device Owner proof when claimed, audit ref.
- Apple Network Extension macOS: fixture bundle/extension/device refs, developer team, entitlement approval, provisioning, signing, declaration, configuration, supervision/MDM proof when claimed, audit ref.
- Apple Network Extension iOS: fixture bundle/extension/device refs, developer team, entitlement approval, provisioning, signing, declaration, configuration, supervision/MDM proof when claimed, audit ref.
- Linux nftables: fixture distro/kernel refs, permission, adapter API, adapter plan, service-manager scope, lab result, audit ref.
- Linux eBPF: fixture distro/kernel refs, permission, adapter API, adapter plan, service-manager scope, lab result, audit ref.
- Linux TUN: fixture distro/kernel refs, permission, adapter API, adapter plan, service-manager scope, lab result, audit ref.

Manual-required and unavailable labels:

- Missing WFP administrator permission records manual follow-up label `windows-wfp.administrator-permission`.
- Unavailable Linux TUN rows remain visible, do not authorize adapter apply, and record follow-up label `linux-adapter.permission` when permission proof is absent.
- Non-ready platform rows cannot carry adapter authorization, so dry-run, research-only, manual-required, and unavailable states remain non-executable.
- Local host probe observations are written to `local-platform-observations.json`: Windows read-only command summaries, Android SDK/emulator availability, row40a named physical-target identity summaries, WSL/Linux tool availability, and macOS/iOS CI/manual-unavailable boundaries.

Screenshots/logs:

- UI screenshots are N/A for this row because the proof is a Rust manifest/harness boundary, not a portal rendering change.
- Command logs are written by this harness under `output/network-plan-proof/52-platform-claims-proof/`.

Known follow-up owner:

- Platform adapter implementation owners must replace fixture refs with real OS/device/permission artifacts before any production platform support claim.

No-claim boundary:

- No generic platform support.
- No live adapter execution.
- No host packet blocking.
- No exact URL from network-only evidence.
- No decrypted payload or page content.
- No UI policy authority.
- No enforcement command publication.
