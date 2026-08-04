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
