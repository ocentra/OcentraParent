# WP08 validation commands

Lane: `account-authority-wp08`
Branch: `codex/account-authority-wp08`
Worktree: `E:\OcentraWorktrees\lanes\account-authority-wp08`

| Command | Exit | Result | Notes |
| --- | ---: | --- | --- |
| `cargo test -p ocentra-schema --test contract` | 0 | pass | 98 passed |
| `cargo test -p ocentra-schema --test contract family_references_generated_typescript_matches_checked_in_file` | 0 | pass | 1 passed; 97 filtered |
| `cargo test -p ocentra-family-identity-core household_authority` | 0 | pass | 15 passed |
| `cargo test -p ocentra-family-identity-core --test unit setup_lifecycle` | 0 | pass | 18 passed |
| `cargo test -p ocentra-family-identity-core --test unit session_lifecycle` | 0 | pass | 13 passed |
| `cargo test -p ocentra-family-identity-core --test unit parent_presence_verification_rejects_duplicate_issuance_without_overwriting_original_binding` | 0 | pass | 1 passed; 135 filtered |
| `cargo test -p ocentra-family-identity-core --test unit parent_presence_verification_rejects_malformed_noncanonical_and_offset_timestamps` | 0 | pass | 1 passed; 135 filtered |
| `cargo test -p ocentra-family-identity-core --test contract device_registration_rejects_wrong_household_child_binding` | 0 | pass | 1 passed; 11 filtered |
| `npm run lint:architecture -- --files crates/schema crates/family-identity-core packages/schema-domain/src/generated-family-references.ts` | 0 | pass | focused architecture policy passed |

No Cloudflare command was substituted: Cloudflare WP06/WP08 own migration and
runner validation.

## Record-derived authority handoff recovery (2026-08-05)

| Command | Exit | Result | Notes |
| --- | ---: | --- | --- |
| `cargo fmt --check -p ocentra-family-identity-core` | 0 | pass | formatted Rust handoff and contract test |
| `cargo check -p ocentra-family-identity-core` | 0 | pass | Rust authority crate check |
| `cargo test -p ocentra-family-identity-core --test contract household_authority_handoff` | 0 | pass | 4 record-derived handoff contract tests |
| `cargo test -p ocentra-family-identity-core household_authority` | 0 | pass | existing 15 authority tests plus the matching contract filter |
| `cargo test -p ocentra-family-identity-core --test unit session_lifecycle` | 0 | pass | 13 session freshness/replay tests |
| `cargo test -p ocentra-family-identity-core --test contract` | 0 | pass | 16 contract tests, including the four record-derived handoff cases |
| `npm run lint:architecture -- --files crates/family-identity-core` | 0 | pass | focused architecture policy passed after exact serialized-envelope redaction assertion |
| `git diff --check` | 0 | pass | no whitespace errors in the recovery slice |

No Cloudflare D1 migration or runner command was run or claimed. Those remain
owned by Cloudflare WP06 and WP08 respectively.
