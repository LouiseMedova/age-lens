# Task Plan

## Goal

Address VAN project review `6` by adding auditable calculation receipts to AgeLens.

## Preconditions

- AgeLens source is pushed to `https://github.com/LouiseMedova/age-lens`.
- Current review outcome is `NeedsChanges`.
- Do not deploy until revised guidance is `Proceed` and Stage 2a code/deploy approval is explicit.

## Ordered Tasks

1. Add receipt DTOs: `CalculationRequest`, `CalculationResult`, `CalculationReceipt`, and `CalculationRecorded`.
2. Add program-owned `AgeLensState` with `next_calculation_id` and receipt map.
3. Update `Program` and `AgeLensService` to pass state by `RefCell`.
4. Implement `RecordCalculation`, `GetCalculation`, `VerifyCalculation`, and `CalculationCount`.
5. Preserve existing query behavior.
6. Add unit tests for record/verify edge cases.
7. Add gtest coverage for receipt recording, event emission, retrieval, and verification.
8. Regenerate generated client and IDL.
9. Update README, `SKILLS.md`, stable `idl/age_lens.idl`, and VAN review notes.
10. Run `cargo fmt`, `cargo clippy --release --all-targets -- -D warnings`, `cargo build --release`, and `cargo test --release --workspace`.
11. Commit and push.

## Dependencies

- `sails-rs` generated client path through existing `build.rs`.
- `sails_rs::cell::RefCell` and `sails_rs::collections::BTreeMap`.
- `sails_rs::gstd::msg` for caller identity in receipts.

## Verification Steps

- Unit tests assert receipt ids, count, get, verify true, verify false for mismatches, invalid input rejection, and no count increase on error.
- Gtest asserts record/get/verify behavior and observes `CalculationRecorded`.
- IDL diff shows the new methods, types, and event.

## Review Checkpoints

- Confirm no existing public method signatures changed.
- Confirm receipt design directly answers reviewer guidance: stateful storage, `calculation_id`, `VerifyCalculation`, event, and provenance call chain.
- Confirm docs warn that AgeLens still does not prove the truth of a supplied date.

## Rollback Notes

Before deployment, rollback is a normal git revert. After deployment, use a new program version and VAN application transition flow.
