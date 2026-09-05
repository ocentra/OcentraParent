# WP203 App/Game Windows Local Policy Evidence Proof

> **Current status (2026-09-02): DONE FOR BOUNDED WINDOWS OBSERVER AND
> CONTRACT PROOF / SERVICE INTEGRATION OPEN.**
> The dedicated Rust Windows owner boundary samples AppIDSvc, AppLocker, and
> Device Guard/App Control into reduced booleans and bounded counts. The typed
> protocol carries that optional evidence on the Windows status row. The real
> no-argument Windows observer, strict parser negatives, and protocol non-claim
> contract passed with retained proof. Agent Service composition, runtime
> delivery, and broader blocking authority remain open.

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP203 App/Game Windows Local Policy Evidence Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Sample Windows local AppLocker and App Control policy evidence as parent-safe
counts and booleans before any broad app/game blocking claim.

This moves Windows broad-blocking work beyond static manual-gate preflight by
checking the local AppIDSvc service, AppLocker local policy readability, and
Device Guard/App Control state without storing raw policy XML, executable
paths, publisher rules, or private policy details.

## Implementation

Current production truth:

- `crates/app-game-windows-local-policy-ffi` owns the no-argument Windows
  observation. It anchors to the System32 Windows PowerShell binary, retains
  verified non-reparse handles, checks path identity and unprivileged write
  access, uses fixed arguments and environment, and enforces a five-second
  deadline plus a 4 KiB strict JSON boundary.
- `AppGameWindowsLocalPolicyEvidence` is an optional nested protocol value on
  Windows platform proof rows. It rejects unknown fields, non-redacted values,
  impossible state/count relationships, malformed opaque references, and any
  dispatch, enforcement, rollback, audit, provider, child, or private-data
  claim.
- Unit and Windows integration tests cover the strict parser, malformed and
  oversized output, count/state relationships, non-Windows unsupported
  behavior, and the real no-argument Windows observation.
- Agent Service does not consume the observer yet. Existing status-row
  constructors set the optional evidence to `None`; no readiness or host
  capability signal is inferred from this foundation.

## Validation

Focused validation for this workpack:

```powershell
cargo test -p ocentra-app-game-windows-local-policy-ffi --all-targets
cargo test -p ocentra-parent-agent-protocol --test contract app_game_platform_proof_status
```

Both commands passed on Windows `10.0.26200 x64` on 2026-09-02 through the
local evidence wrapper. The FFI command ran one real no-argument Windows
observation and four parser/negative tests. The protocol command ran four
focused contract tests for stable names, impossible counts/platforms, unknown
fields, invalid non-claims, and serialization without enforcement claims.

## Proof

- `output/app-game-plan-proof/203-app-game-windows-local-policy-evidence-proof/00-scope-summary.md`
- `output/app-game-plan-proof/203-app-game-windows-local-policy-evidence-proof/01-negative-case-proof.md`
- `output/app-game-plan-proof/203-app-game-windows-local-policy-evidence-proof/02-no-claim-boundary.md`
- `output/app-game-plan-proof/203-app-game-windows-local-policy-evidence-proof/16-validation-commands.log`

This retained bundle closes the bounded Windows observer and protocol-contract
proof only. Agent Service runtime composition remains open and therefore no
service readiness, adapter execution, enforcement, or delivery claim is made.

## Boundaries

Proved:

- Windows local AppLocker/App Control policy state can be sampled as
  parent-safe counts and booleans.
- The real no-argument Windows observer and strict output boundary execute on
  the current Windows host.
- Raw AppLocker policy XML, executable paths, publisher rules, and private
  policy details are not stored.
- Broad app/game blocking remains blocked until enforce proof, system-app
  allowlist proof, rollback proof, audit custody proof, adapter dispatch proof,
  and child-device delivery proof exist.

Not proved:

- Windows broad installed-app launch blocking execution.
- System-app allowlist execution.
- Rollback execution or audit custody.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw executable path custody, or raw policy XML custody.
