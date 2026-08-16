# Per-Thread Instruction Index

Each file is a lane-manager dispatch note derived from the thread self-assessment plus coordinator structural review.

## Dispatch order summary

| Tier | Threads |
| --- | --- |
| Global | `lane-manager`, repo-audit WP01-WP05 |
| Foundation | `logging-domain-parity`, `cloudflare-control-plane-plan`, `data-custody-storage-plan`, `account-identity-family-plan`, `device-trust-bootstrap-plan` |
| Infrastructure | `lan-plan`, `eventing-plan`, `policy-control-plane-plan` |
| Runtime domains | `tracking-plan`, `network-plan`, `browser-plan`, `app-game-plan`, `app-plan`, `ai-plan` |
| Product overlays | `setup-install-provisioning-plan`, `portal-ux-household-surfaces-plan`, `screen-plan`, `screen-ai-pipeline-plan`, `v0-8-enforcement-control-plan` |
| Late closure | `child-agent-runtime-distribution-plan`, `parent-desktop-runtime-package-plan`, `remote-access-plan`, `payment-subscription-plan` |

## Files

| Thread | Instruction |
| --- | --- |
| lane manager | [lane-manager.md](lane-manager.md) |
| account identity | [account-identity-family-plan.md](account-identity-family-plan.md) |
| AI | [ai-plan.md](ai-plan.md) |
| app-game | [app-game-plan.md](app-game-plan.md) |
| app | [app-plan.md](app-plan.md) |
| browser | [browser-plan.md](browser-plan.md) |
| child distribution | [child-agent-runtime-distribution-plan.md](child-agent-runtime-distribution-plan.md) |
| Cloudflare | [cloudflare-control-plane-plan.md](cloudflare-control-plane-plan.md) |
| data custody | [data-custody-storage-plan.md](data-custody-storage-plan.md) |
| device trust | [device-trust-bootstrap-plan.md](device-trust-bootstrap-plan.md) |
| eventing | [eventing-plan.md](eventing-plan.md) |
| LAN | [lan-plan.md](lan-plan.md) |
| logging | [logging-domain-parity.md](logging-domain-parity.md) |
| network | [network-plan.md](network-plan.md) |
| parent runtime package | [parent-desktop-runtime-package-plan.md](parent-desktop-runtime-package-plan.md) |
| payment | [payment-subscription-plan.md](payment-subscription-plan.md) |
| policy | [policy-control-plane-plan.md](policy-control-plane-plan.md) |
| portal UX | [portal-ux-household-surfaces-plan.md](portal-ux-household-surfaces-plan.md) |
| remote access | [remote-access-plan.md](remote-access-plan.md) |
| screen AI pipeline | [screen-ai-pipeline-plan.md](screen-ai-pipeline-plan.md) |
| screen | [screen-plan.md](screen-plan.md) |
| setup/install | [setup-install-provisioning-plan.md](setup-install-provisioning-plan.md) |
| tracking | [tracking-plan.md](tracking-plan.md) |
| enforcement | [v0-8-enforcement-control-plan.md](v0-8-enforcement-control-plan.md) |

## Per-thread closeout rule

Each thread must return exact changed files, exact commands, generated proof outputs, unresolved blockers, and the next slice it recommends. The lane manager decides whether that next slice is accepted.
