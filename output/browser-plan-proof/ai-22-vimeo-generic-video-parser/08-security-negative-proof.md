# AI-22 Security Negative Proof

Rejected states:

- unmanaged browser process rows cannot produce Vimeo or generic video metadata
  evidence;
- generic web OpenGraph-only rows cannot claim video metadata;
- metadata rows cannot claim page-body capture;
- metadata rows cannot claim transcript text capture;
- metadata rows cannot claim content semantics authority;
- metadata rows cannot claim AI decisions, policy decisions, or policy
  authority.

Live proof negative checks:

- `generic-open-graph-only-rejected`
- `unmanaged-vimeo-exact-url-rejected`

Live proof redaction checks:

- `persistedRawHtml=false`
- `persistedRawTitle=false`
- `persistedRawDescription=false`
- `pageBodyCaptured=false`
- `transcriptTextCaptured=false`
- `contentSemanticsClaimed=false`
- `aiDecisionClaimed=false`
- `policyDecisionClaimed=false`
- `policyAuthorityClaimed=false`
- `hiddenPageLoadClaimed=false`
- `enforcementClaimed=false`

Security boundary:

The adapter turns already-collected exact URL and metadata facts into a typed
evidence row. It does not fetch network metadata, load hidden pages, parse
transcripts, evaluate policy, or mutate browser/enforcement state.
