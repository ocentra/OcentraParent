# 07 - Deletion Retention And Custody Gate

## Target State

Raw image custody is explicit. Default behavior deletes temporary images and
does not upload raw screenshots remotely.

## Checklist

- [ ] Queue image encrypted.
- [ ] Raw path redacted outside child agent.
- [ ] Delete after success.
- [ ] Delete after TTL.
- [ ] Delete failure visible.
- [ ] Remote/cloud screenshot upload disabled.
- [ ] Retention requires explicit opt-in if used.

## Proof

- Queue encryption artifact.
- Deletion proof artifact.
- Remote disabled proof.
- Retention non-claim or opt-in proof.
