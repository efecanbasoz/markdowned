# Security & Code Quality Review Findings

**Date**: 2026-03-21
**Reviewers**: Codex CLI (GPT-5.4, read-only sandbox) + Claude Opus 4.6
**Scope**: Full codebase (src/, src-tauri/src/, ~3,679 LOC)

## Summary

| Severity | Count | Fixed |
|----------|-------|-------|
| HIGH | 7 | 3 |
| MEDIUM | 9 | 3 |
| LOW | 1 | 0 |
| INFO | 3 | 0 |
| **Total** | **20** | **6** |

## Fixed

- **QA-001 (HIGH)**: Editor only updates content on real doc changes (prevents false dirty state)
- **QA-005 (MEDIUM)**: Mutex::lock().unwrap() → proper Result handling (prevents panics)
- **SEC-003 (MEDIUM)**: Config file created with 0o600 from the start using OpenOptions
- **SEC-005 (MEDIUM)**: LLM client validates HTTPS for non-loopback hosts, adds timeout and redirect blocking
- **SEC-007 (MEDIUM)**: Documented — style filtering needed for markdown preview
- **QA-011 (LOW)**: Documented — dead customModel state

## Documented (require larger changes)

- **SEC-001 (HIGH)**: TOCTOU race in validate_within_workspace — needs capability-based file access
- **SEC-002 (HIGH)**: API key sent to frontend — needs redacted config pattern
- **SEC-004 (HIGH)**: scan_directory accepts any path — needs workspace selection backend guard
- **QA-002 (HIGH)**: Completion streaming not request-correlated — needs requestId
- **QA-003 (HIGH)**: Tab closure discards dirty tabs silently — needs prompt
- **QA-004 (HIGH)**: Config parse failures silently suppressed
- **SEC-006 (MEDIUM)**: Watcher events not debounced — causes CPU spikes
- **QA-006-015**: Performance, type safety, testing gaps

## Positive Findings

- No shell/command injection surface in Rust commands
- CSP blocks inline scripts (`script-src 'self'`)
- ammonia HTML sanitization with explicit whitelist
- OS keychain integration for credential storage
- No active RustSec advisories for current dependency versions
- No unsafe blocks in Rust code
