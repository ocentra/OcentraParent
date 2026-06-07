# SOCIAL-15 Contract Proof

The bypass detector accepts:

- unmanaged or browser-like process kind;
- redacted executable path, process hash, and signature refs;
- unmanaged detection confidence and reason codes;
- suspected platform refs;
- existing unmanaged detection and fallback states.

The produced evidence is bypass-only and managed-browser-required. It keeps exact
social URL, managed route, account, feed/video, message, native-app, connector,
UI, process-control, and enforcement claims false.

The focused Vitest suite verifies possible social bypass process detection,
supported browser outside the managed session detection, redacted-path rejection,
exact URL proof rejection, managed-session rejection, runtime fallback rejection,
and negative social/runtime claim rejection.

`scripts/test/social-unmanaged-bypass-live-process-proof.mjs` adds live-process
evidence for the same contract. It launches a real local system browser against
public social/video surfaces, captures only redacted executable, process hash,
command-line hash, and target hash refs, parses those refs through the
SOCIAL-15 detector, and rejects dishonest mutations for exact URL, route/content,
UI, native, connector, process-control, relaunch, and enforcement claims.
