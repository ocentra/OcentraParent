# WP01 Network Parser-to-Policy Evidence Handoff

## Scope

- `crates/ocentra-network-evidence/src/parser_policy.rs`
- `crates/ocentra-network-evidence/src/lib.rs`
- `crates/ocentra-network-evidence/tests/unit/parser_policy.rs`
- `crates/ocentra-network-evidence/tests/unit/mod.rs`

## Proven boundary

`NetworkParserEvidence` is a bounded metadata-only handoff from a parser to the
existing evidence-grade policy mapper. The handoff requires a stable parser
evidence reference, carries an explicit A/B/C/D grade, and rejects exact-URL or
decrypted-payload claims before policy mapping. A valid HTTP Host parser result
therefore reaches the existing parent-review/dry-run/observe-only mapping path,
while adapter and enforcement authority remain false.

## Proof routing

- `workpack`: `docs/plans/network-plan/workpacks/01-foundation-contracts-and-eventing.md`
- `checklist_rows`: WP01 contract, parser, and evidence-grade/policy-handoff obligations (bounded sub-slice only)
- `schema_owner`: `crates/ocentra-network-evidence` owns this crate-local handoff; no cross-language schema claim is made
- `rust_owner`: `crates/ocentra-network-evidence`
- `protocol_owner`: not changed by this sub-slice
- `eventing_owner`: not changed by this sub-slice
- `evidence_grade_state`: explicit parser handoff grade is validated and passed to the existing mapper
- `policy_handoff_state`: parser metadata maps to the existing non-enforcing policy handoff modes
- `enforcement_authority_state`: not claimed; both authority flags remain false
- `private_bus_state`: not changed; no private bus or event route is added
- `schema_fixture_ref`: `crates/ocentra-network-evidence/tests/unit/parser_policy.rs`
- `rust_parity_ref`: not applicable; no cross-language contract changed
- `eventing_workpack_ref`: not applicable; no eventing route changed
- `no_claim`: no exact URL, decrypted payload, content, adapter execution, or enforcement command is asserted

## Required negative cases

- Invalid UTF-8 HTTP input returns no parser observation and cannot produce a
  policy evidence reference.
- A blank parser evidence reference is rejected before policy mapping.
- Exact-URL and decrypted-payload flags are rejected before policy mapping.
- Blank policy refs still fail through the existing policy mapper and retain its
  typed error.

## Validation record

```text
command: cargo test -p ocentra-network-evidence --test unit parser_policy -- --nocapture
exit: 0
result: pass
notes: 4 focused parser-to-policy tests passed; malformed parser bytes, blank parser refs, unsupported claims, policy-ref validation, and non-enforcing parent-review mapping are covered

command: npm run lint:architecture -- --files crates/ocentra-network-evidence/src/parser_policy.rs crates/ocentra-network-evidence/src/lib.rs crates/ocentra-network-evidence/tests/unit/parser_policy.rs crates/ocentra-network-evidence/tests/unit/mod.rs
exit: 0
result: pass
notes: scoped architecture gate passed after removing a forbidden Rust facade re-export

command: cargo fmt --all -- --check
exit: 0
result: pass
notes: workspace formatting check passed after formatting the touched module registration and proof tests
```

## No-claim boundary

This is a bounded parser-to-policy contract and negative-path proof. It does not
close the full WP01 schema parity, protocol, eventing, service-runtime, platform,
live-capture, analyzer, AI, adapter, or production enforcement obligations.
