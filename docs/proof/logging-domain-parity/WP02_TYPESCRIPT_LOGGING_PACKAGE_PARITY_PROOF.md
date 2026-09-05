# Logging Domain Parity WP02 Durable Proof Manifest

Workpack: `WP-logging-domain-parity-02-typescript-logging-package-parity`

Reviewed checkout head: `dd65d5d1318db7e7c02d1c6c747b8e5ce9d28c4c`

Retained generated root:

```text
output/logging-domain-parity-proof/02-typescript-logging-package-parity/
```

The generated root is intentionally ignored by Git. This manifest records the
exact retained bundle verified in the canonical lane on 2026-09-02 so the
engineering graph can retain portable proof truth without pretending that an
ignored checkout-local directory is a checked-in reference.

## Retained bundle

| Artifact | Status | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `00-package-export-before-after.json` | passed | 2489 | `d4eabd35712195b4880c5620cf9e654c4ad1b04842783311b23fc5a2dc52d51d` |
| `01-typescript-parity-file-map.json` | passed | 4012 | `58777924feebf66251791e630c7e40604c7152d5e104ed042795d8a5e737b5c8` |
| `02-scope-defaults-proof.json` | passed | 653 | `f858e424ad45590669f38eeece0ef7f87dfef0a137d0676c19bf61490b54c80b` |
| `03-query-script-smoke.json` | passed | 639 | `b83ddce37e029a54fef2d2582511c6596fb21a5cf1f08dbecbdb08b2c853a064` |
| `16-validation-commands.log` | passed | 1210 | `f98370fbdd3970090e55f117678bb3f19a3fd4d3d5dcdd4cfe363ea726379a5d` |

The JSON artifacts all identify plan `logging-domain-parity`, workpack
`02-typescript-logging-package-parity`, and the no-claim boundary "local
TypeScript package parity only; not production telemetry or product runtime
logging readiness".

## Recorded commands

The retained validation log records exit code 0 for:

```text
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain
npm run test:query --workspace @ocentra-parent/logging-domain -- stats --scope=parent-test
```

The bounded query artifact records the parsed `parent-test` stats schema and
exit code 0. The repository-tooling proof-runner test was also re-run from the
reviewed checkout and passed all three cases, including the invariant that the
runner writes exactly these five retained artifacts with explicit blocked and
no-claim states:

```text
node --test tests/repo-tooling/logging-domain-wp02-parity-proof.test.mjs
```

## Proof conclusion

This bundle proves the bounded WP02 local TypeScript package parity and its
focused build, test, export, scope-default, and query-smoke evidence. It does
not prove production telemetry, installer or release custody, product-wide
logging readiness, repository-wide validation, normal pre-commit, CI, PR, or
merge state.

The central `CHECKLIST_INDEX.md` closeout remains a separate graph completion
requirement. It was not changed by this reconciliation because another active
Logging proof-restoration task owns that exact file.
