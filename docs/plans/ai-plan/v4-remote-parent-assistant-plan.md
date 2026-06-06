# V4 Remote Parent Assistant Plan

## Goal

Define remote/API AI as an optional parent-approved explanation and report layer,
not as normal child-device safety.

Household LAN AI provider execution is separate. A household provider is local
household child-safety compute sharing, remains worker-only, and still returns
results to the evidence-owning child agent for validation. Remote/API assistant
use remains parent-approved explanation/report help outside the normal child
safety path.

## Allowed Uses

- Parent report explanation.
- Parent Q&A over parent-approved report bundles.
- Trend summaries from local or parent-owned exports.
- Unknown/degraded explanation support.
- Setup guidance and policy-authoring help.

## Forbidden Uses

- Normal child safety blocking path.
- Time-limit decisions.
- Ask-parent trigger decisions.
- Direct enforcement.
- Raw child activity upload by default.
- Raw screenshot upload by default.
- Ocentra-hosted child-activity retention by default.
- Override of local policy or stricter parent rules.

## Required Request Fields

- parent action/approval ref;
- source/custody boundary;
- permitted evidence refs or report bundle refs;
- retention state;
- prompt/template version;
- model/provider ref;
- redaction/minimization state;
- answer citation requirements.

## Validation

- Remote disabled by default.
- Remote request rejected without parent approval.
- Remote answer rejected without citations.
- Remote outage degrades to local-only explanation.
- Remote output cannot override local policy.
- Privacy/retention state visible in portal.
