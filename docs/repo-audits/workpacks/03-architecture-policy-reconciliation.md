# WP03 Architecture Policy Reconciliation

## Objective

Resolve architecture-gate truth before accepting repo-wide or plan-level architecture-clean claims.

## Scope

Inspect:

- `AGENTS.md`
- `.ocentra-ai/rules/*`
- `scripts/check-architecture-policy.mjs`
- `scripts/check-no-reexports.mjs`
- `tools/no-reexports/**`
- representative crate/package roots

## Questions to answer

| Question | Required answer |
| --- | --- |
| Is no-reexport policy global today? | Yes, staged, or exception-based. |
| Does full repo pass? | Exact command and result, or explicit known blockers. |
| What is allowed during migration? | Changed-scope only, package-scope, or full cleanup. |
| What must a plan report say? | Exact architecture gate scope and remaining debt. |

## Known starting risks

- Rust `pub use` is banned by rule/tool but appears in many crate roots.
- TypeScript export-forwarding/barrels exist or recently existed in shared package roots.
- Scoped architecture passes can be misreported as repo-wide clean.

## Output table

| Surface | Violation type | Scope | Owner | Fix strategy | Blocker? |
| --- | --- | --- | --- | --- | --- |

## Acceptance

- Current policy state is explicit.
- Repo-wide versus scoped clean is impossible to confuse.
- Any temporary exception has owner, reason, and removal trigger.

## Failure conditions

- Claiming architecture clean from changed-file-only validation.
- Adding new barrels/re-exports as adapters.
- Hiding export debt under broad package roots.
