<p align="center">
  <img src="assets/logo.png" alt="SoundKitten logo" width="120" />
</p>

<h1 align="center">SoundKitten</h1>

<p align="center">
  A lightweight, open-source, cross-platform desktop client for SoundCloud.<br />
  Built with <a href="https://tauri.app">Tauri</a> + Rust + Svelte, because the official app shouldn't need Electron to play music.
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License" /></a>
  <img src="https://img.shields.io/badge/platforms-Windows%20%7C%20macOS%20%7C%20Linux-informational" alt="Windows, macOS, Linux" />
</p>

<p align="center">
  <img src="assets/home-banner.gif" alt="SoundKitten's animated Home banner: a cat running through a forest scene" width="420" />
</p>

---

## Why

SoundCloud dropped their native Windows desktop app entirely. On Windows 11, the official "app" is just the web player installed as a Chrome PWA: no real desktop integration, nothing a browser tab wasn't already doing, and you need Chrome installed at all. SoundKitten is an actual native app, with a Rust backend, a native OS webview instead of a bundled browser, and a UI that matches soundcloud.com rather than reinventing it.

## Features

- Search, stream, like, comment, and manage playlists (create, rename, add/remove tracks) just like the web app
- Home feed with SoundCloud's own curated sections (Trending by genre, personalized mixes, etc.), plus an animated banner scene synced to the real time of day and season -- toggle it off for a plain header instead
- A compact mini player (toggle from the app or the system tray) with SoundCloud's real per-track waveform, draggable and resizable
- Shuffle and repeat, click-to-play or drag-to-reorder the queue, a smart Previous button (restarts the current track first, skips back on a second press), and playback state that survives an app restart
- Keeps playing related tracks once your queue runs out, the same "keep listening" behavior the official app has
- Clean, native window chrome, no bundled browser UI, no bloat
- Auto-updates, checked on launch (opt-in per update, never silent)

<p align="center">
  <img src="assets/mini-player.gif" alt="SoundKitten mini player" width="360" />
</p>

## Download

Grab the latest installer for your platform from [Releases](../../releases). Windows (`.msi`/`.exe`), macOS (`.dmg`, Apple Silicon and Intel), and Linux (`.AppImage`/`.deb`) builds are published there.

## How it works

SoundCloud doesn't offer a fully-functional public API for third-party apps, and their official OAuth flow has a long-standing upstream bug (see [`soundcloud/api` #513](https://github.com/soundcloud/api/issues/513) and [#393](https://github.com/soundcloud/api/issues/393)). SoundKitten works around this with two separate, narrowly-scoped logins:

- **Browsing and streaming** uses the same technique browser extensions and tools like `scdl` use: a `client_id` scraped from SoundCloud's own public web app bundle, combined with the user's own `oauth_token` session cookie, which is the same access their browser already has when logged into soundcloud.com.
- **Likes, follows, and playlist management** go through a real OAuth 2.1 + PKCE login instead, since SoundCloud's anti-bot layer blocks those specific write endpoints on the scraped-cookie path. The token exchange happens through a small proxy the developer controls, which is the only place the app's OAuth client secret ever exists; it never ships in the app itself.

Aside from that one token exchange, every request goes straight to SoundCloud's own API; no credentials are sent anywhere else.

**This app is streaming-only.** It does not download, save, export, or otherwise persist audio files, to stay within SoundCloud's API Terms of Use. DRM-protected tracks (major-label content served over encrypted HLS) are detected and clearly marked as unplayable, with a link out to soundcloud.com, rather than pretending they might work.

## Disclaimer

This project is **not affiliated with, endorsed by, or sponsored by SoundCloud**. Browsing and streaming rely on undocumented, unofficial API endpoints; SoundCloud has confirmed directly that this violates their Terms of Service, with account suspension as a possible consequence, and that these endpoints may also change or be revoked at any time regardless. Use at your own risk.

## Development

Requires [Node.js](https://nodejs.org) and the [Rust toolchain](https://www.rust-lang.org/tools/install), plus Tauri's [platform prerequisites](https://tauri.app/start/prerequisites/).

```sh
npm install
npm run tauri dev
```

Run the test suite:

```sh
npm run test    # frontend (Vitest)
npm run check   # type-check
cargo check --manifest-path src-tauri/Cargo.toml
```

### Verification tooling

`tools/sc-probe` is a standalone CLI used during development to verify SoundCloud API behavior against the live service before building app features on top of it. It's not shipped as part of the app.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get set up and what to expect from the PR process.

## License

[MIT](LICENSE)
