# 24 Portal UX And First-Run Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `24 Portal UX And First-Run Handoff`
> Kind: assigned active workpack; read only when this exact workpack is selected.
> Read when: Only when this exact workpack is explicitly selected from `WORKPACK_INDEX.md`.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack's own proof rows and tests support the claim.
> Proves: only this workpack's current presentation/handoff boundary and progress explicitly recorded here.
> Does not prove: current completion of sibling workpacks or broad LAN readiness.
> Proof rule: Rewrite or discard any stale historical assumptions before using this file for execution claims.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../../features/family-setup-device-roles.md),
[family setup expectations](../../../expectations/family-setup.md).
Assumes earlier Rust-owned LAN contract, runtime, and route-proof workpacks
already expose the states and typed commands this UI slice may consume.

## Active scope status

This workpack is part of the authoritative `01-25` LAN execution model. It is
locally complete for the current presentation/handoff slice.

Historical portal-owned implementation recipes, TS-owned command assumptions,
and exact UI file-tree prescriptions from older copies of this draft are stale.
Current direction for this workpack is:

- Rust owns all LAN logic, contracts, runtime behavior, read models, and proof.
- TS is pure UI/presentation only.
- Any UI route, panel, wizard, or badge must consume Rust-backed host-bridge
  state and typed commands only through Rust-owned host-bridge interfaces.
- UI wording, screenshots, and browser tests do not become LAN truth or proof
  authority.

## Where We Are

The product already has Rust-backed LAN state for source labels, route
readiness, trust state, signed-discovery status, stale/offline status, and
parent decision fields. This locally complete slice records the parent-facing
presentation and handoff layer that turns those Rust-backed states into
understandable, nontechnical UX.

The current completed work in this slice is about presentation honesty, not TS
ownership:

- showing first-run or recovery state without inventing business rules;
- rendering route/source/trust state in plain language while preserving
  unknown/manual-required/degraded truth;
- keeping observer/read-only and protection-readiness boundaries explicit; and
- carrying manual-required or deferred flows honestly when the Rust layer does
  not yet prove them.

## Where We Want To Be

A parent-facing surface should eventually be able to consume the Rust-owned LAN
read model and typed command set so that:

1. first-run and add-device flows explain what the parent can do next without
   exposing raw LAN internals as product authority;
2. recovery and degraded states stay visible and understandable;
3. route/source/trust labels are presented in plain language derived from
   Rust-backed state only; and
4. the UI never upgrades a presentation hint into a claim that a device is
   protected, paired, reachable, or ready unless the Rust-owned runtime state
   already says so.

## Presentation boundary

- Rust shared schema/protocol/runtime/read-model crates remain the sole owners
  of LAN command semantics, route labels, trust labels, stale/offline state,
  recovery state, and proof truth.
- TS presentation surfaces may collect parent intent, render Rust-backed state,
  and dispatch typed bridge commands through existing host-bridge contracts.
- TS must not define LAN contracts, invent fallback route logic, infer recovery
  state, or normalize weak hints into business truth.
- Screenshots, browser checks, or visual fixtures are supporting presentation
  artifacts only. They do not close LAN proof by themselves.

## Scope

- Define the presentation-only handoff expectations for first-run, recovery,
  route/source explanation, observer/read-only state, and degraded/manual
  states.
- Keep this slice limited to UI/presentation behavior that consumes Rust-owned
  state and typed commands.
- Do not prescribe portal filesystem layouts, route names, component names, or
  code recipes.
- Do not let future UI flow work recreate TS business ownership; broader
  first-run/platform proof stays in its owning workpack.

## Tests And Proof

- Rust-owned LAN contract/runtime/read-model proof for the consumed states and
  commands must already exist before this workpack can claim presentation
  closure.
- Any UI checks must live in real dedicated UI test folders/groups and prove
  presentation behavior only.
- Required presentation coverage should include: first-run empty/degraded
  states; stale/offline/revoked/manual-required rendering; route/source/trust
  explanation copy; and observer/read-only command visibility boundaries.
- UI tests must not become substitutes for Rust-owned logic, contract, runtime,
  read-model, or proof coverage.
- Inline source-owned tests, placeholder directories, `.gitkeep` trees, fake
  coverage, mock-only readiness, and screenshot-only closure do not count.
- Supporting presentation artifacts may be attached only under a current proof
  root and must carry explicit no-claim notes when the underlying Rust proof is
  still open.
- Proof artifact: `output/lan-plan-proof/24-portal-ux-and-first-run-handoff/`
- Current proof: `output/lan-plan-proof/24-portal-ux-and-first-run-handoff/01-local-validation.md`
- Current validation covered focused portal unit tests, portal build, and the
  exact Windows `setup-first-run-ui-proof` Playwright command. This is
  presentation support only and does not become LAN truth.

## AI Worker Checklist

- [x] Confirm WP24 is the assigned active workpack.
- [x] Rewrite any stale TS-owned portal/business wording still present in this
      file before code moves.
- [x] Confirm the consumed commands and read-model states are already Rust-owned
      before UI work starts.
- [x] Keep TS pure presentation only; no TS-owned LAN command semantics,
      contracts, runtime state, or proof truth.
- [x] No presentation copy upgrades a device into paired, protected, reachable,
      or controllable state before the Rust-owned runtime says so.
- [x] All claimed UI tests live in real dedicated UI test folders/groups; no
      inline source-owned, placeholder, `.gitkeep`, fake, or mock-only test
      surfaces count.
- [x] Manual-required and deferred states remain explicit in both proof and copy.

## Manual-Required Gaps

Camera, OS permission, QR, push-delivery, or other platform-specific first-run
flows remain separate manual-required or deferred proof unless a later artifact
set proves them. UI polish or screenshot completeness alone must not upgrade
those rows into LAN readiness.
