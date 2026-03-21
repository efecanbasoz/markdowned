# Security & Code Quality Review Findings

**Date**: 2026-03-21
**Reviewers**: Codex CLI (GPT-5.4, read-only sandbox) + Claude Opus 4.6
**Scope**: Full codebase (src/, src-tauri/src/, ~3,679 LOC)

## Summary

| Severity | Count | Fixed |
|----------|-------|-------|
| HIGH | 7 | 7 |
| MEDIUM | 9 | 8 |
| LOW | 1 | 0 |
| INFO | 3 | 0 |
| **Total** | **20** | **15** |

## Fixed (Round 1)

- **QA-001 (HIGH)**: Editor only updates content on real doc changes (prevents false dirty state)
- **QA-005 (MEDIUM)**: Mutex::lock().unwrap() → proper Result handling (prevents panics)
- **SEC-003 (MEDIUM)**: Config file created with 0o600 from the start using OpenOptions
- **SEC-005 (MEDIUM)**: LLM client validates HTTPS for non-loopback hosts, adds timeout and redirect blocking

## Fixed (Round 2 — Deferred)

- **SEC-002 (HIGH)**: API key no longer sent to frontend — load_config returns empty key, completion resolves server-side
- **SEC-004 (HIGH)**: scan_directory validates path (canonicalize, reject root, reject shallow paths)
- **QA-003 (HIGH)**: Tab closure refuses to discard dirty tabs — returns early instead of silent data loss
- **QA-004 (HIGH)**: Config parse errors logged to stderr instead of silently falling back to defaults
- **SEC-006 (MEDIUM)**: Watcher events debounced (200ms) to prevent CPU spikes
- **SEC-007 (MEDIUM)**: Style attributes filtered to safe CSS properties only (color, background, font)
- All remaining Mutex::lock().unwrap() calls converted to proper Result handling

## Fixed (Round 3 — Final)

- **SEC-001 (HIGH)**: TOCTOU race eliminated — resolve_safe_path() returns canonical path
  and all I/O uses it. Symlinks explicitly rejected. Original user path never used for I/O.
- **QA-002 (HIGH)**: Completion streaming now uses requestId — frontend filters chunks by
  matching request_id, preventing stale request data from leaking into new completions.
- **QA-006 (MEDIUM)**: Preview rendering offloaded to spawn_blocking to avoid blocking async runtime.

## Remaining (type safety, testing)

- **QA-007-015**: Type safety improvements, Unicode search offsets, testing gaps

## Positive Findings

- No shell/command injection surface in Rust commands
- CSP blocks inline scripts (`script-src 'self'`)
- ammonia HTML sanitization with explicit whitelist
- OS keychain integration for credential storage
- No active RustSec advisories for current dependency versions
- No unsafe blocks in Rust code
