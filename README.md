# SoundCloud Desktop (unofficial)

A lightweight, open-source, cross-platform (Windows/macOS/Linux) desktop client for SoundCloud, built with Tauri.

## Disclaimer

This project is **not affiliated with, endorsed by, or sponsored by SoundCloud**. It relies on:
- SoundCloud's official public API (OAuth 2.1 + PKCE) where available. As of this writing, SoundCloud's own `/authorize` flow has a known upstream bug (see `soundcloud/api` issues #513 and #393) that blocks user login via the official flow.
- As a fallback, an unofficial technique (also used by tools like `scdl`/`soundcloud-dl`): a `client_id` scraped from SoundCloud's public web app bundle, plus the user's own `oauth_token` session cookie, used only to access that user's own account (their likes, playlists, and streaming) — the same access their browser already has when logged into soundcloud.com.

This app is **streaming-only**. It does not download, save, export, or otherwise persist audio files, in order to stay within SoundCloud's API Terms of Use.

Use at your own risk. SoundCloud's unofficial endpoints are undocumented and may change or be revoked at any time.

## Development

```
npm install
npm run tauri dev
```

## Verification tooling

`tools/sc-probe` is a standalone CLI used during development to verify SoundCloud API behavior against the live service before building app features on top of it. It is not shipped as part of the app.
