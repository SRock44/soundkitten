# SoundKitten OAuth token-exchange proxy

A small Cloudflare Worker that does exactly one thing: exchanges a SoundCloud OAuth authorization code (or refresh token) for access tokens, using a `client_secret` that lives only here, never in the app.

## Why this exists

SoundCloud's OAuth token endpoint requires a `client_secret` even for the PKCE flow a native desktop app uses, which normally shouldn't need one. A secret embedded in a distributed, open-source app is not actually secret: it is extractable from the compiled binary, and would be sitting in plain text in this repo if it were ever hardcoded into the app's own source. This worker keeps the secret on infrastructure the developer controls instead. The app completes the PKCE flow itself (the `code_verifier` never leaves the app) and only ever sends this worker the authorization code it got back from SoundCloud. The worker is the only thing that ever combines that code with the secret.

## What it is not

It is not a general authentication server, it does not store anything, it does not see or handle any user's SoundCloud password or session, and it does not gate access to anyone's account data. It is a one-shot, stateless forwarder for a single OAuth step.

## Deploy

Requires a Cloudflare account and [wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/).

```sh
cd services/oauth-proxy
npx wrangler secret put SC_CLIENT_ID
npx wrangler secret put SC_CLIENT_SECRET
npx wrangler secret put WORKER_ACCESS_KEY   # any random string, see note in src/index.js
npx wrangler deploy
```

Then point a domain at it (suggested: `auth.soundkitten.org`, alongside the existing `updates.soundkitten.org` updater infrastructure), via Cloudflare dashboard > Workers & Pages > this worker > Triggers > Custom Domains.

## API

Both endpoints accept `POST` with a JSON body and an `x-soundkitten-key` header matching `WORKER_ACCESS_KEY`.

- `POST /token/exchange`, body: `{ "code": "...", "code_verifier": "...", "redirect_uri": "http://127.0.0.1:8765/callback" }`
- `POST /token/refresh`, body: `{ "refresh_token": "..." }`

Both return whatever SoundCloud's own token endpoint returns (`access_token`, `refresh_token`, `expires_in`, or an error), unmodified, with the same status code.

## Cost

Free tier (100,000 requests/day) covers this easily. Each app login or token refresh is one request.
