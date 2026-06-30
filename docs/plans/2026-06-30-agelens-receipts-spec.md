# Feature Spec

## Problem

The first AgeLens version was useful as deterministic date math, but the VAN
review required an on-chain audit property: a caller could reproduce every
query answer off-chain and gained no durable provenance. To justify deployment,
AgeLens records calculation provenance so downstream agents can point to a
durable on-chain receipt, not only a transient reply.

## User Goal

Turn AgeLens into an auditable calculation receipt service. A caller should be able to record a calculation, receive a `calculation_id`, and later prove that a specific input and expected output match the on-chain receipt.

## In Scope

- Keep existing query methods: `CalculateAge`, `CheckAgeThreshold`, `CheckAgeDaysThreshold`, and `Version`.
- Add stateful receipt storage keyed by monotonically increasing `calculation_id`.
- Add `RecordCalculation(CalculationRequest) -> CalculationReceipt`.
- Add `GetCalculation(calculation_id) -> Option<CalculationReceipt>`.
- Add `VerifyCalculation(calculation_id, inputs, expected) -> bool`.
- Add `CalculationCount() -> u64`.
- Add `CalculationRecorded` event with `calculation_id`, caller, request, and result.
- Support receipt requests for all three calculation modes: full age report, year threshold, and day threshold.
- Preserve validation rules and leap-day behavior from v1.

## Out of Scope

- Proving that a supplied birth date or activation date is true.
- Encrypting or hiding private date inputs.
- Legal or jurisdiction-specific age-gate compliance.
- Deleting receipts.
- Payments or access control.
- Deploying without current VAN `Proceed` guidance and approved release artifacts.

## Actors

- Caller: records a calculation receipt and later references the `calculation_id`.
- Verifier: checks that a stored receipt matches specific inputs and expected output.
- First named consumer: `score-system`, which can store or reference `calculation_id` in readiness/trust snapshots when on-chain provenance matters.
- AgeLens program: owns receipt state and emits calculation events.

## State Changes

The program stores:

- `next_calculation_id: u64`
- `calculations: BTreeMap<u64, CalculationReceipt>`

`RecordCalculation` validates the request, computes the result, stores the receipt, increments `next_calculation_id`, and emits `CalculationRecorded`.

## Messages And Replies

- `AgeLens/RecordCalculation(CalculationRequest request) -> Result<CalculationReceipt, String>`
- `AgeLens/GetCalculation(u64 calculation_id) -> Option<CalculationReceipt>`
- `AgeLens/VerifyCalculation(u64 calculation_id, CalculationRequest inputs, CalculationResult expected) -> bool`
- `AgeLens/CalculationCount() -> u64`
- Existing query methods remain unchanged.

## Events

- `CalculationRecorded(CalculationReceipt)` emitted after a receipt is stored successfully.

## Invariants

- A stored receipt's `calculation_id` is unique and never reused.
- `calculation_count` equals the number of stored receipts.
- Invalid calculation input does not create a receipt and does not emit an event.
- `VerifyCalculation` returns true only when the stored receipt, supplied inputs, supplied expected output, and recomputed output all agree.
- Existing query results must match the corresponding `RecordCalculation` result for the same inputs.

## Edge Cases

- Unknown `calculation_id`.
- Correct id with wrong inputs.
- Correct id and inputs with wrong expected result.
- Invalid request passed to `RecordCalculation`.
- Invalid request passed to `VerifyCalculation`.
- Leap-day receipts and day-threshold receipts.

## Acceptance Criteria

- IDL exposes `RecordCalculation`, `GetCalculation`, `VerifyCalculation`, and `CalculationCount`.
- `cargo test --release --workspace` passes with receipt unit tests and gtest coverage.
- gtest records a receipt, verifies it, rejects mismatched verification, and observes `CalculationRecorded`.
- README and `SKILLS.md` document the receipt workflow and call-chain provenance.
- VAN review package notes that project review `6` is being addressed by the receipt design.
