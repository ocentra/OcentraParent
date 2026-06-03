# 15 - Local Text LLM Execution Dry-Run Adapter

## Target State

Local text inference can run in dry-run for safety support with typed input,
bounded output, timeout, cancellation, and invalid-output rejection.

## Where We Are

Generation request/result/runner files exist in Rust service. The dry-run safety
adapter must bind those pieces to AI context and result parsing.

## Checklist

- [ ] Wire context-builder output to generation request.
- [ ] Add timeout and cancellation.
- [ ] Parse output into result candidate.
- [ ] Reject invalid output.
- [ ] Journal dry-run result.
- [ ] Keep enforcement disabled.

## Proof

- Local dry-run proof script.
- Timeout test.
- Invalid output rejection test.
- Dry-run policy integration test.
