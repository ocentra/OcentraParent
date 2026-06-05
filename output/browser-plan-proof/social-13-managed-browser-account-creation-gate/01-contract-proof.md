SOCIAL-13 contract proof

The activity-domain contract now exposes:

- `BrowserSocialAccountCreationGatePlanSchema` for candidate social account gate plans.
- `planBrowserSocialAccountCreationGate` to copy matching account-flow/form-shape evidence into a gate plan while setting runtime, UI, policy-finality, native, connector, credential, form-submission, account-created, and enforcement claims false.

Focused test:

- `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-social-account-creation-gate.test.ts`
- PASS: 1 test file passed, 4 tests passed.

Live proof:

- `cmd /c node scripts/test/social-account-creation-live-proof.mjs`
- PASS: `plannedCaptureCount=4`
- Planned platforms: `facebook`, `pinterest`, `reddit`, `instagram`.
- The script validates each complete live gate plan with `BrowserSocialAccountCreationGatePlanSchema` before writing proof JSON.

The proof stays partial because it is a plan contract plus live route/form-shape evidence only. It does not pause navigation, block submit, render UI, notify a parent, execute policy, use native app APIs, authorize connectors, or enforce actions.
