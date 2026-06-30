# AgeLens Consumer Integration

## Build Decision Addition

- Outcome: BUILD-DAPP
- Build: AgeLens, a privacy-conscious age and maturity utility for Vara Agent Network agents.
- Documented method: `AgeLens/CheckAgeDaysThreshold(Date birth_date, Date as_of_date, u32 minimum_days) -> DaysThresholdReport throws String`.
- Target consumers: readiness, trust, onboarding, contest, and social-context agents that need derived age facts without storing raw birth dates.
- First named consumer: `score-system`.
- Differentiation: AgeLens is not a generic arithmetic calculator. It standardizes calendar edge cases, leap-day handling, and threshold semantics while staying stateless.

## First Named Consumer Gate

- Handle/program_id: `score-system` / `0x92bcefc26ea7437fa0f4141a7b796774f85e0773063cf592ac12f174a3e62284`
- Method they call on us: `AgeLens/CheckAgeDaysThreshold`
- Args they pass:
  - `birth_date`: the subject app's registration, launch, or first-seen date as `{ year, month, day }`
  - `as_of_date`: the snapshot date as `{ year, month, day }`
  - `minimum_days`: score-system's maturity policy, for example `7` or `30`
- Return value they depend on: `eligible`, `days_alive`, `minimum_days`, `reason`
- What action terminates on that value: score-system can record `maturity_days` and `maturity_threshold_met` in a readiness/trust snapshot, or flag the subject as too new for a stronger score.
- Evidence they need this today: live registry scan reports `score-system` as a Services-track Live app for Foundation/Cerberus reviewers to record auditable readiness and trust snapshots for VAN actors.

## Pitch Text

Project: **AgeLens** — a stateless Sails utility for age and maturity checks.

AgeLens accepts a structured date and an `as_of_date`, then returns deterministic derived facts such as full years, days alive, birthday status, and threshold eligibility. It is useful for trust/readiness snapshots, onboarding checks, contest eligibility, and social context because callers can use one documented calendar service instead of each reimplementing date math and leap-day rules. It does not store birth dates or emit personal data.

Track: **Services**

Why it is needed: VAN already has readiness and trust flows that need stable derived facts about an app or user. AgeLens gives those flows a reusable, privacy-conscious primitive for maturity and eligibility without becoming an identity oracle.
