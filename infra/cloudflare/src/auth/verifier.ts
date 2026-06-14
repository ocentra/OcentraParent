import type { Env } from "../env.js";
import type { AuthState } from "../routes.js";

export interface VerifiedIdentity {
  subject: string;
  state: AuthState;
}

export type AuthResult =
  | { ok: true; identity: VerifiedIdentity }
  | { ok: false; response: Response };

export const ACCOUNT_AUTH_ADAPTER_BLOCKER = "account-auth-adapter-manual-required";
export const INTERNAL_CALL_BLOCKER = "internal-queue-caller-manual-required";

function json(status: number, body: unknown): Response {
  return new Response(JSON.stringify(body, null, 2), {
    status,
    headers: {
      "content-type": "application/json; charset=utf-8",
    },
  });
}

function missingHeader(headerName: string, state: AuthState): AuthResult {
  return {
    ok: false,
    response: json(401, {
      error: "authentication-required",
      authState: state,
      missingHeader: headerName,
    }),
  };
}

function forbidden(reason: string, state: AuthState): AuthResult {
  return {
    ok: false,
    response: json(403, {
      error: "forbidden",
      authState: state,
      reason,
    }),
  };
}

function manualRequired(blocker: string, state: AuthState): AuthResult {
  return {
    ok: false,
    response: json(503, {
      error: "manual-required",
      authState: state,
      blocker,
    }),
  };
}

function signatureHeaderName(pathname: string): string {
  if (pathname.endsWith("/stripe")) {
    return "stripe-signature";
  }
  if (pathname.endsWith("/paypal")) {
    return "paypal-transmission-id";
  }
  if (pathname.endsWith("/razorpay")) {
    return "x-razorpay-signature";
  }
  if (pathname.endsWith("/apple")) {
    return "authorization";
  }
  return "x-goog-signature";
}

export async function verifyAuthState(
  authState: AuthState,
  request: Request,
  env: Env,
): Promise<AuthResult> {
  if (authState === "public") {
    return {
      ok: true,
      identity: {
        subject: "public",
        state: authState,
      },
    };
  }

  if (authState === "internal-queue-only") {
    if (request.headers.get("x-ocentra-internal-call") !== "true") {
      return forbidden("missing-internal-queue-signal", authState);
    }
    return manualRequired(INTERNAL_CALL_BLOCKER, authState);
  }

  if (authState === "provider-webhook-signature-required") {
    const headerName = signatureHeaderName(new URL(request.url).pathname);
    if (!request.headers.get(headerName)) {
      return missingHeader(headerName, authState);
    }
    return manualRequired("provider-webhook-verification-not-wired", authState);
  }

  if (!request.headers.get("authorization")) {
    return missingHeader("authorization", authState);
  }

  if (authState === "trusted-parent-device-required") {
    if (request.headers.get("x-ocentra-trusted-device") !== "true") {
      return forbidden("trusted-parent-device-required", authState);
    }
  }

  if (authState === "admin-required") {
    if (request.headers.get("x-ocentra-role") !== "admin") {
      return forbidden("admin-role-required", authState);
    }
  }

  if (authState === "support-required") {
    const role = request.headers.get("x-ocentra-role");
    if (role !== "support" && role !== "admin") {
      return forbidden("support-role-required", authState);
    }
  }

  void env;
  return manualRequired(ACCOUNT_AUTH_ADAPTER_BLOCKER, authState);
}
