# SOCIAL-18 Contract Proof

The platform connector authorization boundary requires rows for:

- Google/YouTube supervision;
- Meta Family Center;
- TikTok Family Pairing;
- platform export/import;
- parent-provided account refs.

The accepted states keep connector support optional and adjacent to browser
gating. Provider rows remain not-implemented until provider artifacts, visible
parent authorization, token custody, privacy, and runtime proof exist.
Platform export/import stays manual-required. Parent-provided account refs may
be parent-authorized only as redacted refs with a visible parent setting and
consent proof.

The focused Vitest suite accepts an honest five-row boundary and rejects missing
providers, token/API/content/runtime claims, and unsupported authorization
upgrades.

The live public-page proof builds another honest five-row boundary from
Google/YouTube supervision, Meta Family Center, and TikTok Family Pairing
public pages. Those captures become provider proof refs only. They do not
authorize connectors, store tokens, call provider APIs, capture raw account or
message/feed data, deliver UI, make policy decisions, control native apps, or
enforce actions.
