import type { DurableObjectState } from "@cloudflare/workers-types";
import {
  isRouteKillSwitchEnabled,
  parseAllowedOrigins,
  parseRequestMaxBytes,
  validateEnv,
  type Env,
} from "./env.js";
import { findRoute, type RouteManifestEntry } from "./routes.js";
import { verifyAuthState } from "./auth/verifier.js";

const STATE_CHANGING_METHODS = new Set(["POST", "PUT", "PATCH", "DELETE"]);

function json(status: number, body: unknown, headers: HeadersInit = {}): Response {
  return new Response(JSON.stringify(body, null, 2), {
    status,
    headers: {
      "content-type": "application/json; charset=utf-8",
      ...headers,
    },
  });
}

function withCors(response: Response, request: Request, env: Env): Response {
  const headers = new Headers(response.headers);
  const origin = request.headers.get("origin");
  headers.set("access-control-allow-methods", "GET,POST,OPTIONS");
  headers.set("access-control-allow-headers", "authorization,content-type,stripe-signature,paypal-transmission-id,x-razorpay-signature,x-goog-signature,x-ocentra-role,x-ocentra-trusted-device,x-ocentra-internal-call");
  headers.set("access-control-max-age", "86400");
  headers.set("vary", "origin");
  headers.set("access-control-allow-origin", resolveResponseOrigin(origin, env));

  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers,
  });
}

function resolveResponseOrigin(origin: string | null, env: Env): string {
  const allowedOrigins = parseAllowedOrigins(env);
  if (!origin) {
    return allowedOrigins[0] ?? env.APP_ORIGIN;
  }
  return allowedOrigins.includes(origin) ? origin : env.APP_ORIGIN;
}

function isAllowedOrigin(origin: string | null, env: Env): boolean {
  if (!origin) {
    return true;
  }
  return parseAllowedOrigins(env).includes(origin);
}

function manualRequiredResponse(route: RouteManifestEntry): Response {
  return json(501, {
    status: "manual-required",
    handlerKey: route.handlerKey,
    authState: route.authState,
    proofIdFamily: route.proofIdFamily,
    message: "This route is scaffolded only. Real billing logic is still owned by the active plan workpacks.",
  });
}

async function handleRequest(request: Request, env: Env): Promise<Response> {
  const validationErrors = validateEnv(env);
  if (validationErrors.length > 0) {
    return json(500, {
      error: "environment-validation-failed",
      validationErrors,
    });
  }

  if (request.method === "OPTIONS") {
    return new Response(null, { status: 204 });
  }

  if (!isAllowedOrigin(request.headers.get("origin"), env)) {
    return json(403, {
      error: "cors-origin-rejected",
      allowedOrigins: parseAllowedOrigins(env),
    });
  }

  const contentLength = Number(request.headers.get("content-length") ?? "0");
  if (contentLength > parseRequestMaxBytes(env)) {
    return json(413, {
      error: "payload-too-large",
      maxBytes: parseRequestMaxBytes(env),
    });
  }

  if (isRouteKillSwitchEnabled(env) && STATE_CHANGING_METHODS.has(request.method)) {
    return json(503, {
      error: "billing-route-kill-switch-enabled",
      status: "manual-required",
    });
  }

  const route = findRoute(new URL(request.url).pathname, request.method);
  if (!route) {
    return json(404, {
      error: "route-not-found",
    });
  }

  if (route.authState !== "public") {
    const authResult = await verifyAuthState(route.authState, request, env);
    if (!authResult.ok) {
      return authResult.response;
    }
  }

  if (route.handlerKey === "health") {
    return json(200, {
      status: "ok",
      mode: "scaffold-only",
      owner: "cloudflare-control-plane-plan",
    });
  }

  return manualRequiredResponse(route);
}

class BasePlaceholderDO {
  constructor(
    protected readonly state: DurableObjectState,
    protected readonly env: Env,
  ) {}

  async fetch(): Promise<Response> {
    void this.state;
    void this.env;
    return json(501, {
      status: "manual-required",
      message: "Durable Object scaffold exists, but runtime behavior is not implemented yet.",
    });
  }
}

export class BillingControlDO extends BasePlaceholderDO {}

export class ReferralControlDO extends BasePlaceholderDO {}

export class EntitlementSnapshotDO extends BasePlaceholderDO {}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    return withCors(await handleRequest(request, env), request, env);
  },

  async scheduled(): Promise<void> {
    return;
  },
};
