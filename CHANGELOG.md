# Changelog

## [0.1.0] - 2026-04-23

### Added
- Interactive, filterable script picker for `package.json` scripts
- Auto-detects package manager from the `packageManager` field (`pnpm@x`, `npm`, `yarn`, `bun`)
- Falls back to an interactive prompt to choose a package manager when `packageManager` is not set
- Replaces the current process via `exec` so the script runs as if invoked directly (Unix)
