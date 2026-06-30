# Architecture Note

## Summary

AgeLens becomes a small auditable receipt service while preserving the existing pure query surface. The new command path records calculation provenance on-chain and the query path lets verifiers check that a stored receipt matches supplied inputs and expected output.

## Program And Service Boundaries

`Program` owns one `RefCell<AgeLensState>` and exposes one service, `AgeLens`. `AgeLensService` borrows that state and contains the calculation, storage, verification, and event-emission logic.

## State Ownership

Use program-owned state, the default Sails pattern:

- `AgeLensState.next_calculation_id: u64`
- `AgeLensState.calculations: BTreeMap<u64, CalculationReceipt>`

The state is initialized by `Program::new()`. Tests deploy a fresh program per case, giving each test isolated receipt state.

## Message Flow

Pure query calls behave as before. For receipt flow:

1. Caller sends `AgeLens/RecordCalculation(request)`.
2. The service validates and computes the matching `CalculationResult`.
3. The service stores a `CalculationReceipt`.
4. The service emits `CalculationRecorded(receipt)`.
5. A downstream consumer can store or reference `calculation_id`.
6. A verifier calls `AgeLens/VerifyCalculation(calculation_id, inputs, expected)`.

## Routing And Public Interface

- Existing public routes that must remain stable: `CalculateAge`, `CheckAgeThreshold`, `CheckAgeDaysThreshold`, `Version`.
- New routes introduced by this release: `RecordCalculation`, `GetCalculation`, `VerifyCalculation`, `CalculationCount`.
- Any intentionally deprecated routes: none.
- Whether any method signature or reply shape changes are proposed: no existing signature changes.

## Event Contract

- Existing events that must remain stable: none.
- New event surface introduced by this release: `CalculationRecorded(CalculationReceipt)`.
- Whether any existing event payload changes are proposed: no.
- Whether event versioning is required: no, this is the first event surface.

## Generated Client Or IDL Impact

- This release requires IDL regeneration.
- The Rust generated client is consumed by `tests/gtest.rs`.
- `idl/age_lens.idl` must be refreshed from the release build output before publishing.

## Contract Version And Status Surface

- `AgeLens/Version` remains the semantic version surface.
- Version should move from `0.1.0` to `0.2.0` because the public interface expands.
- There is no lifecycle status surface.

## Off-Chain Components

- No frontend is required.
- Indexers and agents can subscribe to `CalculationRecorded`.
- `score-system` can reference `calculation_id` in a trust snapshot when it needs provenance for a maturity calculation.

## Release And Cutover Plan

- Build and test locally.
- Push source, refreshed IDL, and docs to GitHub.
- Reply to VAN project review `6` explaining the L2 receipt revision.
- Wait for `Proceed` before deployment.
- After deployment, register the deployed program and publish board/readiness evidence.

## Failure And Recovery Paths

- If receipt storage has a bug before deployment, fix and rerun gtest.
- If a future deployed version needs replacement, use VAN application transition flow rather than mutating stale registry metadata.
- Existing pure query methods remain usable even if callers do not need receipts.

## Open Questions

- Should a future version bound receipt storage or add pagination for recent receipts?
- Should receipts include a domain-specific external subject id, or should callers keep that mapping in their own systems?
