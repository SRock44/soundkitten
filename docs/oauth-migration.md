# Migrating to official OAuth 2.1 + PKCE

Status: proposed, not started. Written 2026-08-09 after confirming live that SoundCloud's official OAuth flow now works end to end (see "What's confirmed" below). Not yet implemented in the shipped app; only verified in `tools/sc-probe`.

## Summary

SoundKitten currently authenticates two ways:

1. **Unofficial (shipped today).** A `client_id` scraped from soundcloud.com's public JS bundles, paired with the user's own `oauth_token` session cookie, extracted from a Tauri login webview. This is what every user of the app uses right now.
2. **Official OAuth 2.1 + PKCE.** Registered app, real `client_id`/`client_secret`, standard authorization code + PKCE flow against `secure.soundcloud.com`. This was believed broken (SoundCloud's own `/authorize` page rendered blank) until today. It turned out to be a redirect URI registration mismatch on our end, not a platform bug. Once fixed, it passed all 6 verification checks live: authorization code exchange, `/me`, `/me/likes/tracks`, `/me/playlists`, `/me/tracks`, and refresh token exchange.

This document proposes migrating the shipped app from (1) to (2), fully or partially depending on what a short verification pass turns up.

## Why migrate

### Confirmed benefits

- **No more client_id scraping.** The unofficial path depends on parsing soundcloud.com's HTML for script tags and regex-matching a `client_id` out of a JS bundle. This broke once already this session when soundcloud.com's layout or availability changed, and separately caused a real IP block when a request burst against it tripped SoundCloud's bot detection. Official OAuth needs no scraping at all.
- **No more cookie extraction.** The unofficial login flow opens an embedded webview, polls its cookie jar until an `oauth_token` cookie appears, and stores that value. It works, but it is inherently fragile (depends on SoundCloud's cookie name/behavior never changing) and requires embedding a full login webview in the app. Official OAuth uses a real, documented, versioned protocol.
- **Real token lifecycle.** The unofficial path stores one opaque cookie value with no defined expiry or refresh mechanism, if it stops working there's no way to know why or recover except asking the user to log in again. Official OAuth gives a real `access_token` (expires in ~3599s) and `refresh_token`, so expiry is predictable and recoverable without user interaction.
- **Sanctioned traffic pattern.** A registered OAuth client authenticating through SoundCloud's own documented flow looks nothing like scraped-credentials traffic to SoundCloud's abuse detection. This alone may reduce the odds of future IP-level blocks, independent of the DataDome question below.

### Likely benefit, needs verification before relying on it

- **Might bypass the DataDome block on writes.** Liking and following a track/user currently 403 with a DataDome CAPTCHA challenge when done through the unofficial API (`api-v2.soundcloud.com`). That's why the app currently has a "open on soundcloud.com" fallback for both instead of a working in-app button. It is **not confirmed** whether the official API (`api.soundcloud.com`, a different host) is subject to the same protection. If it isn't, this migration would let us ship real in-app liking and following. If it is, this benefit doesn't materialize and those two features stay fallback-only regardless of migrating.

### What migrating does NOT obviously get us

The official API is much narrower in scope than the unofficial one. Endpoints we rely on today that have **not been confirmed to exist on the official API** include: track streaming/transcoding resolution (the single most important one, this is how audio actually gets played), search, the personalized feed (`/stream`), homepage curation (`/mixed-selections`), system playlists ("Your Mix N"), followers/following lists, and comments. Until each of these is checked individually, assume the official API cannot fully replace the unofficial one, and this will end up a hybrid rather than a full replacement.

## Current architecture (unofficial, shipped today)

- `src-tauri/src/soundcloud/mod.rs`: scrapes and caches a `client_id` from soundcloud.com (now properly singleflight-locked so concurrent requests share one scrape), with a persisted manual override as a fallback for when soundcloud.com itself is unreachable.
- `src-tauri/src/auth.rs`: opens a login webview pointed at `https://soundcloud.com/login`, polls `cookies_for_url` for an `oauth_token` cookie, stores it in the OS keychain.
- All API calls go to `https://api-v2.soundcloud.com`, with `client_id` as a query parameter and `Authorization: OAuth <cookie value>` as the header.

## What's confirmed about the official flow (via `tools/sc-probe/src/oauth.rs`)

- Authorize endpoint: `https://secure.soundcloud.com/authorize`
- Token endpoint: `https://secure.soundcloud.com/oauth/token`
- API base: `https://api.soundcloud.com` (note: different host from the unofficial `api-v2.soundcloud.com`)
- Auth header format: `Authorization: Bearer <access_token>` (standard OAuth2 Bearer scheme, different from the unofficial API's custom `Authorization: OAuth <token>` scheme, code cannot be shared as-is between the two)
- Redirect URI: `http://127.0.0.1:{port}/callback` (loopback, RFC 8252 native-app pattern), must be registered exactly (scheme, host, port, path) at soundcloud.com/you/apps for the specific `client_id` in use
- Token exchange currently sends `client_secret` in the POST body, both for the initial exchange and for refresh
- Live-verified working end to end today: authorization code + PKCE exchange, `GET /me`, `GET /me/likes/tracks`, `GET /me/playlists`, `GET /me/tracks`, refresh token exchange

## Critical open questions (verify before committing to a migration shape)

These determine whether this is a full replacement, a hybrid, or narrower than either. All are cheap to check live using `tools/sc-probe`'s existing OAuth flow (add a check, or extend `run_oauth_checks`) before writing any app code.

1. **Does official likes/follows bypass DataDome?** Try `PUT` to like a track and follow a user through `api.soundcloud.com` with a real access token. This is the single most decision-relevant question, since it's the main reason to migrate beyond "less fragile."
2. **Is `client_secret` actually enforced on token exchange?** Try the same exchange with the field omitted. If SoundCloud accepts a PKCE-only exchange, the "secret embedded in every shipped copy of the app" risk (see Risks) disappears entirely.
3. **Does the official API expose track streaming/transcoding data at all?** This is the load-bearing one. If there's no way to resolve a playable stream URL through the official API, playback has to stay on the unofficial path no matter what else migrates.
4. **Does the official API have search, `/stream` (feed), `/mixed-selections`, or system playlists?** Check each. Assume "no" until confirmed, the official API was historically a much smaller surface than the unofficial one used by soundcloud.com's own web app.
5. **Does the official API expose comments, followers/following lists, and playlist detail (hydrated tracks)?** Same as above, check rather than assume.
6. **Can the official `access_token` be used as the `Authorization: OAuth <token>` value against the unofficial `api-v2.soundcloud.com`, in place of the scraped cookie?** If yes, this is the best possible outcome: one login flow (official OAuth), no more cookie-webview fragility, but still hitting `api-v2` for whatever the official API doesn't cover, using the same token everywhere.

## Proposed migration phases

Each phase ends with a live check against the real API before moving on, consistent with how the rest of this project has been built.

### Phase 0: Answer the open questions above

Extend `tools/sc-probe`'s `oauth` subcommand (or add throwaway probe binaries, cleaned up after, per existing project convention) to test all six items above. Write down the actual answers. This phase decides everything after it, don't skip it.

### Phase 1: Backend OAuth client in the real app

Port the working logic from `tools/sc-probe/src/oauth.rs` into `src-tauri/src/auth.rs` (or a new `src-tauri/src/oauth.rs` module):
- PKCE verifier/challenge generation
- A local loopback HTTP server for the redirect (a lightweight crate like `tiny_http`, same as sc-probe uses, or reuse if already a dependency)
- Opening the redirect URL in the **system's default browser**, not an embedded Tauri webview. This is a real UX change from today's in-app login popup: the user gets sent to their normal browser, which is likely already logged into SoundCloud, then bounced back to the app. This is the standard, recommended pattern for native apps (RFC 8252) and should be a smoother login than today's embedded webview in practice, but it is a visible behavior change worth calling out to users.
- Token exchange and refresh, storing `access_token`, `refresh_token`, and an expiry timestamp in the OS keychain (currently only a single opaque cookie value is stored, this needs a small schema change, e.g. a JSON blob instead of a bare string)
- Refresh-before-expiry logic, similar in spirit to the existing "retry once on 401 with a fresh client_id" pattern already in `authed_get`

### Phase 2: API client changes

Depends entirely on Phase 0's answers. At minimum, add an official-API request path (`api.soundcloud.com`, Bearer auth) alongside the existing unofficial one. If Phase 0 question 6 comes back yes, this simplifies a lot: the unofficial path keeps working exactly as it does today, just swap the cookie value for the official access token, no scraping, no webview.

### Phase 3: Migrate write actions (likes, follows)

If Phase 0 question 1 confirms official writes bypass DataDome, switch `like_track`/`unlike_track`/follow-equivalent calls to the official API and remove the "open on soundcloud.com" fallback buttons, replacing them with real in-app actions.

### Phase 4: Migrate what else the official API covers

Endpoint by endpoint, based on Phase 0's findings. Whatever isn't covered stays on the unofficial path.

### Phase 5: Rollout

- Keep the unofficial path as a fallback for existing users (don't force a re-login on update if avoidable).
- New logins go through official OAuth.
- Consider a manual "re-authenticate" action in Settings for existing users to opt into the new flow.

## Risks and mitigations

**Client secret shipped in every copy of the app.** If Phase 0 confirms the secret is required, every installed copy of SoundKitten embeds the same `client_secret`, extractable by anyone who inspects the binary. This is a known weakness in how some OAuth providers (including, apparently, SoundCloud) handle "public clients" that should really only need PKCE. The actual blast radius is limited: a leaked secret lets someone impersonate the app's *client* identity to SoundCloud, it does not by itself grant access to any user's account, since each user still has to complete their own login/consent. Still worth minimizing exposure if possible (checking whether it's actually required is Phase 0's job).

**UX change on login.** Moving from an embedded webview to the system browser is a visible change. Should be communicated clearly (a one-time explanatory screen the first time a user hits the new login flow) rather than silently swapped.

**Feature gaps if the official API is narrower than assumed.** Mitigated by Phase 0 answering this before any app code changes, and by the hybrid approach (Phase 2 onward) rather than a hard cutover.

**Registered redirect URI is tied to one client_id, used by every install.** Since every user runs the app locally against the same registered `client_id`, and the redirect is a fixed loopback URL (`http://127.0.0.1:{port}/callback`), this works the same way for every install (each user's own local port, standard native-app pattern), no per-user registration needed. Worth confirming the chosen port doesn't commonly collide with other local services.

## Rollback plan

Since the unofficial path isn't being deleted, only supplemented, rollback is just: stop routing new logins through official OAuth, keep serving everyone through the existing cookie-based flow. No data migration to undo.
