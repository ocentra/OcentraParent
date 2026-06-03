# 45 - Remote Redacted Report Assistant Lane

## Target State

Remote parent assistant can explain parent-approved reports with citations,
redaction, retention state, and uncertainty, while local safety remains
authoritative.

## Where We Are

Parent assistant routing proof exists. Report assistant must wait for
parent-owned source bundle/custody proof.

## Checklist

- [ ] Define report bundle source refs.
- [ ] Add redaction/minimization state.
- [ ] Require parent approval.
- [ ] Require cited answer.
- [ ] Degrade to local-only on remote failure.

## Proof

- Remote report request tests.
- Citation required tests.
- Retention state portal proof.
