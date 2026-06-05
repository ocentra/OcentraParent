# AI-24 Security Negative Proof

The AI-24 provider fallback contract rejects state that would turn AI routing
into hidden authority or enforcement.

Negative cases covered by real parser tests:

- Hidden parent fallback visibility is rejected.
- Hidden child fallback visibility is rejected.
- Claimed AI analysis result authority is rejected.
- Claimed policy decision authority is rejected.
- Missing local safety preservation is rejected.
- Remote default-for-blocking behavior is rejected.
- Remote outage disabling local safety is rejected.
- Family-hub fallback while child-device local AI is already selected is
  rejected.
- Parent-approved remote fallback while child-device local AI is already
  selected is rejected.
- Parent-approved remote fallback while family AI hub is already selected is
  rejected.
- Selected runtime refs that do not match the selected route runtime are
  rejected.

Data custody and no-claim checks:

- The contract carries refs and route states only; it does not carry raw page
  body, transcript text, screenshots, cookies, tokens, credentials, connector
  tokens, or local storage.
- Remote AI remains parent-approved only and cannot become the default blocking
  path.
- AI routing remains evidence for later parent policy; it is not policy
  authority and cannot directly enforce.
- UI delivery, model execution, provider runtime calls, alerts, native app
  control, connector behavior, and browser enforcement remain unclaimed.
