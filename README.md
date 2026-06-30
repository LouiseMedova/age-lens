## AgeLens

[![Build Status](https://github.com/LouiseMedova/age-lens/workflows/CI/badge.svg)](https://github.com/LouiseMedova/age-lens/actions)

AgeLens is a privacy-conscious age, eligibility, and calculation-receipt utility for Vara Agent Network agents.
It turns a structured birth or activation date plus an `as_of_date` into deterministic derived facts, and can optionally record an auditable on-chain receipt for calculations that need provenance.

## Why This Is Useful

Agents often need age-derived facts, not raw birth dates: onboarding checks, contest eligibility, birthday-aware social flows, wellness personalization, or agent activation age for lifecycle displays. AgeLens gives them a shared, documented, policy-neutral method so each consumer does not reimplement date math, leap-day handling, and threshold semantics differently.

For workflows that need an audit trail, `RecordCalculation` stores a receipt keyed by `calculation_id` and emits `CalculationRecorded`. Callers that do not need provenance can keep using the query-only methods. AgeLens still does not prove that a supplied date is true, and callers should avoid recording sensitive personal birth dates unless they have a clear reason.

## First Consumer

The first named integration target is `score-system`, a live Services-track app that records readiness and trust snapshots for Vara Agent Network actors. Score-system can call `AgeLens/RecordCalculation` with an app's registration or launch date and a policy threshold such as 7 or 30 days, then include the returned `calculation_id`, `eligible` flag, and `days_alive` value in a trust snapshot without implementing calendar math itself.

## Public Methods

- `AgeLens/CalculateAge(birth_date, as_of_date) -> AgeReport`
- `AgeLens/CheckAgeThreshold(birth_date, as_of_date, minimum_age) -> ThresholdReport`
- `AgeLens/CheckAgeDaysThreshold(birth_date, as_of_date, minimum_days) -> DaysThresholdReport`
- `AgeLens/RecordCalculation(request) -> CalculationReceipt`
- `AgeLens/GetCalculation(calculation_id) -> Option<CalculationReceipt>`
- `AgeLens/VerifyCalculation(calculation_id, inputs, expected) -> bool`
- `AgeLens/CalculationCount() -> u64`
- `AgeLens/Version() -> String`

Dates use `{ year, month, day }`. For leap-day birthdays, non-leap years observe the birthday on March 1.

## Receipt Flow

Use `RecordCalculation` when another agent needs on-chain provenance. The stored `CalculationReceipt` contains the caller, request, result, and `calculation_id`; the service emits `CalculationRecorded` after storage succeeds. Later, `VerifyCalculation` recomputes the request and returns true only if the stored receipt, supplied inputs, and expected result all match.

The stable integration artifact is committed at [`idl/age_lens.idl`](idl/age_lens.idl). Release builds also regenerate the IDL and WASM artifacts under `target/wasm32-gear/release/`.

### 🏗️ Building

```bash
cargo build --release
```

### ✅ Testing

```bash
cargo test --release
```

# License

The source code is licensed under the [MIT license](LICENSE).
