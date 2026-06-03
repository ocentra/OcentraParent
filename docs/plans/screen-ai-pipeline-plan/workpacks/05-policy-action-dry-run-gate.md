# 05 - Policy Action Dry-Run Gate

## Target State

Pipeline proves policy actions or dry-run actions without letting AI enforce
directly.

## Checklist

- [ ] Observe action proof.
- [ ] Allow action proof.
- [ ] Warn action proof.
- [ ] Ask-parent action proof.
- [ ] Time-limit action proof with timer/expiry refs.
- [ ] Block dry-run or real adapter proof.
- [ ] Unknown/manual-required proof.

## Proof

- Action or dry-run artifact.
- Enforcement-adapter non-claim when real adapter is not in scope.
- Audit event artifact.
