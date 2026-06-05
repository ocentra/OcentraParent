SOCIAL-13 security and negative proof

Negative cases covered by `packages/activity-domain/tests/browser-social-account-creation-gate.test.ts`:

- mismatched account-flow and form-shape evidence is rejected;
- hold-for-parent-approval without an approval request ref is rejected;
- block-submit without a policy decision candidate ref is rejected;
- navigation pause claims are rejected;
- form-submit block claims are rejected;
- child and parent UI claims are rejected;
- final policy decision claims are rejected;
- enforcement, native app, and connector claims are rejected;
- credential capture, form submission, and account creation claims are rejected;
- manual-review candidates with a planned state are rejected.

Live negative cases covered by `scripts/test/social-account-creation-live-proof.mjs`:

- each live gate plan is parsed by `BrowserSocialAccountCreationGatePlanSchema` before mutation;
- live mutations with `navigationPausedClaimed`, `formSubmitBlockedClaimed`, `childUiRenderedClaimed`, `parentUiNotifiedClaimed`, `policyDecisionClaimed`, `enforcementClaimed`, `nativeAppControlClaimed`, `platformConnectorClaimed`, `credentialCaptured`, `formSubmittedClaimed`, or `accountCreatedClaimed` set to true are rejected;
- a live manual-review mutation with `gateState: planned` is rejected.

Security boundary:

SOCIAL-13 accepts route-only account-flow evidence, sanitized visible form-shape evidence, and refs. The live proof removes query strings and hashes from final URLs before persistence, stores page titles only as hashes, and rejects raw credential/form/account outcome claims plus all runtime/UI/native/connector/enforcement authority.
