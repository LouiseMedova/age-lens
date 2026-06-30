## AgeLens

[![Build Status](https://github.com/LouiseMedova/age-lens/workflows/CI/badge.svg)](https://github.com/LouiseMedova/age-lens/actions)

AgeLens is a privacy-conscious age and eligibility utility for Vara Agent Network agents.
It turns a structured birth or activation date plus an `as_of_date` into a deterministic age report without storing the input date on-chain.

## Why This Is Useful

Agents often need age-derived facts, not raw birth dates: onboarding checks, contest eligibility, birthday-aware social flows, wellness personalization, or agent activation age for lifecycle displays. AgeLens gives them a shared, documented, policy-neutral method so each consumer does not reimplement date math, leap-day handling, and threshold semantics differently.

The service is intentionally stateless. It does not persist birth dates, does not emit events containing personal data, and returns only derived facts.

## First Consumer

The first named integration target is `score-system`, a live Services-track app that records readiness and trust snapshots for Vara Agent Network actors. Score-system can call `AgeLens/CheckAgeDaysThreshold` with an app's registration or launch date and a policy threshold such as 7 or 30 days, then include the returned `eligible` flag and `days_alive` value in a trust snapshot without implementing calendar math itself.

## Public Methods

- `AgeLens/CalculateAge(birth_date, as_of_date) -> AgeReport`
- `AgeLens/CheckAgeThreshold(birth_date, as_of_date, minimum_age) -> ThresholdReport`
- `AgeLens/CheckAgeDaysThreshold(birth_date, as_of_date, minimum_days) -> DaysThresholdReport`
- `AgeLens/Version() -> String`

Dates use `{ year, month, day }`. For leap-day birthdays, non-leap years observe the birthday on March 1.

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
