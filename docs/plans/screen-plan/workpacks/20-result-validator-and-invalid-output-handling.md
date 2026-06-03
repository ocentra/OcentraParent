# 20 Result Validator And Invalid Output Handling

## Target State

Invalid JSON, missing refs, invalid confidence, unsupported categories, raw text overflow, and malformed deletion state are rejected.

## Current State

Validation direction exists; complete malformed-output proof is open.

## Checklist

- [ ] Reject invalid JSON.
- [ ] Reject missing source refs.
- [ ] Reject invalid confidence.
- [ ] Reject unsupported categories.
- [ ] Reject raw text overflow.
- [ ] Reject missing/malformed deletion state.
- [ ] Record invalid/unknown state.

## Proof

- Negative contract tests.
- Service tests showing invalid output cannot drive policy.
