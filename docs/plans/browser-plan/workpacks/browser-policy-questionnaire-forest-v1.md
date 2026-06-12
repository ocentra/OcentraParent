<!-- agent-capsule -->

> Agent Capsule
> Doc: Browser Policy Questionnaire Forest v1
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Browser Policy Questionnaire Forest v1

Raw source: 1057 browser-control settings reduced into numbered questionnaire groups.
Scope is already handled outside by Family / Per Device switch.
AI is separate service, not inside browser policy.

---

# 0. Global display states

## 0.1 Always start with

```text
1.1 Should browser policy be active?
```

This is the root gate.

---

## 0.2 Root answers

```text
Off
On
Paused until time
Emergency allow
Emergency block
```

---

# 1. Root forest

## 1.1 Should browser policy be active?

### If answer = Off

Show:

```text
1.1 Should browser policy be active?
14.1 What should parent see?            only if reports still enabled globally
14.2 What report detail level?          only if reports still enabled globally
18.1 How much audit history should be kept?
18.2 What must be audited?
```

Never show:

```text
1.2 What should controlled browser activity do?
1.3 Should test/dry-run mode be used?
2 Browser Coverage
3 Managed Browser
4 Unmanaged Browser Handling
5 Rule Targets
6 Rule Actions
7 Search Rules
8 Video Rules
9 Downloads
10 Schedule
11 Time Limits
12 Approvals
13 Evidence / Privacy
15 Offline / Capability Failure
16 Setup, except broken setup status
17 Data, except global data page
```

Reason:

```text
Browser policy is off, so there is no policy to configure.
```

---

### If answer = Paused until time

Show:

```text
1.1 Should browser policy be active?
10.2 Should browser policy support temporary overrides?
14.1 What should parent see?
14.2 What report detail level?
14.3 When should parent be notified?
18.1 How much audit history should be kept?
18.2 What must be audited?
```

Never show as editable:

```text
2 Browser Coverage
3 Managed Browser
4 Unmanaged Browser Handling
5 Rule Targets
6 Rule Actions
7 Search Rules
8 Video Rules
9 Downloads
11 Time Limits
12 Approvals
13 Evidence / Privacy
15 Offline / Capability Failure
16 Setup
```

Show read-only summary only:

```text
Current policy summary
Paused-until time
Who paused it
Audit event
```

Reason:

```text
Policy exists but is temporarily suspended.
```

---

### If answer = Emergency allow

Show:

```text
1.1 Should browser policy be active?
10.2 Should browser policy support temporary overrides?
14.1 What should parent see?
14.3 When should parent be notified?
18.1 How much audit history should be kept?
18.2 What must be audited?
```

Never show as active config:

```text
2 Browser Coverage
3 Managed Browser
4 Unmanaged Browser Handling
5 Rule Targets
6 Rule Actions
7 Search Rules
8 Video Rules
9 Downloads
11 Time Limits
12 Approvals
13 Evidence / Privacy
15 Offline / Capability Failure
16 Setup
```

Reason:

```text
Emergency allow overrides normal browser policy.
```

---

### If answer = Emergency block

Show:

```text
1.1 Should browser policy be active?
10.2 Should browser policy support temporary overrides?
12.1 What requires parent approval?      only emergency unlock / exception
12.2 What happens if parent does not answer?
14.1 What should parent see?
14.3 When should parent be notified?
18.1 How much audit history should be kept?
18.2 What must be audited?
```

Never show as active config:

```text
2 Browser Coverage
3 Managed Browser
4 Unmanaged Browser Handling
5 Rule Targets
6 Rule Actions
7 Search Rules
8 Video Rules
9 Downloads
11 Time Limits
13 Evidence / Privacy
15 Offline / Capability Failure
16 Setup
```

Reason:

```text
Emergency block overrides normal browser policy.
```

---

### If answer = On

Show next:

```text
1.2 What should controlled browser activity do?
1.3 Should test/dry-run mode be used?
```

Then branch by answers.

---

# 2. Activity behavior forest

## 1.2 What should controlled browser activity do?

Answers:

```text
Observe
Warn
Ask parent
Limit
Block
```

This is multi-choice.

---

## 2.1 If only Observe is selected

Show:

```text
2.1 Which browser coverage level should be used?
2.2 How aggressively should browsers be discovered?
2.3 What should happen when a new browser is found?
5.1 What should browser rules target?
5.2 What should happen to unknown pages?
7.1 Should search be controlled?             observe choices only
8.1 Should video/web media be controlled?    observe choices only
9.1 Should downloads be controlled?          observe choices only
13.1 What browser evidence may be collected?
13.2 What evidence must never be collected?
14.1 What should parent see?
14.2 What report detail level?
14.3 When should parent be notified?
15.1 What should happen if child device or local agent is offline?
15.2 What should happen if browser/platform capability is unsupported?
16.1 Should setup/provisioning controls be shown?
17.1 How long should browser-control data be kept?
17.2 What data can parent export/delete?
18.1 How much audit history should be kept?
18.2 What must be audited?
```

Never show:

```text
3.1 Should a managed browser be required?       unless exact URL/search evidence is selected
3.2 Managed-browser setup behavior              unless managed browser required
3.3 Managed-browser locked features             unless managed browser required
4.1 Unmanaged browser hard action               except Observe / Notify
4.2 Unmanaged exceptions                        not needed
4.3 Bypass browsers blocked                     not needed
6.1 Rule actions                                only Observe/log allowed
6.2 Per-target action matrix                    not needed
10.1 Schedule                                   optional only if observe schedule needed
11 Time Limits                                  no hard limits
12 Approvals                                    no approval-before-continue
15.3 Exact proof missing hard fallback           no hard fallback
```

Reason:

```text
Observe means no child interruption.
```

---

## 2.2 If Warn is selected

Show:

```text
2.1 Which browser coverage level should be used?
2.2 How aggressively should browsers be discovered?
2.3 What should happen when a new browser is found?
4.1 What should happen if an unmanaged browser is used?      Warn / Notify choices
5.1 What should browser rules target?
5.2 What should happen to unknown pages?
6.1 Which actions can a rule perform?                       Warn + Observe
7 Search Rules                                               if Search selected in 5.1
8 Video Rules                                                if Video selected in 5.1
9 Downloads                                                  if Downloads selected in 5.1
10.1 When should browser policy apply?
10.2 Should browser policy support temporary overrides?
13 Evidence / Privacy
14 Reports
15 Offline / Capability Failure
16 Setup
17 Data
18 Audit
```

Show only if escalation enabled:

```text
12 Approvals
11 Time Limits
6.2 Per-target action matrix
```

Never show:

```text
Hard block settings
Close browser settings
Require managed browser for all activity
Emergency unlock approval flow
```

Reason:

```text
Warn is soft enforcement unless escalation is selected.
```

---

## 2.3 If Ask parent is selected

Show:

```text
2 Browser Coverage
2.2 Browser Discovery
2.3 New Browser Found
3 Managed Browser                         if exact/browser proof needed
4 Unmanaged Browser Handling
5 Rule Targets
6 Rule Actions
7 Search Rules                            if Search selected
8 Video Rules                             if Video selected
9 Downloads                               if Downloads selected
10 Schedule
12 Approvals
13 Evidence / Privacy
14 Reports
15 Offline / Capability Failure
16 Setup
17 Data
18 Audit
```

Always show in this branch:

```text
12.1 What requires parent approval?
12.2 What happens if parent does not answer?
12.3 How long does approval last?
```

Never show:

```text
Limit-only controls unless Limit also selected
Block-only controls unless Block also selected
```

Reason:

```text
Ask parent activates approval behavior.
```

---

## 2.4 If Limit is selected

Show:

```text
2 Browser Coverage
3 Managed Browser                         if exact enforcement needs proof
4 Unmanaged Browser Handling              if unmanaged browsers can bypass limits
5 Rule Targets
6 Rule Actions
7 Search Rules                            if Search selected
8 Video Rules                             if Video selected
9 Downloads                               if Downloads selected
10 Schedule
11 Time Limits
12 Approvals                              only if extension/override requires parent approval
13 Evidence / Privacy
14 Reports
15 Offline / Capability Failure
16 Setup
17 Data
18 Audit
```

Always show:

```text
11.1 Should browser activity have time limits?
11.2 What type of time limit?
```

Never show:

```text
Download blocking choices unless Downloads selected
Search blocking choices unless Search selected
Video blocking choices unless Video selected
```

Reason:

```text
Limit requires schedule/time-budget config.
```

---

## 2.5 If Block is selected

Show:

```text
2 Browser Coverage
2.2 Browser Discovery
2.3 New Browser Found
3 Managed Browser
4 Unmanaged Browser Handling
5 Rule Targets
6 Rule Actions
7 Search Rules                            if Search selected
8 Video Rules                             if Video selected
9 Downloads                               if Downloads selected
10 Schedule
11 Time Limits                            if Limit also selected
12 Approvals                              if Ask parent also selected or override allowed
13 Evidence / Privacy
14 Reports
15 Offline / Capability Failure
16 Setup
17 Data
18 Audit
```

Always show:

```text
5.3 What should happen when exact evidence is unavailable?
6.1 Which actions can a rule perform?
15.2 What should happen if browser/platform capability is unsupported?
15.3 What should happen if exact proof is missing?
```

Reason:

```text
Block requires fallback and capability handling.
```

---

# 3. Dry-run forest

## 1.3 Should test/dry-run mode be used?

### If No

Continue normally.

### If Yes, simulate decisions only

Show:

```text
2 Browser Coverage
2.2 Browser Discovery
5 Rule Targets
6 Rule Actions
13 Evidence / Privacy
14 Reports
18 Audit
```

Never show active enforcement:

```text
4.1 Close / block unmanaged browser
9.3 Download block
11 hard time limits
12 approval-before-continue
15 block-until-fixed
```

Reason:

```text
Dry-run evaluates but does not interrupt.
```

### If Yes, simulate and report what would happen

Show additionally:

```text
14.1 What should parent see?
14.2 What report detail level?
14.3 When should parent be notified?
```

Never show:

```text
Actual block/close/limit actions as enabled runtime behavior
```

---

# 4. Browser coverage forest

## 2.1 Which browser coverage level should be used?

### Common browsers

Show:

```text
2.2 Basic or Standard discovery
3 Managed Browser                 optional
4 Unmanaged Browser Handling      only if managed browser required
```

Never show:

```text
4.3 strict bypass browsers
Tor/private/portable/renamed/unknown browser controls
```

---

### All known browsers

Show:

```text
2.2 Standard discovery
3 Managed Browser
4.1 Unmanaged Browser Handling
4.2 Unmanaged browser exceptions
```

Never show unless Custom/Strict:

```text
4.3 strict bypass browsers
Executable hash/path/signature technical checks
```

---

### Strict anti-bypass coverage

Show:

```text
2.2 Strict discovery
3 Managed Browser
4.1 Unmanaged Browser Handling
4.2 Unmanaged browser exceptions
4.3 Which bypass browsers should be blocked or treated specially?
15 Offline / Capability Failure
16 Setup
```

Never hide:

```text
15.2 Unsupported capability fallback
15.3 Missing exact proof fallback
```

---

### Custom

Show:

```text
Full browser checklist
Full discovery checklist
4.2 Exceptions
4.3 Bypass list
15 Capability fallback
```

---

# 5. Managed browser forest

## 3.1 Should a managed browser be required?

### No, any covered browser is allowed

Show:

```text
2 Browser Coverage
5 Rule Targets
6 Rule Actions
13 Evidence / Privacy
14 Reports
```

Never show:

```text
3.2 Managed-browser setup behavior
3.3 Managed-browser locked features
4.1 Close and open managed browser
4.2 Managed-browser exceptions
```

---

### Prefer managed browser

Show:

```text
3.2 Managed-browser setup behavior
14 Reports
16 Setup
```

Show only if strict actions selected:

```text
4 Unmanaged Browser Handling
15 Capability Failure
```

---

### Require managed browser for exact URL/search rules

Show:

```text
3.2 Managed-browser setup behavior
3.3 Managed-browser locked features
5.3 What should happen when exact evidence is unavailable?
7 Search Rules                       if Search selected
13 Evidence / Privacy
15.3 Exact proof missing
16 Setup
```

Never show exact URL/search enforcement unless:

```text
13.1 allows Exact URL or Search term evidence
```

---

### Require managed browser for all browser activity

Show:

```text
3.2 Managed-browser setup behavior
3.3 Managed-browser locked features
4 Unmanaged Browser Handling
15 Offline / Capability Failure
16 Setup
```

Never hide unmanaged handling:

```text
4.1 must be visible
```

---

# 6. Unmanaged browser forest

## 4.1 What should happen if an unmanaged browser is used?

### Allow

Show:

```text
4.2 Exceptions only if Custom wanted
14 Reports optional
```

Never show:

```text
4.3 Bypass blocking
Close unmanaged browser
Block unmanaged browser
Ask parent before continuing
Grace period before action
```

---

### Observe

Show:

```text
13 Evidence / Privacy
14 Reports
14.3 Notifications
```

Never show:

```text
Close
Block
Ask before continuing
```

---

### Warn child

Show:

```text
14 Reports
14.3 Notifications
```

Show only if escalation:

```text
12 Approvals
6 Rule Actions
```

Never show:

```text
Close/block unless escalation selected
```

---

### Notify parent

Show:

```text
14.3 When should parent be notified?
14 Reports
```

Never show:

```text
Close/block/ask settings unless another action selected
```

---

### Ask parent before continuing

Show:

```text
12.1 What requires parent approval?
12.2 What happens if parent does not answer?
12.3 How long does approval last?
14 Reports
```

---

### Close browser

Show:

```text
15 Capability Failure
14 Reports
18 Audit
```

Never show:

```text
Managed browser launch settings unless close-and-open-managed selected
```

---

### Close browser and open managed browser

Show:

```text
3.2 Managed-browser setup behavior
16 Setup
15 Capability Failure
14 Reports
18 Audit
```

---

### Block launch

Show:

```text
4.3 Which bypass browsers should be blocked or treated specially?
15 Capability Failure
14 Reports
18 Audit
```

---

# 7. Rule target forest

## 5.1 What should browser rules target?

Only show child sections for selected targets.

### If Exact URL selected

Show:

```text
3.1 Managed browser requirement
5.3 Exact evidence unavailable
6 Rule Actions
13.1 Evidence: Exact URL
13.2 Evidence restrictions
15.3 Exact proof missing
```

Never allow exact URL action if:

```text
13.2 says No exact URL unless required
AND no managed/proof source exists
```

---

### If Domain selected

Show:

```text
6 Rule Actions
13.1 Evidence: Domain
14 Reports
```

Do not require:

```text
Managed browser
Exact URL proof
```

---

### If Category selected

Show:

```text
5.2 Unknown pages
6 Rule Actions
A1 Classification service              only if category needs classification
A3 AI/classification unavailable        only if classification enabled
```

Never show AI controls inside browser policy card.

Show only a service dependency note:

```text
Uses classification service if enabled.
```

---

### If Search terms selected

Show:

```text
7.1 Should search be controlled?
7.2 What search evidence is allowed?
3.1 Managed browser requirement         if exact search rules needed
13 Evidence / Privacy
15.3 Exact proof missing
```

---

### If Video selected

Show:

```text
8.1 Should video/web media be controlled?
8.2 What video targets are controlled?
11 Time Limits                         if Limit selected
13 Evidence / Privacy
```

---

### If Downloads selected

Show:

```text
9.1 Should downloads be controlled?
9.2 What download evidence is allowed?
9.3 What download actions are allowed?
12 Approvals                           if Ask parent selected
13 Evidence / Privacy
14 Reports
```

---

### If Browser session / app time selected

Show:

```text
11.1 Should browser activity have time limits?
11.2 What type of time limit?
14 Reports
```

---

### If Unknown web activity selected

Show:

```text
5.2 What should happen to unknown pages?
5.3 What should happen when exact evidence is unavailable?
12 Approvals                           if Ask parent selected
15 Capability Failure
```

---

# 8. Rule action forest

## 6.1 Which actions can a rule perform?

### If only Allow + Observe

Never show:

```text
12 Approvals
11 Time Limits
Block fallback
Close browser fallback
Redirect settings
```

---

### If Warn selected

Show:

```text
14 Reports
14.3 Notifications
```

Show escalation only if selected:

```text
12 Approvals
Block after repeated warnings
```

---

### If Ask parent selected

Show:

```text
12 Approvals
14.3 Notifications
```

Never show unanswered behavior unless:

```text
12.1 has at least one approval trigger
```

---

### If Limit selected

Show:

```text
10 Schedule
11 Time Limits
12 Approvals only if time extension allowed
```

---

### If Block selected

Show:

```text
5.3 Exact evidence unavailable
12 Approvals only if override allowed
15 Capability Failure
18 Audit
```

---

### If Redirect selected

Show:

```text
Safe redirect page target
Fallback if redirect fails
14 Reports
18 Audit
```

Never show if:

```text
1.2 = Observe only
1.3 = Dry-run only
```

---

### If Close browser selected

Show:

```text
3 Managed Browser if close-and-open-managed is used
15 Capability Failure
18 Audit
```

Never show if:

```text
1.2 does not include Block
```

---

## 6.2 Should rule actions be chosen per target type?

Show only if:

```text
5.1 has 2 or more selected targets
AND 6.1 has 2 or more selected actions
```

Never show if:

```text
Only one target selected
OR only one action selected
```

---

# 9. Search forest

## 7.1 Should search be controlled?

Show only if:

```text
5.1 includes Search terms or Safe search
```

Never show if:

```text
Search is not selected in 5.1
```

### If No

Never show:

```text
7.2 Search evidence
Search approval
Search block
Search report detail
```

### If Observe search only

Show:

```text
7.2 Search evidence
13 Evidence / Privacy
14 Reports
```

Never show:

```text
Search blocking
Search approval before continue
```

### If Enforce safe search

Show:

```text
3.1 Managed/browser capability if needed
15 Capability Failure
14 Reports
```

### If Warn / Ask / Block search terms

Show:

```text
7.2 Search evidence
6 Rule Actions
12 Approvals if Ask selected
13 Evidence / Privacy
15.3 Exact proof missing
```

---

# 10. Video forest

## 8.1 Should video/web media be controlled?

Show only if:

```text
5.1 includes Video platforms or Video channels
```

Never show if:

```text
Video not selected in 5.1
```

### If No

Never show:

```text
8.2 Video targets
Video time quotas
Video reports
Video block settings
```

### If Observe only

Show:

```text
8.2 What video targets are controlled?
13 Evidence / Privacy
14 Reports
```

Never show:

```text
Video block
Video time limit
Video approval
```

### If Limit

Show:

```text
8.2 Video targets
10 Schedule
11 Time Limits
14 Reports
```

### If Warn / Ask / Block

Show:

```text
8.2 Video targets
6 Rule Actions
12 Approvals if Ask selected
13 Evidence / Privacy
14 Reports
15 Capability Failure
```

---

# 11. Download forest

## 9.1 Should downloads be controlled?

Show only if:

```text
5.1 includes Downloads
```

Never show if:

```text
Downloads not selected in 5.1
```

### If Ignore

Never show:

```text
9.2 Download evidence
9.3 Download actions
Download approvals
Download reports
Download notifications
```

### If Observe

Show:

```text
9.2 Download evidence
13 Evidence / Privacy
14 Reports
```

Never show:

```text
Download block
Download approval
Download quarantine
```

### If Notify parent

Show:

```text
9.2 Download evidence
14.3 Notifications
14 Reports
```

### If Ask parent

Show:

```text
9.2 Download evidence
9.3 Download actions
12 Approvals
14 Reports
```

### If Block risky downloads

Show:

```text
9.2 Download evidence
9.3 Download actions
12 Approvals if override allowed
15 Capability Failure
14 Reports
18 Audit
```

### If Block all unless approved

Show:

```text
9.2 Download evidence
9.3 Download actions
12 Approvals
14 Reports
18 Audit
```

---

# 12. Schedule forest

## 10.1 When should browser policy apply?

Show if:

```text
1.1 = On
AND 1.2 includes Warn, Ask parent, Limit, or Block
```

Optional if:

```text
1.2 = Observe
```

Never show if:

```text
1.1 = Off
Emergency allow
Emergency block
Paused until time, except pause display
```

---

## 10.2 Temporary overrides

Show if:

```text
1.1 = On
OR 1.1 = Paused
OR 1.1 = Emergency allow
OR 1.1 = Emergency block
```

Never show if:

```text
No parent override permission exists globally
```

---

# 13. Time limit forest

## 11.1 Should browser activity have time limits?

Show if:

```text
1.2 includes Limit
OR 5.1 includes Browser session
OR 5.1 includes Browser app time
OR 8.1 = Limit
```

Never show if:

```text
1.2 does not include Limit
AND no time-based target selected
```

---

## 11.2 What type of time limit?

Show only if:

```text
11.1 = Yes
```

Never show if:

```text
11.1 = No
```

---

# 14. Approval forest

## 12.1 What requires parent approval?

Show if:

```text
1.2 includes Ask parent
OR 4.1 = Ask parent before continuing
OR 5.2 = Ask parent
OR 6.1 includes Ask parent
OR 7.1 = Ask parent
OR 8.1 = Ask parent
OR 9.1 = Ask parent
OR time extension is allowed
OR emergency unlock is allowed
```

Never show if:

```text
No ask-parent behavior exists anywhere
```

---

## 12.2 What happens if parent does not answer?

Show only if:

```text
12.1 has at least one selected approval trigger
```

Never show if:

```text
12.1 has no selected approval trigger
```

---

## 12.3 How long does approval last?

Show only if:

```text
12.1 has at least one selected approval trigger
```

Never show if:

```text
12.1 has no selected approval trigger
```

---

# 15. Evidence / privacy forest

## 13.1 What browser evidence may be collected?

Show if:

```text
1.1 = On
AND any of these are visible:
2 Browser Coverage
5 Rule Targets
7 Search Rules
8 Video Rules
9 Downloads
14 Reports
```

Always show before exact URL/search/report detail choices.

---

## 13.2 What evidence must never be collected?

Show if:

```text
13.1 is shown
```

Never show if:

```text
1.1 = Off
AND reports/evidence globally disabled
```

Important blockers:

```text
If No exact URL -> disable exact URL rules/reports unless required.
If No search term -> disable search-term reports.
If No screenshots -> never show screenshot report options.
If No raw browser data upload -> use local summary/evidence refs only.
If No file contents -> never show file-content scan/upload.
```

---

# 16. Reports forest

## 14.1 What should parent see?

Show if:

```text
1.1 is any value
```

But content changes:

```text
Off -> policy status only
Paused -> pause status + audit
Emergency -> emergency status + audit
On -> full report choices
```

---

## 14.2 What report detail level?

Show if:

```text
14.1 has any selected report item beyond policy status
```

Never show if:

```text
Report item = policy status only
```

---

## 14.3 When should parent be notified?

Show if:

```text
Any event can happen:
new browser found
unmanaged browser used
blocked site
blocked download
approval request
time limit reached
bypass attempt
managed browser cannot launch
setup repair requested
emergency override used
```

Never show if:

```text
No notification-capable event is selected
```

---

# 17. Offline / capability forest

## 15.1 What should happen if child device or local agent is offline?

Show if:

```text
1.1 = On
AND 1.2 includes Ask parent, Limit, or Block
```

Optional if:

```text
1.2 includes Observe or Warn
```

Never show if:

```text
1.1 = Off
```

---

## 15.2 What should happen if browser/platform capability is unsupported?

Show if:

```text
2.1 includes Strict coverage
OR 2.1 = Custom
OR 3.1 requires managed browser
OR 5.1 includes Exact URL/Search/Downloads/Video
OR 6.1 includes Block/Close/Require managed browser
```

Never show if:

```text
Only domain/category observe reporting is used
```

---

## 15.3 What should happen if exact proof is missing?

Show if:

```text
5.1 includes Exact URL
OR 5.1 includes Search terms
OR 7.1 uses search enforcement
OR 3.1 requires managed browser for exact rules
```

Never show if:

```text
No exact rule/evidence is selected
```

---

# 18. Setup forest

## 16.1 Should setup/provisioning controls be shown?

Show if:

```text
3.1 requires/prefer managed browser
OR 2.2 = Standard/Strict/Custom
OR 2.3 asks/blocks new browsers
OR 15.2 unsupported capability fallback is relevant
OR setup is broken
```

Never show if:

```text
Any browser allowed
AND basic discovery only
AND no setup issue exists
```

---

## 16.2 What setup tasks are allowed?

Show only if:

```text
16.1 = Show full setup controls
OR setup is broken and repair is available
```

Never show if:

```text
16.1 = Hide unless broken
AND setup is healthy
```

---

# 19. Data forest

## 17.1 How long should browser-control data be kept?

Show if:

```text
14 Reports enabled
OR 18 Audit enabled
OR 13 Evidence collection enabled
OR 12 Approvals enabled
```

Never show if:

```text
No reports
No audit
No evidence
No approvals
```

---

## 17.2 What data can parent export/delete?

Show if:

```text
17.1 is not Do not store
```

Never show if:

```text
17.1 = Do not store
```

---

# 20. Audit forest

## 18.1 How much audit history should be kept?

Show always.

Reason:

```text
Even Off / Pause / Emergency are policy events.
```

---

## 18.2 What must be audited?

Show if:

```text
18.1 = Standard
OR 18.1 = Detailed
OR 18.1 = Custom
```

Never show if:

```text
18.1 = Minimal
```

---

# 21. AI / classification service forest

AI is outside browser policy.

## A1. Can classification help browser decisions?

Show only if:

```text
5.1 includes Category
OR 5.2 = Use category/classification service if available
OR unknown-page handling needs classification
OR report summaries are enabled globally
```

Never show inside browser policy rule card as normal browser setting.

Show as:

```text
Service dependency: Classification
```

---

## A2. What AI/browser assistance is allowed?

Show if:

```text
A1 is not No
OR parent opens AI service settings
```

Never show if:

```text
A1 = No, deterministic rules only
AND parent is inside browser policy page
```

---

## A3. What happens if AI/classification is unavailable?

Show if:

```text
A1 uses local classification
OR A1 uses portal/cloud classification
```

Never show if:

```text
A1 = No, deterministic rules only
```

---

# 22. Compact show order

Use this order when rendering cards:

```text
1.1
1.2
1.3

2.1
2.2
2.3

3.1
3.2
3.3

4.1
4.2
4.3

5.1
5.2
5.3

6.1
6.2

7.1
7.2

8.1
8.2

9.1
9.2
9.3

10.1
10.2

11.1
11.2

12.1
12.2
12.3

13.1
13.2

14.1
14.2
14.3

15.1
15.2
15.3

16.1
16.2

17.1
17.2

18.1
18.2

A1
A2
A3
```

But only render a card when its show condition is true.

---

# 23. Simplest renderer rule

Every question card should have:

```ts
showWhen: Condition[]
neverShowWhen: Condition[]
disabledWhen: Condition[]
readonlyWhen: Condition[]
```

Example:

```ts
{
  id: "12.2",
  question: "What happens if parent does not answer?",
  showWhen: [
    "12.1 has at least one selected approval trigger"
  ],
  neverShowWhen: [
    "No ask-parent behavior exists anywhere"
  ]
}
```

Example:

```ts
{
  id: "9.2",
  question: "What download evidence is allowed?",
  showWhen: [
    "5.1 includes Downloads",
    "9.1 is not Ignore"
  ],
  neverShowWhen: [
    "5.1 does not include Downloads",
    "9.1 = Ignore"
  ]
}
```

Example:

```ts
{
  id: "15.3",
  question: "What should happen if exact proof is missing?",
  showWhen: [
    "5.1 includes Exact URL OR Search terms",
    "OR 3.1 requires managed browser for exact rules"
  ],
  neverShowWhen: [
    "No exact rule/evidence is selected"
  ]
}
```
