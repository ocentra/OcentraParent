# 40 - Model Catalog Artifact Integrity Lane

## Target State

Local model artifacts have ids, versions, checksums, licenses, source, download
state, install path, cache status, corruption state, and removal behavior.

## Where We Are

Model artifact contracts and runtime cache status exist. Product-grade artifact
integrity still needs proof.

## Checklist

- [ ] Define model catalog entry.
- [ ] Add artifact checksum and version.
- [ ] Add license/source fields.
- [ ] Add download/resume/corruption states.
- [ ] Keep model cache separate from evidence storage.

## Proof

- Artifact parser tests.
- Checksum/corruption tests.
- Cache separation test.
