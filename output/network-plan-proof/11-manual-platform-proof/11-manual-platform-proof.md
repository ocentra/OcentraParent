# Network Manual Platform Proof

Branch: codex/network-manual-platform-proof
Source commit: 281854e1cfac440706126bbc9b5e8fe306614fb2
Source status: clean

This proof aggregates the existing platform-specific Rust proof gates into the required network-plan row 11 manual/platform proof pack.
It names the OS/device/permission evidence needed before platform claims can move beyond manual-required, unavailable, research-only, or proof-gated state.

## Windows - Npcap live capture observation
Proof rows: 13
Manual-required label: manual-required until host driver, permission, bounded capture, stop, quota, retention, custody, and private-traffic refs are supplied
Required permission: Administrator or equivalent Npcap capture permission on a named child device interface
Exact manual steps:
1. Identify child Windows host, Ocentra agent build, network interface, and Npcap installation version.
2. Attach driver, interface, permission, bounded-capture, clean-stop, quota, retention/delete/export, custody, and private-traffic-exclusion refs.
3. Run the live-capture proof gate and retain command log plus host/device proof evidence before claiming capture readiness.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Windows - DNS proxy/block/redirect and Windows Firewall adapter proof boundaries
Proof rows: 37, 38
Manual-required label: manual-required/unavailable unless supported capability, policy, apply/result, rollback, and audit refs are present
Required permission: Host DNS configuration or Windows Firewall administrative permission, depending on adapter kind
Exact manual steps:
1. Name child Windows host, adapter kind, parent rule ref, evidence ref, and policy decision ref.
2. Attach target/rule, supported capability, adapter authorization, apply/result, rollback, and audit refs.
3. Keep dry-run/manual/unavailable states non-executable until artifact refs are present and validated.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Windows - WFP signed/permissioned lab proof gate
Proof rows: 39
Manual-required label: manual-required unless signed driver/package, admin permission, provider registration, layer matrix, rollback, lab result, and audit refs are present
Required permission: Administrator permission plus signed Windows Filtering Platform driver/package proof
Exact manual steps:
1. Name child Windows host, WFP target/provider/layer refs, and signed package version.
2. Attach administrator permission, driver signing/package, provider-registration, layer-capability, rollback, lab-result, and audit refs.
3. Keep research-only/manual/unavailable states non-executable and do not claim packet blocking without the lab proof pack.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Android - VpnService physical-device proof gate
Proof rows: 40
Manual-required label: manual-required until physical device, VpnService declaration, consent, package identity, interface, traffic observation, rollback, and audit refs are present
Required permission: Android VpnService user consent, with Device Owner proof only when that authority is claimed
Exact manual steps:
1. Name physical child Android device, OS version, package identity, service declaration, and VpnService consent artifact.
2. Attach virtual-interface, traffic-observation, rollback, audit, and physical-device proof refs.
3. Attach Device Owner proof only if the product claim uses Device Owner authority.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Apple - Network Extension entitlement/device proof gate
Proof rows: 41
Manual-required label: manual-required until entitlement, provisioning, signing, device/TestFlight, extension configuration, rollback, and audit refs are present
Required permission: Approved Apple Network Extension entitlement and device/TestFlight proof; supervision/MDM proof only when claimed
Exact manual steps:
1. Name Apple device, OS version, developer team, entitlement approval, provisioning profile, signing, and bundle/extension refs.
2. Attach device/TestFlight, extension declaration/configuration, rollback, and audit refs.
3. Attach supervision or MDM proof only if the product claim relies on managed-device authority.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Linux - nftables/eBPF/TUN distro proof gate
Proof rows: 42
Manual-required label: manual-required until distro/kernel, permission, adapter API, service-manager, rollback, lab result, and audit refs are present
Required permission: Distro-specific privileged network adapter permission for nftables, eBPF, or TUN
Exact manual steps:
1. Name distro, kernel, service manager, selected adapter kind, and child host proof.
2. Attach permission, adapter API capability, adapter plan, service-manager scope, rollback, lab-result, and audit refs.
3. Keep generic Linux support unavailable until the selected distro/kernel proof pack exists.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Not Claimed
- live packet capture driver invocation
- host DNS mutation, proxy installation, Windows Firewall mutation, WFP driver install, or packet blocking
- Android VPN tunnel/filtering or Device Owner behavior without physical-device proof
- Apple Network Extension behavior, supervision, MDM, or app-level control without entitlement/device proof
- Linux adapter install, packet filtering, kernel hook load, TUN mutation, or service-manager install
- exact URL, page content, private message, search query, or decrypted payload availability
- policy authority, adapter action authority, or enforcement command publication

Screenshot policy: Screenshots are not attached because this slice is a non-UI contract/proof harness; live host or device proof must attach screenshots/logs before platform capability claims are upgraded.
