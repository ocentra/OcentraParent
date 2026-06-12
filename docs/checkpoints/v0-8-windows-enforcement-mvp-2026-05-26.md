<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Windows Enforcement MVP Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.8 Windows Enforcement MVP Proof

## Scope

This record covers the first real V0.8 Windows enforcement MVP path on branch
`codex/v0.8-windows-enforcement-mvp`. It is not a docs-only proof spine.

The implemented product path is intentionally narrow:

- a typed `agent.enforcement.execute` WebSocket command from the parent/control
  side to the child-device Rust service;
- service-side parsing of the policy decision, target, evidence references,
  action/result/audit ids, rollback token, and process target;
- core authorization that builds an adapter request only for a schema-valid,
  non-dry-run, supported process-control decision;
- Windows process-control execution for an owned local process target;
- typed `agent.enforcement.audit.reported` response payload;
- encrypted activity journal plus SQLite ingest using
  `activity.enforcement.audit-recorded`;
- sanitized machine-readable proof output under ignored `test-results`.

## Proof Boundary

This is a real local service-to-adapter proof for process termination, but it is
not a full production enforcement product claim.

| Area                                | State                                                                                                 |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Windows process termination         | Proven through a real spawned `node.exe` child process owned by the harness.                          |
| Typed command and event protocol    | Implemented in TypeScript protocol-domain contracts and Rust protocol parity.                         |
| Core authorization                  | Implemented before adapter execution; dry-run and unavailable paths do not request adapters.          |
| Audit journal and query ingest      | Implemented through the existing encrypted journal and SQLite ingest path.                            |
| Timer and rollback fields           | Preserved in action/result/audit payloads; proof records timer `created` and rollback `not-required`. |
| Network/domain blocking             | Not implemented in this slice.                                                                        |
| Managed-browser-only enforcement    | Not implemented in this slice.                                                                        |
| Timeout unblock/recovery executor   | Timer/recovery contracts exist; a real unblock scheduler is not implemented in this slice.            |
| Anti-tamper or production hardening | Not implemented or claimed.                                                                           |
| Portal UI changes                   | Not touched by worker A.                                                                              |
| LAN pairing/control                 | Owned separately by worker B for V0.9.                                                                |

## Local Proof Run

Command:

```powershell
node scripts/test/v0-8-windows-enforcement-mvp.mjs
```

Output summary:

```text
v0-8-windows-enforcement-mvp-ok=true
evidence=C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent\test-results\v0-8-windows-enforcement-mvp\2026-05-26T18-23-58-766Z.json
status=actually-enforced adapter=process-terminated rollback=not-required journal=audit-v08-owned-process
Compiling ocentra-parent-agent-service v0.1.1
Finished dev profile
```

Sanitized evidence summary:

| Field              | Value                                                                  |
| ------------------ | ---------------------------------------------------------------------- |
| Generated UTC      | `2026-05-26T18:23:58.766Z`                                             |
| Platform           | `win32`                                                                |
| Agent endpoint     | `loopback-redacted`                                                    |
| Run root           | `test-results\v0-8-windows-enforcement-mvp\run-F7xpxf`                 |
| Child process      | `node.exe`, pid recorded in ignored artifact                           |
| Policy decision id | `decision-v08-owned-process`                                           |
| Status             | `actually-enforced`                                                    |
| Adapter result     | `process-terminated`                                                   |
| Rollback state     | `not-required`                                                         |
| Journal event id   | `audit-v08-owned-process`                                              |
| Timer event kind   | `created`                                                              |
| Database ready     | `true`                                                                 |
| Events stored      | `52`                                                                   |
| Activity journal   | `test-results\v0-8-windows-enforcement-mvp\run-F7xpxf\activity.ndjson` |
| Activity store     | `test-results\v0-8-windows-enforcement-mvp\run-F7xpxf\activity.sqlite` |

The harness does not manually insert database rows. It starts the real Rust
service, opens a real WebSocket command path, spawns a local process, asks the
service to enforce the typed policy decision, waits for the owned process to
exit, verifies the audit response, and verifies the activity journal/store
artifacts exist.

## Proof Labels

- `v0.8.enforcement.command-contract`
- `v0.8.enforcement.core-authorization`
- `v0.8.enforcement.windows-process-terminate`
- `v0.8.enforcement.service-websocket-path`
- `v0.8.enforcement.audit-journal`
- `v0.8.enforcement.timer-created`
- `v0.8.enforcement.rollback-state`

## Validation Ledger

Focused validation run before this record:

| Command                                                                  | Result                                         |
| ------------------------------------------------------------------------ | ---------------------------------------------- |
| `cargo fmt --all`                                                        | Passed                                         |
| `cmd /c npm run lint:schema-boundaries`                                  | Passed; advisory source-shape warnings remain. |
| `cmd /c npm run test --workspace @ocentra-parent/activity-domain`        | Passed, 11 files and 38 tests.                 |
| `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain`  | Passed, 3 files and 23 tests.                  |
| `cargo test -p ocentra-parent-agent-protocol enforcement -- --nocapture` | Passed, 10 tests.                              |
| `cargo test -p ocentra-parent-agent-core enforcement -- --nocapture`     | Passed, 18 tests.                              |
| `cargo test -p ocentra-parent-agent-service enforcement -- --nocapture`  | Passed, 4 tests.                               |
| `node scripts/test/v0-8-windows-enforcement-mvp.mjs`                     | Passed with evidence file above.               |
| `git diff --check`                                                       | Passed before this doc was added.              |
| `cmd /c npm run validate`                                                | Passed full root validation gate.              |

Final guard status and commit state are reported in the hub DONE message and PR
body outline.

## Remaining Gaps And Risks

- The process-control adapter only terminates a named process id supplied by the
  command and verified by process name. It is not a global application-blocking
  system.
- The proof target is an owned synthetic local process, not an installed child
  app, browser, game, or protected process.
- There is no network/domain blocking adapter in this slice.
- There is no managed-browser-only enforcement adapter in this slice.
- There is no real timeout unblock scheduler or restart-recovery executor in
  this slice.
- Rollback state is explicit, but process termination has no reversible
  rollback action.
- Dev builds remain uninstallable/debuggable and do not include anti-tamper
  hardening.
- Parent portal UI and C-owned portal/protocol surface files were not changed by
  this branch.
- LAN pairing/control remains separate V0.9 work owned by worker B.

## Next Owner Steps

1. Review the command/event contract and service proof for the narrow Windows
   process-control path.
2. Run full validation and PR CI before merging the branch to `main`.
3. Add a separate network/domain enforcement adapter only after the policy and
   rollback semantics are explicit.
4. Add a managed-browser-specific enforcement path separately from unmanaged
   process termination.
5. Add a real timer executor/recovery owner before claiming complete timeout
   enforcement.
6. Keep every future platform enforcement claim tied to a proof artifact, an
   unavailable/degraded state, or an explicit manual-required gap.
