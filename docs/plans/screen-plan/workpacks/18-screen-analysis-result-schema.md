# 18 Screen Analysis Result Schema

## Target State

Categories, risk signals, text snippets, redaction notes, confidence, uncertainty, evidence refs, image digest, and deletion state are schema-backed.

## Current State

Partial schema direction exists in activity-domain and docs.

## Checklist

- [ ] Define visible categories.
- [ ] Define risk signals.
- [ ] Define OCR snippets with limits.
- [ ] Define redaction notes.
- [ ] Define confidence and uncertainty reasons.
- [ ] Define image digest and source refs.
- [ ] Define deletion status.

## Proof

- Contract tests for valid/invalid results.
- Rust protocol conversion tests.
