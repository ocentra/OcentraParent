# Logger Usage Pattern Standard

## Purpose

Every core file that participates in runtime, event, command, request, proof, or test chains must be logger-ready. Logging is not random console output. It is controlled proof and diagnostics infrastructure.

This standard is universal for OcentraParent. It is not Cloudflare-only. It applies to packages, apps, scripts, infra workers when present, tests, proof runners, and Rust crates through the Rust parity rule below.

This standard adapts the Ocentra Games logger pattern to OcentraParent names and boundaries.

## Existing OcentraParent surface

Use the existing package:

```ts
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace, type StackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
```

`logging-domain` already provides core logger, stack trace, test-log, app-log, and bridge transport exports.

## Applies to

| Surface | Rule |
| --- | --- |
| `packages/*/src` runtime/domain behavior | logger-ready unless pure schema/constants/brands only |
| `packages/*/tests` | initialize logger for chain/proof tests |
| `apps/*/src` command paths, dev panels, route actions | logger-ready for command/read-model proof |
| `apps/*/tests` and Playwright/e2e | initialize logger and correlate UI proof to logs/events/read models |
| `scripts/test/*` and proof runners | logger-ready and artifact-producing |
| `scripts/*` operational helpers | logger-ready when part of validation/proof/runtime flow |
| `infra/*` workers/handlers, if present | same pattern; not special-cased to Cloudflare only |
| `crates/*` Rust runtime/proof code | Rust parity trace/event adapter required |

## Module-level TS pattern

Use in handlers, services, proof helpers, and tests that are not classes:

```ts
const log = Logger.instance;
log.register(import.meta.url);

const logInfo = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  log.logInfo(message, stackTrace, data, enabled);
};

const logWarn = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  log.logWarn(message, stackTrace, data, enabled);
};

const logError = (message: string, stackTrace: StackTrace, data?: unknown): void => {
  log.logError(message, stackTrace, data);
};

const logDebug = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  log.logDebug(message, stackTrace, data, enabled);
};
```

## Class/service TS pattern

Use in classes, worker-style services, Durable Object-style services, and stateful helpers:

```ts
private readonly log = Logger.instance;

constructor(/* args */) {
  this.log.register(import.meta.url);
}

private logInfo = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  this.log.logInfo(message, stackTrace, data, enabled);
};

private logWarn = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  this.log.logWarn(message, stackTrace, data, enabled);
};

private logError = (message: string, stackTrace: StackTrace, data?: unknown): void => {
  this.log.logError(message, stackTrace, data);
};

private logDebug = (message: string, stackTrace: StackTrace, data?: unknown, enabled: boolean = false): void => {
  this.log.logDebug(message, stackTrace, data, enabled);
};
```

## Usage points

Log only meaningful chain points:

| Point | Level |
| --- | --- |
| command/request entry | info, usually disabled by default |
| branch decision | info/debug, disabled by default unless proof run enables it |
| rejection/manual-required/degraded state | warn or info depending severity |
| validation/runtime error | error |
| important success outcome | info, disabled by default unless proof run enables it |
| proof milestone | info/debug under proof/test enablement |

Always pass `getStackTrace()` as the stack trace argument.

## Enablement rule

Default runtime must not become noisy.

| Context | Expected enablement |
| --- | --- |
| production | errors and selected warnings only |
| dev command route | selected flow logs enabled |
| unit/contract tests | targeted logs enabled only when they prove chain behavior |
| integration/e2e/proof | chain logs enabled and persisted |
| failed proof debug | expanded logs allowed if scoped and redacted |

## Test rule

Every new test file for a runtime/proof chain should initialize the logger pattern. Tests should log the proof chain start, key boundary milestones, and chain completion or failure.

## Lint rule

Do not remove logger helpers just because a file has not used every helper yet. If lint complains about unused logger helpers, add or adjust a repo lint exception narrowly for this standard rather than deleting the helpers.

## Rust parity rule

Rust runtime crates need an equivalent pattern:

- crate/module-level trace context;
- event/journal correlation id;
- helper functions or adapters for info/warn/error/debug-style milestones;
- integration with `crates/ocentra-eventing` journal/envelope/request surfaces;
- proof/test access to the emitted trace.

If a Rust crate cannot emit or expose proof trace, add a small logging/event adapter slice before treating the crate as proof-ready.

## Anti-patterns

| Anti-pattern | Correction |
| --- | --- |
| `console.log` proof | use logger/test-log/proof artifact |
| no logger in core runtime file | add logger-ready pattern |
| noisy production logs | keep default disabled/gated |
| proof without trace id | add run id and correlation id |
| test only asserts return value | also assert log/event/read-model milestone when proving chain |
| treating this as Cloudflare-only | apply it universally to runtime/proof/test chain files |
