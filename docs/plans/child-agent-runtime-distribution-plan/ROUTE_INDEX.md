# Route Index

Use [AGENTS.md](AGENTS.md), then [PLAN_STATE.md](PLAN_STATE.md), [NEXT_ACTIONS.md](NEXT_ACTIONS.md), and [WORKPACK_INDEX.md](WORKPACK_INDEX.md). Select one graph-legal workpack and stop.

This plan owns child artifact identity, installed child startup, authenticated child ingress and health, platform package/lifecycle integration, parent-authorized platform removal callbacks, updater/distribution consumption, and the child aggregate release gate. It does not own the Setup producer/UI journey, Device Trust material, Account authority, or parent-client distribution.

Current first source routes are WP06 (canonical iOS child identity) and WP10 (Device Trust-backed startup, authenticated ingress/health, and live handoff/update consumption). WP02-WP05 follow WP10; WP07 follows the platform packets; WP08/WP09 follow their reviewed source dependencies; WP11 is last. Implementation routing never promotes normal READY/DONE.
