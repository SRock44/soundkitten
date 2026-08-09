# Migrating to official OAuth 2.1 + PKCE

Status: Phase 0 (verification) complete, implementation in progress on the `worktree-oauth-migration` branch, kept out of `main` until tested. Written 2026-08-09, updated same day after live-verifying every open question below, corrected again same day after the like-write endpoint was found to actually work (see note below).

**Correction (2026-08-09, same day):** an earlier version of this document concluded like-write did not exist on the official API, based on an exhaustive sweep of 11 candidate endpoint shapes. That sweep tried the right paths but only ever tried `PUT` as the HTTP method. The real endpoint is `POST`/`DELETE /likes/tracks/{id}` (and `POST`/`DELETE /likes/playlists/{id}` for playlists), documented in SoundCloud's own OpenAPI spec, correctly identified by the developer, and now confirmed live: both return clean `200 OK` on a real account. This was a research gap, not a platform limitation. The "scoped migration" framing below is now wrong and has been replaced.

## Summary

SoundKitten currently authenticates two ways:

1. **Unofficial (shipped today, main branch).** A `client_id` scraped from soundcloud.com's public JS bundles, paired with the user's own `oauth_token` session cookie, extracted from a Tauri login webview. This is what every user of the app uses right now.
2. **Official OAuth 2.1 + PKCE.** Registered app, real `client_id`/`client_secret`, standard authorization code + PKCE flow against `secure.soundcloud.com`. Believed broken until 2026-08-09, turned out to be a redirect URI registration mismatch on our end, not a platform bug.

Phase 0 answered every open question this document originally raised (see below). The conclusion: **this is not a full replacement for the unofficial API**, playback and homepage curation still have to stay on the unofficial path, since the official API only serves 30-second previews and doesn't expose `/mixed-selections`. But both write actions that currently don't work at all in-app (DataDome-blocked on the unofficial API), **liking tracks/playlists and following users**, work cleanly through official OAuth. That's the target for this migration: fix both broken buttons, leave playback/browsing/search alone.

## Phase 0 results (live-verified 2026-08-09)

| Question | Answer |
|---|---|
| Does official likes/follows bypass DataDome? | **Both: yes, confirmed live.** Follow: `PUT /me/followings/{id}` and `POST /me/followings/{id}` both return clean 200/201 with real data (immediately unfollowed after each test to leave no trace). Like: `POST /likes/tracks/{id}` and `DELETE /likes/tracks/{id}` both return clean `200 OK`, and the same shape works for playlists too (`POST`/`DELETE /likes/playlists/{id}`), confirmed and immediately restored to leave no trace. An initial sweep of 11 path/method combinations (mostly guessing `PUT` against various plausible paths) all 405'd and wrongly looked like proof the endpoint didn't exist, but the actual issue was the HTTP method, not the path. `/likes/tracks/{id}` was in that first sweep, just tried with `PUT` instead of `POST`/`DELETE`. Re-tested with the correct verbs (per SoundCloud's own OpenAPI spec) and it works cleanly, plain numeric id, no URN needed. |
| Is `client_secret` actually enforced? | **Yes.** Refresh-token exchange without it returns `401 Unauthorized`. The "secret ships in every copy of the app" risk is real, not hypothetical. |
| Does the official API expose streaming? | **Only 30-second previews.** `GET /tracks/{id}` returns a `stream_url` field, and it resolves (HTTP 200, real MP3 bytes), but response headers confirm `x-media-interval: 0-30` and `Content-Length` matches ~30s of audio at the given bitrate. There is no `media`/transcodings field like the unofficial API has. **Playback must stay on the unofficial path, full stop.** |
| Search, feed, mixed-selections, system playlists? | Search (`GET /tracks?q=`) and feed (`GET /me/activities`) both work. `/mixed-selections` does not exist (405 unknown route, same signature as the missing like endpoint). System playlists were not separately tested but given the pattern, assume absent. |
| Comments, followers/following, playlist detail? | Comments (`GET /tracks/{id}/comments`), followers, and followings all work fine on the official API. |
| Can the official access_token be reused against the unofficial api-v2 host? | **No.** `403 Forbidden` when tried. These are fully separate token systems; there is no way to get "one login, works everywhere." Adopting official OAuth for anything means running two auth systems side by side, not replacing one with the other. |

## Decision: scoped migration, not a full one

Given the above, migrating everything would mean losing playback and homepage curation entirely, not an option. But the concrete, verified win now covers both of the app's broken write actions, not just one. The scope for this branch:

- Add a second, separate official-OAuth login (system browser + local loopback redirect, PKCE), used to enable real like/unlike (tracks and playlists) and real follow/unfollow.
- Keep the existing unofficial cookie-based auth exactly as it is today for everything else (playback, browsing, search, homepage curation, and all reads: likes-list, playlists, followers).
- `LikeButton` and `FollowButton` both gain a real, working state when official OAuth is connected; fall back to today's "open on soundcloud.com" behavior when it isn't.

This is intentionally still narrow on the read side. A broader migration (moving reads like likes-list/playlists/followers over to official OAuth too) was considered and rejected for this pass: the unofficial API already serves those reliably, and migrating them would add a second auth system's worth of complexity for zero new capability, since nothing about those reads is currently broken or blocked. Only the two writes that are actually broken today move to the official path.

## Architecture

### The client_secret problem, and the actual fix

Every version of this plan needs a `client_secret` at token-exchange time, confirmed required via live testing (a refresh attempt without it returns `401`). The first draft of this document said "embed it as a constant in the binary" and called that an accepted tradeoff. That was wrong, and worth being explicit about why: this repo is public and open source. A hardcoded secret would not just be extractable from a compiled binary with effort, it would be sitting in plain text in the committed source, scraped by bots within minutes of being pushed. CI-injecting it at build time (the pattern already used for the updater's signing key) is better but still insufficient on its own here, the secret still ends up as a literal string inside every distributed binary, extractable by anyone willing to run `strings` on it.

The actual, correct fix, and what this branch implements: **the token exchange happens on a small server, not in the app.** See `services/oauth-proxy/`, a Cloudflare Worker that holds `SC_CLIENT_SECRET` as a Worker environment variable, never committed anywhere, and does nothing except forward a PKCE authorization code (or refresh token) to SoundCloud's real token endpoint and hand back the result. The app completes the PKCE dance itself (the `code_verifier` never leaves the app), and only ever sends the resulting authorization code to this proxy. **No secret of any kind exists in the app's source or its compiled binary.** Full detail and deployment steps in `services/oauth-proxy/README.md`.

This does mean depending on a small piece of infrastructure the developer runs (Cloudflare's free tier easily covers the expected volume), and login/refresh fail if that infrastructure is down, which the unofficial auth path is entirely unaffected by since it doesn't touch this proxy at all.

### Backend

- New module, `src-tauri/src/official_oauth.rs`: PKCE generation, local loopback listener (`tiny_http`, matching `tools/sc-probe/src/oauth.rs`), opens the system's default browser (not an embedded Tauri webview, a deliberate UX difference from the existing login, the user completes this in their normal browser, likely already logged into SoundCloud there) to `https://secure.soundcloud.com/authorize`. On redirect, sends the resulting `code` + `code_verifier` + `redirect_uri` to the oauth-proxy Worker's `/token/exchange`, not to SoundCloud directly, and never touches `client_secret`.
- Token storage: `access_token` + `refresh_token` + expiry timestamp, stored as a JSON blob in a new OS keychain entry, separate from the existing unofficial `oauth_token` entry. Refreshed proactively before expiry (tokens last ~3599s) via the proxy's `/token/refresh`.
- `client_id` only (not the secret) is a plain constant in the app, this one is fine to be public, it identifies the app, not a credential.
- New Tauri commands: `start_official_login`, `is_official_connected`, `sc_follow_user_v2`, `sc_unfollow_user_v2`, `sc_like_track_v2`, `sc_unlike_track_v2`, `sc_like_playlist_v2`, `sc_unlike_playlist_v2` (official-API versions, distinct from the existing unofficial `sc_*` commands which are untouched).

### Frontend

- `FollowButton.svelte` and `LikeButton.svelte`: when official OAuth is connected, clicking calls the real backend commands and updates state immediately, no more redirecting to soundcloud.com. When not connected, same fallback behavior as today, plus a prompt to connect.
- A small, one-time explanatory moment the first time a user would benefit from connecting (not a forced gate, this is opt-in), since this is a second, separate login most users won't expect.

## Risks

- **Proxy availability.** Official login/refresh depends on the proxy service (`services/oauth-proxy/`, deployed on the existing soundkitten swarm host) being up. The unofficial auth path is entirely independent of it, so this only affects the two write actions, not the app as a whole.
- **Two separate login flows in one app.** Real added complexity and a UX surface that needs to be clear about what each login is for (the existing one: everything, this new one: real likes/follows). Mitigated by keeping it opt-in and scoped to the two features it unlocks.
- **SoundCloud could remove or further lock down these write endpoints.** No indication of this, but worth keeping the unofficial fallback code path intact rather than deleting it, so a regression degrades gracefully back to "open on soundcloud.com" instead of breaking.

## Rollback

Everything here is additive and isolated to this worktree/branch. If it doesn't work out, the branch is simply not merged. Nothing on `main` changes until this is tested and deliberately merged.
