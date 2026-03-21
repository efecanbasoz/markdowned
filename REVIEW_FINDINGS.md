# Security & Code Quality Review Findings

**Date**: 2026-03-21
**Reviewers**: Codex CLI (GPT-5.4, read-only sandbox) + Claude Opus 4.6
**Scope**: Full codebase (src/, src-tauri/src/, ~3,679 LOC)

## Summary

| Severity | Count | Fixed |
|----------|-------|-------|
| HIGH | 7 | 6 |
| MEDIUM | 9 | 7 |
| LOW | 1 | 0 |
| INFO | 3 | 0 |
| **Total** | **20** | **13** |

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

## Remaining (require deeper refactoring)

- **SEC-001 (HIGH)**: TOCTOU race in validate_within_workspace — needs capability-based file access (cap_std crate)
- **QA-002 (HIGH)**: Completion streaming not request-correlated — needs requestId in both Rust and Svelte
- **QA-006-015**: Performance (sync I/O), type safety, testing gaps

## Positive Findings

- No shell/command injection surface in Rust commands
- CSP blocks inline scripts (`script-src 'self'`)
- ammonia HTML sanitization with explicit whitelist
- OS keychain integration for credential storage
- No active RustSec advisories for current dependency versions
- No unsafe blocks in Rust code
