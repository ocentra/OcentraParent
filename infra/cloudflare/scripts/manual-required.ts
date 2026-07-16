const PREFIX = '[cloudflare-control-plane]';

export function failWithBlocker(scope: string, blocker: string, nextStep: string): never {
  const payload = {
    scope,
    status: 'manual-required',
    blocker,
    nextStep,
  };

  console.error(`${PREFIX} ${JSON.stringify(payload, null, 2)}`);
  process.exit(1);
}
