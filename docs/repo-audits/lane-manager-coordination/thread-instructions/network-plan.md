# network-plan Instruction

## Verdict

`partial / false-green-risk`. Real code exists; proof roots are missing; parent-domain shims and inline Rust tests block closure.

## Assign first

`network-foundation-shim-cleanup`:

- finish cleanup of `packages/parent-domain/src/network*.ts` shims;
- keep `packages/network-domain` canonical;
- rerun scoped TS tests and architecture on touched files;
- create canonical proof root skeleton with generator command mapping.

## Then

1. `network-rust-test-rehome`: move network Rust tests from `src` to crate `tests/` categories.
2. `network-parser-capture-runtime-proof`: parser/capture/runtime proof bundles.
3. `network-platform-proof`: Windows/Android/Linux feasible artifacts; Apple external.

## Coordinate with

- `eventing-plan` for event transport/consumer proof.
- `v0-8-enforcement-control-plan` for report-only/enforcement boundary.
- `browser-plan`, `screen-plan`, `ai-plan`, and `lan-plan` for cross-slice evidence rows.

## Do not

- Do not count `parent-domain` network wrappers as ownership.
- Do not claim platform proof from status wiring only.
- Do not count inline Rust tests as final category coverage.
