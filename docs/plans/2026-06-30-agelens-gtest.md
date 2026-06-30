# Gtest Report

## Target Workspace
`/Users/luisa/agent-project/agelens`

## Commands Run
- `cargo fmt --all`
- `cargo test --release --workspace`
- `cargo build --release`

## Failing Cases
- Initial workspace test failed because unit tests tried to call a Sails macro-exported method directly. Fixed by moving threshold logic into a pure helper and keeping the Sails route as a thin wrapper.
- Initial gtest compile failed because generated DTO types live under `age_lens`, not `age_lens::io`, and `CodeId` needed an explicit import.

## Fix Summary
- Added generated client crate `age-lens-client`.
- Added `tests/gtest.rs` that deploys AgeLens in `GtestEnv` and calls `CalculateAge`, `CheckAgeThreshold`, and `CheckAgeDaysThreshold`.
- Kept privacy-sensitive data out of events and state.

## Final Green State
`cargo test --release --workspace` passes:
- 2 gtest tests passed.
- 6 app unit tests passed.

## Remaining Gaps
- No local-node smoke run yet.
- Local-node smoke has not been run yet.
