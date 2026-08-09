/**
 * SoundKitten OAuth token-exchange proxy (Cloudflare Worker).
 *
 * This exists for exactly one reason: SoundCloud's official OAuth token
 * endpoint requires a client_secret even for a PKCE-based native/desktop
 * app, which is not how a public client is supposed to work. A secret
 * baked into a distributed, open-source app binary is extractable by
 * anyone willing to decompile it, and worse, would be sitting in plain
 * text in this very repo if it were ever hardcoded into the app's source.
 *
 * The fix: the secret lives ONLY here, as a Worker environment variable
 * (never committed to source, set via `wrangler secret put` or the
 * Cloudflare dashboard), on infrastructure the developer controls. The
 * app itself never sees it. All the app does is complete the normal PKCE
 * flow (code_verifier/code_challenge stay entirely app-side) and hand the
 * resulting authorization code to this worker, which is the only thing
 * that ever combines that code with the secret.
 *
 * This worker does nothing else. No user data passes through it beyond
 * what SoundCloud's own token endpoint already requires, and it never
 * stores anything, every request is stateless.
 *
 * Deploy:
 *   wrangler secret put SC_CLIENT_ID
 *   wrangler secret put SC_CLIENT_SECRET
 *   wrangler secret put WORKER_ACCESS_KEY   (see note below)
 *   wrangler deploy
 */

const TOKEN_URL = "https://secure.soundcloud.com/oauth/token";

// Only loopback redirect URIs are accepted, matching the native-app PKCE
// pattern (RFC 8252) the desktop app actually uses. This keeps the worker
// scoped to "help the real app log in," not a general-purpose token proxy.
const ALLOWED_REDIRECT_PREFIX = "http://127.0.0.1:";

function json(data, status = 200) {
  return new Response(JSON.stringify(data), {
    status,
    headers: { "content-type": "application/json" },
  });
}

async function forwardToSoundCloud(env, params) {
  const resp = await fetch(TOKEN_URL, {
    method: "POST",
    headers: { "content-type": "application/x-www-form-urlencoded" },
    body: new URLSearchParams({
      client_id: env.SC_CLIENT_ID,
      client_secret: env.SC_CLIENT_SECRET,
      ...params,
    }),
  });
  const data = await resp.json();
  return json(data, resp.status);
}

async function handleExchange(env, body) {
  const { code, code_verifier, redirect_uri } = body;
  if (!code || !code_verifier || !redirect_uri) {
    return json({ error: "missing code, code_verifier, or redirect_uri" }, 400);
  }
  if (!redirect_uri.startsWith(ALLOWED_REDIRECT_PREFIX)) {
    return json({ error: "redirect_uri not allowed" }, 400);
  }
  return forwardToSoundCloud(env, {
    grant_type: "authorization_code",
    redirect_uri,
    code,
    code_verifier,
  });
}

async function handleRefresh(env, body) {
  const { refresh_token } = body;
  if (!refresh_token) {
    return json({ error: "missing refresh_token" }, 400);
  }
  return forwardToSoundCloud(env, {
    grant_type: "refresh_token",
    refresh_token,
  });
}

export default {
  async fetch(request, env) {
    if (request.method !== "POST") {
      return json({ error: "POST only" }, 405);
    }

    // Not the real security boundary, PKCE plus a genuine, freshly-completed
    // SoundCloud login is what actually protects this. This key just keeps
    // random internet scanners from costing free-tier invocations by
    // hitting the endpoint with garbage. It's fine for this to also live in
    // the app binary (it protects nothing sensitive on its own), unlike
    // SC_CLIENT_SECRET, which must never appear in the app at all.
    if (env.WORKER_ACCESS_KEY && request.headers.get("x-soundkitten-key") !== env.WORKER_ACCESS_KEY) {
      return json({ error: "unauthorized" }, 401);
    }

    let body;
    try {
      body = await request.json();
    } catch {
      return json({ error: "invalid JSON body" }, 400);
    }

    const { pathname } = new URL(request.url);
    if (pathname === "/token/exchange") return handleExchange(env, body);
    if (pathname === "/token/refresh") return handleRefresh(env, body);
    return json({ error: "not found" }, 404);
  },
};
