# V0.7 AI Model Routing And Queue Plan

## Goal

Create a bounded local AI queue and provider router so each task uses the
cheapest safe worker first and heavier models only when required.

## Worker Lanes

1. Deterministic classifier.
2. Local text LLM.
3. OCR.
4. Guided VLM.
5. Embedding/memory worker.
6. Parent-approved remote assistant.

## Routing Rules

- Deterministic classifiers run before model calls.
- Local text model consumes typed evidence and summaries, not raw sources.
- OCR extracts visible text from approved temporary screen jobs.
- Guided VLM answers scoped visual questions only.
- Embeddings and graph updates cite stored evidence.
- Remote assistant is disabled for normal child safety.

## Queue Requirements

- priority;
- source evidence refs;
- parent-rule refs;
- task scope;
- provider route;
- model/runtime ref;
- timeout;
- cancellation;
- retry policy;
- resource class;
- custody label;
- result journal ref.

## Validation

- Queue parser tests.
- Provider route selection tests.
- Backpressure tests.
- Cancellation tests.
- Remote disabled-by-default tests.
- Invalid route rejection tests.
- Resource limit proof on child-device runtime.
