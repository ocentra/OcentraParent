# Common Architecture Failures

Block these before a plan expands.

| Failure | Why bad | Correct pattern |
| --- | --- | --- |
| Domain imports sibling runtime behavior | Breaks owner boundary and makes tests lie. | Use typed command/event/request/read model. |
| Tracking/network/screen directly calls AI | Moves orchestration into wrong owner. | Evidence owner emits event; app/service orchestrator requests AI. |
| Portal state treated as runtime truth | UI can render fake or stale state. | Portal consumes service/read-model; proof checks runtime logs too. |
| Parent-domain wrapper treated as owner | Hides real domain owner and often violates re-export rules. | Route source to narrow owner package/crate. |
| Protocol domain owns product behavior | Protocol layer becomes dumping ground. | Protocol owns wire contract only. |
| Inline Rust tests counted as final proof | Public boundary not proved through crate API. | Move/mirror public behavior tests to crate `tests/`. |
| Empty test folders counted as coverage | Folder taxonomy is not executable proof. | Count only real tests and proof runners. |
| Proof script points at old package | Generates fake red/green. | Proof script targets current owner and real test path. |
| Playwright checks only screenshot/text | Does not prove runtime chain. | Verify UI plus log/event/read-model artifact. |
| Event bus duplicated per plan | Fragments replay, queue, diagnostics. | Reuse shared eventing surface where applicable. |
| Logging added as noisy print | Not queryable, not correlated, not proof. | Use controlled logger/run id/correlation id/redacted data. |
| Manual-required hides feasible proof | Lets Windows/Android/Linux gaps survive. | Use host feasibility matrix; reserve host-limited for real Apple/external gaps. |
