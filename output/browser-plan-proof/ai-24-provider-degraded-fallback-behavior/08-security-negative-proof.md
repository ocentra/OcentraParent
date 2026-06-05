# AI-24 Security Negative Proof

Rejected states:

- hidden parent fallback visibility;
- hidden child fallback visibility;
- claimed AI analysis results;
- claimed policy decisions;
- disabled local safety preservation;
- remote default blocking authority;
- remote outage disabling local safety;
- selected family hub fallback while the local route is already selected;
- selected runtime refs that do not match the selected route;
- selected remote fallback while a local route is already selected;
- selected remote fallback while a family hub route is already selected;
- local, family, or remote route request ids that do not match the fallback
  decision request id.

Security boundary:

AI-24 joins provider route outcomes into a visible fallback decision. It does
not run a model, call a local/family/remote provider, evaluate policy, deliver
UI, mutate the browser runtime, or execute enforcement. The proof harness
records route-derived decision summaries and negative-check booleans only; it
does not persist raw browser state, page body, transcript text, screenshots, or
provider payloads.
