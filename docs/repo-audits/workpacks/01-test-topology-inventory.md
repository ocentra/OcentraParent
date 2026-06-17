# WP01 Test Topology Inventory

## Objective

Build the repo-level test topology inventory before accepting any plan completion claim.

## Scope

Inspect:

- `crates/*/src/**/*.rs`
- `crates/*/tests/**`
- `packages/*/tests/**`
- `apps/*/tests/**`
- `apps/*/e2e/**`
- `scripts/test/**`

## Required classification

| Class | Meaning |
| --- | --- |
| real test | Executable test file with assertions and command path. |
| empty scaffold | `.gitkeep` or empty folder with no executable tests. |
| inline private seam | `#[cfg(test)]` test justified by private helper or service wiring. |
| inline move candidate | `#[cfg(test)]` test that exercises public behavior and should move to crate-level `tests/`. |
| proof script | Script that generates proof artifacts under ignored local/CI output paths. |

## Output table

Create or update an inventory table with:

| Surface | Real tests | Empty scaffolds | Inline src tests | Move candidates | Proof scripts | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |

## Acceptance

- Empty scaffold folders are listed explicitly.
- Inline Rust tests are classified, not just counted.
- Packages/crates with only minimal smoke tests are marked weak.
- No plan report may count a test category until this inventory confirms executable tests exist.

## Failure conditions

- Counting folder names as tests.
- Counting generated proof artifacts as tests.
- Treating `scripts/check-required-tests.mjs` as full coverage proof.
