# logging-domain-parity Instruction

## Verdict

`partial but useful early`. WP07/WP10 proof-root slice appears coherent; full plan still not done.

## Assign first

`logging-wp03-portal-dev-log-consumer-closeout`:

- align portal/dev-log consumer proof with the restored MCP/proof-trace roots;
- update docs and proof inventory only for verified rows;
- rerun focused portal/logging tests.

## Then

`logging-wp06-checker-enforcement-hardening`:

- ensure proof inventory/checker enforcement catches missing proof roots and stale claims.

## Coordinate with

- Lane manager and all plans: logging proof/debug surface is shared infrastructure.

## Do not

- Do not claim full logging-domain-parity closure from WP07/WP10 only.
- Do not rely on ambient old logs; proof roots must be self-seeded.
- Do not weaken proof-trace or MCP smoke assertions.
