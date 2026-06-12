# Model And Runtime Candidate Strategy

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `Model And Runtime Candidate Strategy`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Goal

Keep model choice product-grade and testable. The plan must support current
local text model work while leaving room for better OCR, VLM, and embedding
models.

## Current Product Position

- llama.cpp/GGUF is the first product runtime direction already represented in
  current Rust runtime/config/status work.
- The current local text model lane can be used for typed-evidence reasoning,
  short explanations, category support, parent summaries, conflict summaries,
  and unknown/degraded explanations.
- The current local text model must not be treated as the OCR or VLM engine.
- OCR and VLM need separate worker lanes with their own quality proof.

## Candidate Evaluation Criteria

- Runs locally on target child-device hardware.
- Has known artifact format, version, checksum, and license.
- Supports the required task: text, OCR, VLM, embeddings, or classifier.
- Has bounded memory and CPU/GPU use.
- Can degrade honestly when unavailable.
- Produces output that can be schema-validated.
- Avoids remote calls by default.
- Has test fixtures and quality evaluation data.

## Immediate Stack

| Layer         | Initial direction                                                 |
| ------------- | ----------------------------------------------------------------- |
| Deterministic | Rules, parsers, catalogs, metadata, policy schedules              |
| Text LLM      | Existing local text model lane through Ocentra runtime contracts  |
| OCR           | Dedicated local OCR worker to be selected by screen plan proof    |
| VLM           | Small guided local VLM worker after OCR baseline                  |
| Embeddings    | Local semantic index after memory source-citation contract exists |
| Remote        | Parent-approved assistant only, disabled for child safety         |

## Proof Before Claim

- Model artifact integrity proof.
- Runtime status proof.
- Prompt/version proof.
- Output parser proof.
- Quality fixture proof.
- Performance/resource proof.
- Degraded/unavailable proof.
- UI screenshot proof for status and explanations.
