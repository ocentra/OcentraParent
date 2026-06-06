# SOCIAL-22 Contract Proof

The social audit/explanation snapshot requires readable rows for:

- account approval;
- feed/video gate;
- native-app gap;
- connector boundary;
- decision memory;
- manual-required gap.

Rows carry evidence links, audit refs, policy refs when applicable, parent
approval refs, decision-memory refs, connector/native/manual gap refs, policy
reason codes, explanation reasons, and an audience. Ready parent rows must cite a
policy candidate. Account approval rows must cite parent approval refs.
Feed/video gate rows must cite route evidence and a non-unknown action
candidate. Decision-memory rows stay contract-only. Native, connector, and
manual gap rows stay manual-required.

The focused Vitest suite accepts an honest six-row snapshot and rejects missing
subjects, ready rows without backing refs, unknown feed/video action candidates,
and runtime overclaims.
