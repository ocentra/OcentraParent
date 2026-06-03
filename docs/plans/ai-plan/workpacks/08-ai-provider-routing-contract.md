# 08 - AI Provider Routing Contract

## Target State

Every AI task explicitly routes to deterministic, local text LLM, OCR, VLM,
embedding, or parent-approved remote assistant.

## Where We Are

Local provider routing proof exists for parent assistant states. The product
router must cover all AI task families.

## Checklist

- [ ] Define provider route contract.
- [ ] Add task-to-route matrix.
- [ ] Make deterministic route first.
- [ ] Make remote route unavailable for normal child safety.
- [ ] Record selected route in result journal.

## Proof

- Route selection tests.
- Unsupported task tests.
- Remote safety-path rejection tests.
