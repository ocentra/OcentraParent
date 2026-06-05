SOCIAL-13 live public surface proof

Live capture command:

- `cmd /c node scripts/test/social-account-creation-live-proof.mjs`

Saved artifacts:

- `test-results/social-account-creation-live-proof/proof.json`
- `output/browser-plan-proof/social-13-managed-browser-account-creation-gate/11-live-proof.json`
- `output/browser-plan-proof/social-13-managed-browser-account-creation-gate/06-live-screenshots/facebook-signup.png`
- `output/browser-plan-proof/social-13-managed-browser-account-creation-gate/06-live-screenshots/pinterest-login.png`
- `output/browser-plan-proof/social-13-managed-browser-account-creation-gate/06-live-screenshots/reddit-register.png`
- `output/browser-plan-proof/social-13-managed-browser-account-creation-gate/06-live-screenshots/instagram-signup.png`

Captured public surfaces:

| Target | Public route | Contract result |
| --- | --- | --- |
| Facebook signup | `https://www.facebook.com/r.php` | Signup form evidence; `hold-for-parent-approval`; parent approval request ref present. |
| Pinterest login | `https://www.pinterest.com/login/` | Login form evidence; `allow-navigation-candidate`; policy candidate ref present. |
| Reddit register | `https://www.reddit.com/register/` | Signup form evidence; `block-submit-candidate`; policy candidate ref present. |
| Instagram signup | `https://www.instagram.com/accounts/emailsignup/` | Signup form evidence; `manual-review-required`; policy candidate ref present. |

Privacy and no-claim boundary:

- Playwright only navigates to public signup/login/register routes and takes screenshots.
- The proof does not type credentials, submit forms, create accounts, persist raw DOM, persist raw page body, or store field values.
- Final URLs are persisted as route-only URLs; query strings and hashes are removed before proof JSON is written.
- Page titles are stored only as length and SHA-256 hash.
- The detector stores sanitized visible control kinds such as `email-input`, `password-input`, `submit-button`, and optional profile/birthdate hints.
