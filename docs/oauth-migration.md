# Migrating to official OAuth 2.1 + PKCE

Status: Phase 0 (verification) complete, scoped implementation in progress on the `worktree-oauth-migration` branch, kept out of `main` until tested. Written 2026-08-09, updated same day after live-verifying every open question below.

## Summary

SoundKitten currently authenticates two ways:

1. **Unofficial (shipped today, main branch).** A `client_id` scraped from soundcloud.com's public JS bundles, paired with the user's own `oauth_token` session cookie, extracted from a Tauri login webview. This is what every user of the app uses right now.
2. **Official OAuth 2.1 + PKCE.** Registered app, real `client_id`/`client_secret`, standard authorization code + PKCE flow against `secure.soundcloud.com`. Believed broken until 2026-08-09, turned out to be a redirect URI registration mismatch on our end, not a platform bug.

Phase 0 answered every open question this document originally raised (see below). The conclusion: **this is not a full replacement.** Most of the app has to stay on the unofficial path regardless, since the official API cannot stream full tracks and doesn't expose homepage curation. But one specific, high-value capability is confirmed real: **in-app following, which currently doesn't work at all (DataDome-blocked on the unofficial API), works cleanly through official OAuth.** That's the scoped target for this migration.

## Phase 0 results (live-verified 2026-08-09)

| Question | Answer |
|---|---|
| Does official likes/follows bypass DataDome? | **Follow: yes, confirmed twice.** `PUT /me/followings/{id}` and `POST /me/followings/{id}` both return clean 200/201 with real data, no CAPTCHA, verified against a real account (immediately unfollowed after each test to leave no trace). **Like: exhaustively confirmed absent.** Tried 11 different path/method/body-shape combinations across every reasonable REST convention: `/me/favorites/{id}` (PUT+POST), `/tracks/{id}/favoriters`, `/likes/tracks/{id}`, `/me/likes/tracks/{id}`, `/tracks/{id}/likers`, `/tracks/{id}/likes`, `/me/favorites/tracks/{id}`, `/likes/{id}`, and `POST /me/likes/tracks` with the track id in the request body instead of the path. Every single one returned the identical `405 Method Not Allowed: "unknown route"` signature, including an `OPTIONS` check on the working read path (`/me/likes/tracks`) which came back 200 but with no `Allow` header at all. This is not a guessing problem, it's SoundCloud having removed like-writes from the third-party-accessible API entirely while leaving follow-writes in place. Asymmetric, but the evidence is thorough enough to stop looking. |
| Is `client_secret` actually enforced? | **Yes.** Refresh-token exchange without it returns `401 Unauthorized`. The "secret ships in every copy of the app" risk is real, not hypothetical. |
| Does the official API expose streaming? | **Only 30-second previews.** `GET /tracks/{id}` returns a `stream_url` field, and it resolves (HTTP 200, real MP3 bytes), but response headers confirm `x-media-interval: 0-30` and `Content-Length` matches ~30s of audio at the given bitrate. There is no `media`/transcodings field like the unofficial API has. **Playback must stay on the unofficial path, full stop.** |
| Search, feed, mixed-selections, system playlists? | Search (`GET /tracks?q=`) and feed (`GET /me/activities`) both work. `/mixed-selections` does not exist (405 unknown route, same signature as the missing like endpoint). System playlists were not separately tested but given the pattern, assume absent. |
| Comments, followers/following, playlist detail? | Comments (`GET /tracks/{id}/comments`), followers, and followings all work fine on the official API. |
| Can the official access_token be reused against the unofficial api-v2 host? | **No.** `403 Forbidden` when tried. These are fully separate token systems; there is no way to get "one login, works everywhere." Adopting official OAuth for anything means running two auth systems side by side, not replacing one with the other. |

## Decision: scoped migration, not a full one

Given the above, migrating everything would mean losing playback and homepage curation entirely, not an option. The only concrete, verified win is real in-app following. The scope for this branch:

- Add a second, separate official-OAuth login (system browser + local loopback redirect, PKCE), used **only** to enable real follow/unfollow.
- Keep the existing unofficial cookie-based auth exactly as it is today for everything else (playback, likes-fallback, browsing, search, homepage curation).
- The `FollowButton` component gains a real, working state when official OAuth is connected; falls back to today's "open on soundcloud.com" behavior when it isn't.
- Like stays fallback-only regardless, no official endpoint exists for it.

This is intentionally narrow. A broader migration (moving reads like likes-list/playlists/followers over to official OAuth too) was considered and rejected for this pass: the unofficial API already serves those reliably, and migrating them would add a second auth system's worth of complexity for zero new capability, since nothing about those reads is currently broken or blocked.

## Architecture

### The client_secret problem, and the actual fix

Every version of this plan needs a `client_secret` at token-exchange time, confirmed required via live testing (a refresh attempt without it returns `401`). The first draft of this document said "embed it as a constant in the binary" and called that an accepted tradeoff. That was wrong, and worth being explicit about why: this repo is public and open source. A hardcoded secret would not just be extractable from a compiled binary with effort, it would be sitting in plain text in the committed source, scraped by bots within minutes of being pushed. CI-injecting it at build time (the pattern already used for the updater's signing key) is better but still insufficient on its own here, the secret still ends up as a literal string inside every distributed binary, extractable by anyone willing to run `strings` on it.

The actual, correct fix, and what this branch implements: **the token exchange happens on a small server, not in the app.** See `services/oauth-proxy/`, a Cloudflare Worker that holds `SC_CLIENT_SECRET` as a Worker environment variable, never committed anywhere, and does nothing except forward a PKCE authorization code (or refresh token) to SoundCloud's real token endpoint and hand back the result. The app completes the PKCE dance itself (the `code_verifier` never leaves the app), and only ever sends the resulting authorization code to this proxy. **No secret of any kind exists in the app's source or its compiled binary.** Full detail and deployment steps in `services/oauth-proxy/README.md`.

This does mean depending on a small piece of infrastructure the developer runs (Cloudflare's free tier easily covers the expected volume), and login/refresh fail if that infrastructure is down, which the unofficial auth path is entirely unaffected by since it doesn't touch this proxy at all.

### Backend

- New module, `src-tauri/src/official_oauth.rs`: PKCE generation, local loopback listener (`tiny_http`, matching `tools/sc-probe/src/oauth.rs`), opens the system's default browser (not an embedded Tauri webview, a deliberate UX difference from the existing login, the user completes this in their normal browser, likely already logged into SoundCloud there) to `https://secure.soundcloud.com/authorize`. On redirect, sends the resulting `code` + `code_verifier` + `redirect_uri` to the oauth-proxy Worker's `/token/exchange`, not to SoundCloud directly, and never touches `client_secret`.
- Token storage: `access_token` + `refresh_token` + expiry timestamp, stored as a JSON blob in a new OS keychain entry, separate from the existing unofficial `oauth_token` entry. Refreshed proactively before expiry (tokens last ~3599s) via the proxy's `/token/refresh`.
- `client_id` only (not the secret) is a plain constant in the app, this one is fine to be public, it identifies the app, not a credential.
- New Tauri commands: `start_official_login`, `is_official_connected`, `sc_follow_user_v2`, `sc_unfollow_user_v2` (official-API versions, distinct from the existing unofficial `sc_*` commands which are untouched).

### Frontend

- `FollowButton.svelte`: when official OAuth is connected, clicking Follow/Unfollow calls the real backend commands and updates state immediately, no more redirecting to soundcloud.com. When not connected, same fallback behavior as today, plus a prompt to connect.
- A small, one-time explanatory moment the first time a user would benefit from connecting (not a forced gate, following is optional), since this is a second, separate login most users won't expect.

## Risks

- **Proxy availability.** Official login/refresh depends on the Worker being up. Cloudflare's uptime is very good and the free tier has generous headroom, but it's a new dependency that didn't exist before. The unofficial auth path is entirely independent of it.
- **Two separate login flows in one app.** Real added complexity and a UX surface that needs to be clear about what each login is for (the existing one: everything, this new one: real following). Mitigated by keeping it opt-in and scoped to the one feature it unlocks.
- **SoundCloud could remove the follow-write endpoint too**, same as they apparently did for likes. No indication of this, but worth keeping the unofficial fallback code path intact rather than deleting it, so a regression degrades gracefully back to "open on soundcloud.com" instead of breaking.

## Rollback

Everything here is additive and isolated to this worktree/branch. If it doesn't work out, the branch is simply not merged. Nothing on `main` changes until this is tested and deliberately merged.
