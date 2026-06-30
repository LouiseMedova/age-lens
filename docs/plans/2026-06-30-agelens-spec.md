# Feature Spec

Note: this was the initial v0.1 stateless-calculation spec. VAN project review `6` requested an auditable receipt layer; the v0.2 extension is specified in `2026-06-30-agelens-receipts-spec.md`.

## Problem
Agents may need age-derived facts for onboarding, eligibility, lifecycle displays, birthday-aware social behavior, or personalization. If each agent implements date math independently, edge cases such as leap years, birthdays later in the year, and threshold semantics can drift. Storing birth dates in every consumer also increases privacy risk.

## User Goal
Build a useful Vara Agent Network application that accepts a birth date or agent activation date and returns deterministic age and eligibility facts for another agent to consume.

## In Scope
- A stateless Sails service named `AgeLens`.
- Structured date input as `{ year, month, day }`.
- `CalculateAge` query returning age in years, months since birthday, days until next birthday, age band, and birthday-today flag.
- `CheckAgeThreshold` query returning whether the date meets a caller-provided minimum age.
- `CheckAgeDaysThreshold` query returning whether the date meets a caller-provided minimum number of days since birth, registration, or launch.
- Deterministic leap-day behavior: February 29 birthdays are observed on March 1 in non-leap years.
- Validation errors for invalid dates, future birth dates relative to `as_of_date`, and unreasonable age thresholds.
- Documentation that explains why callers should not store raw birth dates on-chain.

## Out of Scope
- Identity verification or proof that the supplied date is true.
- Legal compliance advice for jurisdiction-specific age gates.
- Storage of birth dates, user profiles, or eligibility history.
- Oracle data, off-chain attestations, or private-data cryptography.
- Frontend work for v1.

## Actors
- Calling agent: passes a structured date and consumes the derived result.
- First named consumer: `score-system`, a live Services-track application that records readiness and trust snapshots for Vara Agent Network actors.
- End user or agent profile owner: may provide a birth date or activation date to the calling agent.
- AgeLens program: validates dates and returns deterministic derived facts without storing input.

## State Changes
None. AgeLens v1 is stateless and does not persist input dates or results.

## Messages And Replies
- `AgeLens/CalculateAge(Date birth_date, Date as_of_date) -> Result<AgeReport, String>`
- `AgeLens/CheckAgeThreshold(Date birth_date, Date as_of_date, u16 minimum_age) -> Result<ThresholdReport, String>`
- `AgeLens/CheckAgeDaysThreshold(Date birth_date, Date as_of_date, u32 minimum_days) -> Result<DaysThresholdReport, String>`
- `AgeLens/Version() -> String`

## Events
None. Emitting events would risk publishing sensitive input data or creating misleading activity evidence for a pure query service.

## Invariants
- No input birth date is written to contract state.
- No event includes a birth date.
- `as_of_date` must be equal to or later than `birth_date`.
- Date validation follows Gregorian leap-year rules.
- `CheckAgeThreshold` must use the same age calculation as `CalculateAge`.
- `CheckAgeDaysThreshold` must use the same `days_alive` value as `CalculateAge`.
- Leap-day birthdays use the documented March 1 observation rule in non-leap years.

## Edge Cases
- Invalid month, day zero, and impossible month/day pairs.
- February 29 on leap and non-leap years.
- Birthday today.
- Birthday later in the current year.
- `as_of_date` before `birth_date`.
- `minimum_age > 150`.
- New app or agent maturity checks where the threshold is measured in days rather than years.

## Acceptance Criteria
- `cargo test --release --workspace` passes for valid dates, invalid dates, before-birthday cases, year threshold checks, day threshold checks, and leap-day behavior.
- The generated IDL exposes `AgeLens/CalculateAge`, `AgeLens/CheckAgeThreshold`, `AgeLens/CheckAgeDaysThreshold`, and `AgeLens/Version`.
- README and `SKILLS.md` document the privacy model, method args, return shape, errors, and first named consumer.
- The VAN pitch frames AgeLens as a composable utility for onboarding, contest eligibility, social context, and agent activation age rather than a raw calculator.
