# Gtest Report

## Target Workspace
`/Users/luisa/agent-project/agelens`

## Commands Run
- `cargo fmt -- --check`
- `cargo clippy --release --all-targets -- -D warnings`
- `cargo build --release`
- `cargo test --release --workspace`

## Failing Cases
- Initial workspace test failed because unit tests tried to call a Sails macro-exported method directly. Fixed by moving threshold logic into a pure helper and keeping the Sails route as a thin wrapper.
- Initial gtest compile failed because generated DTO types live under `age_lens`, not `age_lens::io`, and `CodeId` needed an explicit import.
- During the v0.2 receipt update, gtest initially compared receipt `caller` to raw `[u8; 32]` bytes. Fixed by comparing to `ActorId`.

## Fix Summary
- Added generated client crate `age-lens-client`.
- Added `tests/gtest.rs` that deploys AgeLens in `GtestEnv` and calls `CalculateAge`, `CheckAgeThreshold`, and `CheckAgeDaysThreshold`.
- Added `records_and_verifies_calculation_receipt_in_gtest`, covering `RecordCalculation`, `CalculationRecorded`, `GetCalculation`, `VerifyCalculation`, and `CalculationCount`.
- Preserved query-only paths for callers that do not need provenance.

## Final Green State
`cargo test --release --workspace` passes:
- 3 gtest tests passed.
- 8 app unit tests passed.

## Remaining Gaps
- No local-node smoke run yet.
