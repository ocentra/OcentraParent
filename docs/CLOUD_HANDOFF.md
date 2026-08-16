# Doc Enhance Cloud Handoff

This is the continuity handoff for the Ocentra Parent documentation-routing and
plan-depth work. It exists so a new Codex session can continue without asking the
user to repeat the full chat history.

## 0. Current checkout

- Repo: `E:\OcentraParent`
- Branch: `codex/doc-docs-gap-audit-pass2`
- Remote branch: `origin/codex/doc-docs-gap-audit-pass2`
- Last pushed commit before this handoff rewrite: `286ab41be`
- Current task: rewrite this handoff so it carries the real objective,
  decisions, mistakes, and next work.

## 1. Why we are doing this

The repo has a large docs tree, many feature docs, expectation docs, roadmap
files, implementation plans, workpacks, proof docs, and checkpoints. Future
agents were burning context by opening giant READMEs, broad folders, old
checkpoints, and unrelated workpacks. The user asked for a doc system where an
agent can read the least amount needed for a job without missing required
scope, tests, proof, or safety constraints.

The real goal is not "make docs shorter." The goal is a high-information-density
decision forest:

- Root `AGENTS.md` routes to role and work type.
- Agent flow docs route to plan, source boundary, validation, or release flow.
- Each plan has its own local `AGENTS.md` and indexes.
- A worker should read only the owning feature, owning plan, selected workpack,
  selected checklist/proof expectations, and touched source boundaries.
- A worker should not read all features, all expectations, all plans, all
  checkpoints, or the whole source tree.
- A worker should not skip tests/proof because the route was too vague.

The user repeatedly corrected that this is not a cosmetic docs job. It is a
systems-design job for future agent behavior: route precisely, avoid token
churn, avoid missing critical tests/proof, and avoid false "done" claims.

## 2. Original input that started this branch

The user had another ChatGPT generate three zip files:

- `ocentra-docs-v3-plan-normalized-pack.zip`
- `ocentra-docs-v3-replacements.zip`
- `ocentra-docs-v3-docs-folder.zip`

The user extracted them under `E:\OcentraParent\docs\TEMP` and explicitly said
not to blindly copy them into the real repo. The user wanted Codex to understand
what they were for, diff them carefully, and only apply what made sense.

Meaning of the three zips:

- Full pack: analysis reports, validation reports, manifest, apply
  instructions, and replacement folder.
- Replacement overlay: the actual files intended to be copied into repo root.
- Docs-folder-only zip: only a replacement `docs/` tree.

The user did not know why the other chat gave three folders. The answer is that
they represent different packaging levels for the same generated doc update.
The actionable one, if accepted, would be the replacement overlay, but the user
explicitly wanted review before touching the actual repo.

## 3. What the user wanted from those generated docs

The user wanted:

- Root `AGENTS.md` to be a short bootloader with pointers, not a giant rulebook.
- Each plan to have a local `AGENTS.md` or equivalent first-read router.
- The main agent file to contain a matrix that tells agents where to go based on
  role:
  - main/primary coordinator
  - dedicated worker lane
  - worktree lane
  - validation/proof worker
  - release/package worker
- Plan docs to form a tree/forest decision structure:
  - if doing X, read Y
  - if doing Z, read these exact workpacks/checklists/proof docs
  - otherwise stop and report missing route
- The language should be high-information-density English:
  - exact goal
  - scope
  - definitions
  - rules
  - constraints
  - deliverables
  - validation
  - failure conditions
- No broad "read everything" behavior.
- No vague "do tests" language.
- No generated stubs that look complete but contain no useful work.

## 4. Strong user rules from the chat

These are binding for continuation:

- Do not blindly copy generated docs into the real repo.
- Do not delete existing real plans just because a generated audit calls them
  duplicate.
- Do not delete either `app-plan` or `app-game-plan`.
- Do not collapse `app-plan` and `app-game-plan`; games are technically apps,
  but the product treats app control and game/addiction/screen-time control as
  separate policy surfaces.
- Do not write future implementation code recipes into docs.
- Docs may define expected outcomes, interfaces, shapes, boundaries, schema
  expectations, proof requirements, test names, and failure conditions.
- Docs must not spoon-feed exact implementation code unless a tiny snippet is
  strictly needed to define an artifact shape.
- Each plan should say what tests/proofs are expected in the eventual per-plan
  crate/package/domain test folder, even if that crate/package is being created
  by another Codex lane.
- Write docs first, verify all at end, do not churn validation repeatedly.
- A plan existing is not the same as a detailed plan.
- A short workpack file with generic text is not a real workpack.
- Future agents should be forced to ask:
  - did we write code?
  - did we write tests?
  - did the listed tests actually exist?
  - are there no fake mocks/fake green tests?
  - is proof attached?
  - does proof include logs/screenshots/artifacts where required?
  - is the done claim honest?

## 5. Test/proof expectation the user specifically wanted represented

The user gave a broad list of test/proof categories and wanted plans to route
agents to relevant ones by risk surface, not habit. The list included:

- unit, integration, e2e
- invariant, property-based, mutation, differential
- no mock or limited mock, beyond happy path, no fake green
- authN, authZ matrix, privilege escalation, token lifecycle, replay
- dedicated security tests
- request smuggling, desync, cache poisoning
- API fuzzing, schema fuzzing, GraphQL depth
- rate limit, abuse, brute force, DoS
- CORS, origin, headers, host, redirect, URL hijack
- header injection, request splitting, open redirect
- concurrency, race, idempotency, replay, ordering
- load, spike, soak, memory, FD, connection exhaustion
- migration, rollback, backward compatibility, schema drift
- contract, consumer-driven, version skew
- chaos, partial outage, slow dependency, retry storm
- clock skew, expiry boundary, DST
- prompt injection, hallucination regression, AI output invariant
- temperature sensitivity, safety boundary
- logging assertion, metrics sanity, tracing completeness
- alert firing, error budget burn
- flaky detection, mutation score, CI dependency kill
- canary and rollback validation
- human misuse, double submit, refresh abuse
- monitoring and alert proof

The point is not that every plan runs every test type. The point is every plan
must name the relevant test/proof expectations so future agents cannot claim
done with only happy-path validation.

## 6. Antigravity/session changes the user reported

While Codex was interrupted/token-burned, Antigravity made repo changes and
reported these:

- Deleted 11 "junk" plan folders that had been blindly generated and were
  redundant with the original core plan structure.
- Moved 20 roadmap milestone files out of `docs/expectations/` into
  `docs/roadmaps/`, so agents do not confuse milestones with executable plans.
- Updated markdown links after that move.
- Added a rule in `docs/agent/PRODUCT_DOC_FLOW.md` forbidding roadmaps from
  being treated as engineering plans.
- Moved 21 detailed root `docs/*.md` schema/capability/settings files into the
  owning plan `workpacks/` directories:
  - browser files -> `browser-plan`
  - app/game files -> `app-game-plan`
  - tracking files -> `tracking-plan`
  - network files -> `network-plan`
  - screen files -> `screen-plan`
- Moved broad UI/AI stray files:
  - `data and AI Ui plan.md` -> `data-custody-storage-plan/workpacks/`
  - `full-platform-portal-ai-execution-plan.md` -> `ai-plan/workpacks/`
  - `policy Ui fix.md`, `portal and account Ui fix.md`,
    `manage UI proof checklist.md` -> `portal-ux-household-surfaces-plan/workpacks/`
- Renamed `evidence-store-query-plan` to `data-custody-storage-plan` because the
  user wants a full data custody plan, not merely evidence query/storage.
- Antigravity accidentally deleted `app-plan`, then restored it from Git. Any
  uncommitted local changes previously made inside `app-plan` were lost.

Do not blindly trust the quality of those moved files. The user said many
generated workpacks are stub-like and may be useless unless expanded.

## 7. Antigravity feature/plan audit summary reported by user

Antigravity identified 18 feature docs under `docs/features`:

- app-game-control
- app-install-purchase-approval
- browser-web-control
- child-agent-local-service
- enforcement-integrity-tamper
- evidence-store-query
- family-setup-device-roles
- local-ai-safety-evaluator
- location-geofence-device-status
- network-domain-control
- parent-assistant-actions
- policy-schedules-approvals
- production-distribution-support
- remote-lan-mobile-platforms
- reports-notifications-sync
- screen-evidence-analysis
- screen-visibility-live-view
- social-video-control

Antigravity also listed the original 13 hand-written core plans:

- `ai-plan`
- `app-game-plan`
- `app-plan`
- `browser-plan`
- `eventing-plan`
- `lan-plan`
- `network-plan`
- `parent-desktop-runtime-package-plan`
- `portal-ux-household-surfaces-plan`
- `screen-ai-pipeline-plan`
- `screen-plan`
- `tracking-plan`
- `v0-8-enforcement-control-plan`

Antigravity concluded the 11 generated extra feature-plan folders were blind
duplicates because all 18 feature docs were at least claimed by one of those 13
core plans. User accepted the broad warning against duplicate folders but later
corrected that "claimed by a plan" is not the same as "detailed enough."

## 8. Specific gaps the user highlighted after that audit

The user wants a fresh, informed audit of expectations/features/root docs to
decide what plans are actually deep enough.

Known or suspected gaps from the chat:

- `data-custody-storage-plan` needs a full-blown detailed plan:
  - data export/import
  - parent-owned cloud storage
  - Google Drive or similar provider connection
  - config/data custody for "everything", not just evidence
  - encryption so exported/stored data is not readable JSON
  - privacy promise: Ocentra should not be a data-hungry free app that steals
    or legally exploits family data
- `remote-access-plan` is needed:
  - remote desktop/control is not the same as screen capture/screen evidence
  - screen plan owns screen visibility/evidence
  - remote access needs its own plan for relay/control/security/consent
  - there are scattered RustDesk/remote desktop comparison docs that should be
    incorporated or routed
- Install/setup/provisioning is under-planned:
  - who installs what
  - family.ocentra page journey
  - parent portal login first or installer first
  - child device install
  - pairing
  - security of install and pairing
  - UI/UX for install, recovery, and device role setup
- Account/login/user model needs a plan or explicit ownership:
  - registration
  - household roles
  - login/session/token lifecycle
  - likely ties into family site, parent portal, child service, and billing
- Payment/subscription needs a plan:
  - user referenced `E:\ocentra-games` where Cloudflare/Firebase/Stripe exist
  - parent product may not copy the game flow exactly
  - decide Cloudflare-only vs Cloudflare + Firebase vs other provider
  - decide direct Stripe or Worker-mediated Stripe
- Policy UI/control-plane may need its own plan or clear ownership:
  - portal UI sets policy
  - policy travels to child Rust service
  - domain policy compilers affect app/game/browser/tracking/network/screen
  - avoid one giant ad hoc policy blob
  - define effect schema / protocol / ack / audit route
- `full-platform-portal-ai-execution-plan.md` likely belongs under `ai-plan`.
- Portal/account UI fix docs may belong under portal UX or a policy/account plan,
  but must not remain root soup.

## 9. Important domain distinction: app vs app-game

The user explicitly corrected this:

- Apps and games are technically both applications at OS level.
- The product treats them differently because parental-control concern differs.
- YouTube/social/productivity/ordinary apps and Roblox/game/addiction/screen-time
  surfaces have different safety/policy semantics.
- Keep both `app-plan` and `app-game-plan`.
- Do not repeat the mistake of deleting or merging one into the other.

## 10. What Codex did in this branch before this handoff

Important: the previous Codex work was not fully satisfactory to the user.
This handoff must be honest about that.

Observed branch commit history:

- `edf7f2e01` docs: normalize agent routing and plan workpacks
- `a706796b7` docs: add first-pass feature-plan audit and tighten proof matrix
- `d3652b238` docs: refresh plan feature ownership audit and gap matrix
- `784b925ef` docs: refresh plan feature ownership audit and gap matrix
- `286ab41be` docs: add Codex cloud handoff continuity and continuation steps

What appears to be present now:

- Root `AGENTS.md` is a short router/bootloader.
- `docs/agent/` contains routing flow docs.
- `docs/agent/TEST_PROOF_DECISION_MATRIX.md` exists.
- `docs/FEATURE_ROUTE_INDEX.md`, `docs/PLAN_INDEX.md`,
  `docs/PLAN_HEALTH_INDEX.md`, and related indexes exist.
- `docs/plans` currently has 19 plan folders.
- New first-pass plans exist for data custody, account identity, setup/install,
  payment/subscription, policy control plane, and remote access.

But do not assume these are complete. The user is specifically unhappy with
stubby workpacks and handwave docs. The next agent must inspect plan quality,
not just file count.

## 11. Branch confusion and current single-branch decision

At one point there were several related branches:

- `codex/doc-docs-gap-audit-pass2`
- `codex/doc-routing-gaps-pass1`
- `codex/doc-update-agent-routing-v3`

The user said this was confusing and wanted only one branch with all work.

Current intended branch:

- Use `codex/doc-docs-gap-audit-pass2`.
- Do not create a sibling branch for the same objective.
- Do not continue work on deleted/superseded branches.

## 12. Hook/guard issue that happened

The user pasted a hub warning:

- Current `main` no longer has `scripts/dev/worktree-lanes.mjs`.
- Hooks should call Enforcer coordination through:
  - `scripts/enforcer/run-ocentra-enforcer.mjs coordination hub:guard`
- `git pull` on a feature branch may only pull that branch's upstream; use
  `git merge origin/main` or rebase to bring main into the branch.
- Avoid broad `npm run format`; for docs-only use targeted prettier if needed.

Codex initially did not fix the hook and suggested bypass. User challenged that.

Actual issue found:

- `.git/hooks/pre-commit` was stale and still called:
  - `scripts/dev/worktree-lanes.mjs`
  - `scripts/dev/hub-mailbox.mjs`

Fix performed:

- Ran `npm run hooks:install`.
- Confirmed the hook file now calls Enforcer coordination directly.
- Ran:
  - `git fetch origin`
  - `git merge origin/main`
  - `npm run ledger:install`
  - `npm run hub:guard`
- `hub:guard` passed.
- Branch was pushed to `origin/codex/doc-docs-gap-audit-pass2` at commit
  `286ab41be`.

## 13. Current file state warning

At the time this handoff was rewritten, `docs/CLOUD_HANDOFF.md` had staged and
unstaged edits from earlier failed handoff attempts (`MM` in `git status`).
Before committing, run:

```powershell
git status --short --branch
git diff -- docs/CLOUD_HANDOFF.md
git diff --cached -- docs/CLOUD_HANDOFF.md
```

Then stage this final handoff version explicitly:

```powershell
git add docs/CLOUD_HANDOFF.md
```

## 14. What the next agent must do next

Do not start by writing new plans blindly. First confirm current reality with
the smallest useful read path.

Startup:

```powershell
cd E:\OcentraParent
git fetch origin
git checkout codex/doc-docs-gap-audit-pass2
git merge origin/main
npm run ledger:install
npm run hub:guard
git status --short --branch
```

Read path:

1. `AGENTS.md`
2. `docs/agent/TASK_ROUTER.md`
3. `docs/PLAN_INDEX.md`
4. `docs/FEATURE_ROUTE_INDEX.md`
5. `docs/PLAN_HEALTH_INDEX.md`
6. `docs/PLAN_AUDIT_PASS1.md`
7. `docs/features/*.md`
8. `docs/expectations/*.md`
9. root-level `docs/*.md` that are product, feature, capability, roadmap, or
   orphan-planning docs
10. `docs/roadmaps/*.md` only as milestone/context inputs, not executable plans

Then audit only the relevant owning plan docs/workpack indexes.

For this specific audit task, reading all feature docs and expectation docs is
allowed because the job is to prove feature/expectation coverage. This does not
change normal worker routing. Normal feature workers must still use the smallest
route and must not browse the whole docs tree.

## 14A. What to compare against main

The next agent must distinguish:

- what existed on `origin/main`
- what this branch added or reorganized
- what was moved from root docs into plan workpacks
- what Antigravity changed outside this Codex run
- what is only a first-pass generated plan
- what is actually a detailed plan/workpack ready for future implementation

Use `origin/main` as the baseline for "what existed before this docs branch".
Do not assume a new folder is justified just because it exists on this branch.
Do not assume a missing folder means the feature has no plan; many features are
owned by an existing plan.

Recommended baseline commands:

```powershell
git diff --stat origin/main..HEAD -- docs AGENTS.md
git diff --name-status origin/main..HEAD -- docs AGENTS.md
git show origin/main:docs/FEATURE_ROUTE_INDEX.md
```

If a file does not exist on `origin/main`, treat that as evidence that this
branch introduced it and verify whether it is a real route/workpack or only a
generated placeholder.

## 14B. Current tracking continuation branch must be inspected

There is a parallel branch:

- `origin/codex/tracking-plan-full-continuation-a`

That branch is doing real tracking-plan implementation and organization work in
parallel. It is not expected to write all docs. This docs branch must provide
the contract that later tracking/domain/crate agents can read and apply.

High-level files seen in that branch include:

- `apps/portal` tracking UI/proof files
- `crates/agent-core/src/tracking/*`
- `crates/agent-protocol/src/tracking/*`
- `crates/agent-service/src/websocket/tracking_retention_settings_write.rs`
- `packages/agent-protocol-domain`
- `docs/plans/tracking-plan/README.md`
- `docs/plans/tracking-plan/implementation-checklist.md`
- `docs/plans/tracking-plan/event-driven-runtime-test-matrix.md`
- new tracking workpacks around event contracts, config command flow,
  detection cascade, journal replay/projection, notification/escalation, and
  portal read-model proof

Before changing tracking-plan docs, compare against that branch with a targeted
diff. Do not overwrite or contradict tracking work that is already happening in
parallel.

Useful commands:

```powershell
git fetch origin
git diff --stat origin/main..origin/codex/tracking-plan-full-continuation-a -- docs/plans/tracking-plan crates packages apps
git diff --name-status origin/main..origin/codex/tracking-plan-full-continuation-a -- docs/plans/tracking-plan crates packages apps
```

If tracking code/domain/crate work already covers a requirement, docs should
route to it and state remaining proof requirements. If the parallel branch
implements something without docs coverage, this docs branch should document the
expectation and proof gate so future agents do not miss it after merge.

## 15. Required audit output

Produce or update a clear matrix with rows like:

- Feature/promise
- Owning plan(s)
- Evidence found in feature docs / expectations / roadmap
- Workpack depth status:
  - detailed
  - shallow
  - stub
  - missing
- Required tests/proofs
- Missing route/doc/workpack
- Recommended action
- Whether to create/expand plan or route to existing plan

The key distinction:

- "Plan claims this feature" is not enough.
- "Plan has detailed executable workpacks, boundaries, tests, and proof
  expectations" is what matters.

## 16. Specific areas to verify first

1. Data custody/storage:
   - Must be a full plan if not already deep.
   - Cover parent-owned storage, cloud provider connection, export/import,
     encryption, config custody, evidence custody, no data exploitation.

2. Setup/install/provisioning:
   - Must cover family site, registration, login/install journey, child
     install, pairing, recovery, security, platform packaging route.

3. Account/identity/family:
   - Must cover login/session/roles/household/device ownership.

4. Payment/subscription:
   - Must compare against `E:\ocentra-games` patterns before deciding provider.
   - Consider Cloudflare, Firebase, Stripe, Worker-mediated vs direct.

5. Remote access:
   - Must be separate from screen capture.
   - Include remote desktop/control, relay, consent, authz, abuse prevention,
     audit/proof.

6. Policy control plane:
   - Must define parent portal -> policy schema/effect schema -> child service
     delivery -> ack/audit -> domain policy compiler route.

7. Existing 13 core plans:
   - Check whether they are truly detailed or only have generated stubs.
   - Do not delete `app-plan` or `app-game-plan`.

## 17. Rules for expanding docs

When expanding docs:

- Use high-information-density plain English.
- Prefer decision tables and explicit routes over long prose.
- Define expected outcomes and proof, not implementation code.
- Mention expected test names/intents, not exact test implementation.
- Add failure conditions.
- Add no-read boundaries.
- Add "stop and ask/update route" behavior when ownership is unclear.

Avoid:

- giant READMEs as first-read docs
- generic "write tests" text
- "happy path only" validation
- fake completion language
- duplicate plan folders for a feature already owned by a real plan unless the
  existing plan is proven structurally wrong

## 17A. The expected doc quality bar

The output of this work should let a future agent land in the repo and make a
correct narrow decision without prior chat context.

For every plan touched, ask:

- Does local `AGENTS.md` tell the agent what to read first?
- Does it tell the agent what not to read?
- Does `WORKPACK_INDEX.md` route real tasks to specific workpacks?
- Are workpacks detailed enough to guide scope without writing code for the
  future agent?
- Are old docs, root stray docs, and moved docs linked or routed, not lost?
- Are missing scopes called gaps instead of hidden behind "done" wording?
- Are tests and proof requirements tied to the risk surface?
- Are evidence artifacts named: logs, screenshots, traces, command output,
  journal rows, event records, snapshots, or alert proofs where relevant?
- Does the plan say where future per-plan/domain/crate tests should live once
  the parallel repo-organization work creates those locations?
- Can a future worker execute one task without reading unrelated feature docs,
  old checkpoints, or every workpack?

If the answer is no, fix the routing/workpack/proof doc before claiming the plan
is ready.

The desired structure is a decision forest:

- root router chooses lane and work type
- agent flow chooses plan/source/validation/release route
- plan router chooses feature/workpack/checklist/proof
- workpack names expected outcomes and evidence
- validation matrix selects tests by risk
- done flow checks proof before status claims

This is how token reduction is supposed to work: not by deleting important
context, but by putting the right context behind the right route and making the
stop/read boundaries explicit.

## 18. Definition of done for this docs objective

This objective is done only when:

- One branch contains the final docs.
- Root routing and plan routing tell agents exactly where to go.
- Every feature/expectation promise has an owning plan or an explicit gap.
- Shallow/stub workpacks are marked as such or expanded.
- New/first-pass plans clearly say they require research/implementation closure.
- Test/proof expectations exist per relevant plan.
- No doc claims implementation readiness without evidence.
- `git status` is clean after commit.
- Branch is pushed.

## 19. One-message prompt for the next Codex session

Continue in `E:\OcentraParent` on branch `codex/doc-docs-gap-audit-pass2`.
Read `docs/CLOUD_HANDOFF.md` first. The job is not to blindly create or delete
plans. The job is to finish the evidence-backed doc routing and plan-depth audit
so future agents read the least necessary docs but do not miss scope, tests, or
proof. Preserve both `app-plan` and `app-game-plan`. Treat folder existence as
not enough. Verify whether each feature/expectation/root stray doc has detailed
ownership and executable workpack depth. Expand only necessary docs using
high-information-density language, expected outcomes, boundaries, tests/proof,
and failure conditions. Do not write code recipes. Keep one branch and push it.
