# Parent Desktop Runtime Package 20-Step Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `Parent Desktop Runtime Package 20-Step Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This plan turns D-lane work into one concrete parent desktop/runtime/package
program. It keeps packaging and shell proof separate from child-device runtime
authority.

This is a plan document only. It does not claim signed production releases,
store distribution, notarization, mobile child-agent support, remote relay,
privileged OS capability, or production support readiness until proof artifacts
exist.

Companion docs:
[Parent Desktop Runtime Package Test Blueprint](parent-desktop-runtime-package-test-blueprint.md)
and [Runtime Package Requirements Guide](runtime-package-requirements-guide.md).

## Product Boundary

- Owning feature: Production distribution and support.
- Secondary feature overlap: Child agent local service, remote/LAN/mobile
  platforms, and parent desktop shell.
- Main expectations: platforms, release installer, real evidence proof.
- Product goal: package the parent portal as a parent-owned desktop shell that
  connects to typed service state and proves platform/package mechanics honestly.
- Non-goals: child-device capture, policy evaluation, enforcement, timers, local
  AI execution, production publishing from `main`, store claims, and signing
  claims without credentials.

## 20-Step Plan

1. Establish the Tauri shell contract boundary.
   The desktop app packages the portal and connects to local/LAN/relay/cache
   paths through typed contracts. It does not become a child-device agent.

2. Add local service connection command proof.
   The Tauri command should check real Rust service reachability and return
   controller, observer, route, package, and provider state through typed values.

3. Show LAN route and controller state.
   Desktop surfaces should distinguish loopback, LAN, relay, cache, unavailable,
   selected child, controller lease, observer read-only, and stale/offline state.

4. Preserve parent observer read-only behavior.
   Observer state can view allowed read models but cannot write policy, approve
   requests, take controller lease, or imply child-device authority.

5. Add custody and source labels.
   Desktop and future package surfaces should label live local, LAN, relay,
   parent cache, parent-owned storage, Ocentra-hosted non-activity metadata, and
   unavailable states.

6. Keep parent mobile bridge separate.
   Parent mobile scaffold/proof can reuse contracts, but parent mobile does not
   mean Android/iOS child-agent support.

7. Prove Windows installer/package preview.
   Build and smoke-check Windows artifacts where available, while keeping
   signing/manual release states explicit.

8. Add cross-platform package preview matrix.
   Track Windows, macOS, Linux, Android parent, iOS parent, Android child, and
   iOS child states separately.

9. Represent update channel and rollback scaffold.
   Updater state should distinguish scaffold, unsigned dev preview,
   signature-required, rollback available, rollback unavailable, and production
   release boundary.

10. Keep signing, notarization, and store claims honest.
    Credentials, signing, notarization, Play signing, TestFlight/App Store, and
    production installer publishing remain manual-required until artifacts exist.

11. Add support diagnostics and redaction.
    Package/runtime support output should include version, commit, platform,
    service route, connection, package, and capability state with secrets and
    private data redacted.

12. Update privacy and release docs.
    Public/support docs should say what is packaged, what is local, what is
    hosted, what is signed, what is preview-only, and how parent data custody is
    handled.

13. Add desktop launch smoke.
    Launch smoke should verify the shell starts, reaches configured service
    state, handles unavailable service, and does not claim backend work.

14. Keep Tauri build/dev scripts managed.
    Scripts should use repo defaults, lane ports where configured, and managed
    process cleanup that does not affect unrelated Ocentra projects.

15. Add platform capability matrix.
    Package, shell, service, child-agent, parent mobile, Android child, iOS
    child, signing, store, and relay claims should map to implemented/scaffold/
    unavailable/manual-required proof states.

16. Preserve release branch boundary.
    `main` builds previews and PR CI. `production` or explicit promotion handles
    production release publishing.

17. Check GitHub Actions artifact proof.
    Package preview CI and artifact states should be part of PR/merge reports
    before claiming package readiness.

18. Add manual platform proof runbook.
    Manual proof should name host/device, command/UI action, permission state,
    package version, screenshot/log/proof JSON, and known gaps.

19. Keep product checklist and feature docs synchronized.
    Update production distribution, child service, remote/LAN/mobile, and
    checklist rows when proof status changes; report if a checklist lock blocks
    the update.

20. Close with PR, CI, and rollout gates.
    Primary reviews branch diff and validation, opens/watches PR when ready,
    merges after green CI, pulls main, and tells active workers to rebase.

## Implementation Order

1. Finish current D branch implementation and resolve checklist lock with primary
   coordination.
2. Validate local Tauri command and package proof scripts.
3. Sync feature docs/checklist once locks allow it.
4. Push D branch, open or request PR, and include detailed proof scope.
5. Add CI/artifact evidence before merging package-readiness claims.

## Validation Expectations

- Tauri build/dev/package checks for touched desktop paths where feasible.
- Service connection proof using the real Rust service path.
- Script tests for package proof and platform matrix output.
- Feature doc/checklist sync or explicit blocker report.
- Full `npm run validate` before PR-ready handoff when scope is integration-ready.

## Open Product Questions

- What is the first parent desktop artifact the user should manually inspect?
- Which signing/store credentials are available, and which remain future/manual?
- What support bundle fields are acceptable for early manual testing?
- Should parent mobile shell proof stay in D or move to a future mobile-specific
  lane after desktop package proof lands?
