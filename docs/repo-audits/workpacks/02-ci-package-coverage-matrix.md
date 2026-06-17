# WP02 CI Package Coverage Matrix

## Objective

Map every crate, package, and app to the local command and CI job that actually validates it.

## Scope

Inspect:

- `Cargo.toml`
- `package.json`
- `package-lock.json`
- `.github/workflows/*.yml`
- root npm scripts
- crate/package local scripts

## Required output

| Surface | Local build/check | Local test | CI job | CI command | Covered? | Gap |
| --- | --- | --- | --- | --- | --- | --- |

## Required distinctions

| Claim wording | Required proof |
| --- | --- |
| CI covered | Exact workflow job and command. |
| Local covered | Exact local command and last run evidence. |
| Workspace covered | Command must include every workspace member or use a generated matrix. |
| Contract covered | Must identify specific package tests, not just `test:contract` name. |

## Known starting risks

- Rust CI is segmented and does not automatically equal `cargo test --workspace`.
- Root `build:contracts` is hand-maintained.
- Root `test:contract` may cover only selected workspaces.
- Some package workspaces may exist but not appear in the root build/test gate.

## Acceptance

- Every `crates/*` workspace member has a row.
- Every `packages/*` workspace package has a row.
- Every `apps/*` workspace app has a row.
- Omitted surfaces are explicit and justified.

## Failure conditions

- Saying “CI passed” without naming the job and command.
- Treating `Full Validation Gate` aggregation as a test command.
- Treating local `npm run validate` and CI segmented jobs as equivalent.
