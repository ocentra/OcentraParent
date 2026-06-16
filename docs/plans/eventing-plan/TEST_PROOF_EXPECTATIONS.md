# Test and Proof Expectations

Status: complete.

## Purpose
This file tracks the required execution flow. It does not store proof artifacts.

## Required flow
- [x] Code written.
- [x] Tests written.
- [x] Compile and validate passed.
- [x] Tests passed.
- [x] Full crate or package validation passed.
- [x] Proof collected in the designated local artifact path.
- [x] Proof pointer recorded outside the plan folder.

## Proof storage
Proof artifacts live in the designated local artifact path for the workpack or crate, not in this plan folder.

## Failure conditions
- Do not mark DONE or PR_READY until the code, tests, validation, and proof flow are complete.
- Do not store proof inventories inside the plan folder.
