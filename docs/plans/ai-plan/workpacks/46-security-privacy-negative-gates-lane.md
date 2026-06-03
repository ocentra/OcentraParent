# 46 - Security Privacy Negative Gates Lane

## Target State

Negative tests prove AI cannot violate custody, direct-scan, direct-enforcement,
remote, screenshot, memory, graph, or prompt-minimization boundaries.

## Where We Are

Expectations define the boundaries. The test suite must enforce them before
runtime behavior expands.

## Checklist

- [ ] AI no direct OS scan test.
- [ ] AI no direct browser/network/screen scan test.
- [ ] AI no direct enforcement test.
- [ ] Remote disabled-by-default test.
- [ ] Raw screenshot API guard test.
- [ ] Unsourced memory/graph rejected tests.
- [ ] Prompt minimization test.
- [ ] Custody label guard test.

## Proof

- Security test suite output.
- No test doubles.
- Validation gate includes negative tests.
