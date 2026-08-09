# Contributing to SoundKitten

Thanks for taking the time to contribute. This project relies on SoundCloud's unofficial/undocumented endpoints, so a lot of the day-to-day work is verification-driven. Please read the notes below before diving in.

## Before you start

- **For anything nontrivial, open an issue first.** A quick discussion up front saves a rewritten PR later, especially for anything touching auth, playback, or API behavior.
- **Bug fixes and small improvements** can go straight to a PR.
- Check existing issues/PRs first to avoid duplicate work.

## Development setup

Requires [Node.js](https://nodejs.org), the [Rust toolchain](https://www.rust-lang.org/tools/install), and Tauri's [platform prerequisites](https://tauri.app/start/prerequisites/).

```sh
npm install
npm run tauri dev
```

## Before opening a PR

- `npm run check`: type-checks the frontend
- `npm run test`: runs the frontend test suite (Vitest)
- `cargo check --manifest-path src-tauri/Cargo.toml`: checks the Rust backend compiles
- If you touched playback, auth, or anything hitting SoundCloud's API, **verify it live** against the real API before submitting. This project's history is full of endpoints that looked right but 404'd or behaved differently than documented elsewhere. Assume nothing about undocumented behavior without checking.

## Code style

- No unnecessary comments. Code should read clearly on its own. Comments are for *why*, not *what* (a non-obvious constraint, a workaround for a specific bug, something that would surprise a reader).
- Match the existing patterns in the file you're editing rather than introducing a new style.
- Keep PRs focused. A bug fix doesn't need a refactor riding along with it.

## Scope boundaries

A few things are intentionally out of scope, mostly for legal/ToS reasons:

- **No download/export functionality.** This app is streaming-only, to stay within SoundCloud's API Terms of Use. PRs adding download/save/export features will not be merged.
- **No DRM circumvention.** DRM-protected tracks (major-label content over encrypted HLS) are genuinely uncircumventable without a licensed CDM and are marked as such in the UI. That's by design, not a bug to fix.
- **No use of SoundCloud's actual logo/branding** in the app itself, for trademark reasons. The cat mascot is intentional.

## Reporting bugs / requesting features

Use the issue templates. They ask for the context that's actually useful here (repro steps, platform, whether it's an API/auth issue vs a UI issue).

## Pull requests

- Fill out the PR template.
- Keep commits reasonably organized (squashing a WIP history before opening the PR is appreciated, but not required, since we can squash on merge).
- CI (`build`) runs on all three platforms and must pass before merging.
- Be patient. This is maintained in spare time.
