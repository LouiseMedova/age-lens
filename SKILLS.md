# AgeLens Agent Skill

AgeLens is a stateless Sails service for deterministic age and eligibility checks.

## Use Cases

- Onboarding agents can ask whether a user meets a configured age threshold.
- Social agents can compute birthday-aware context without storing raw birth dates.
- Event or contest agents can calculate eligibility at a fixed `as_of_date`.
- Agent lifecycle tools can compute an agent's activation age from its launch date.
- Readiness and trust systems can check whether an app has existed long enough to count as mature.

## Privacy Model

AgeLens does not store input dates and does not emit events with birth-date data. Callers should pass only the date needed for the current query and should avoid writing raw birth dates into their own public state unless they have an explicit reason.

## Methods

### `AgeLens/CalculateAge`

Args:

```json
[
  { "year": 1998, "month": 4, "day": 21 },
  { "year": 2026, "month": 6, "day": 30 }
]
```

Returns:

```json
{
  "years": 28,
  "months_since_birthday": 2,
  "days_alive": 10297,
  "days_until_next_birthday": 295,
  "age_band": "Adult",
  "is_birthday_today": false
}
```

Errors:

- invalid month or day
- `as_of_date` before `birth_date`

### `AgeLens/CheckAgeThreshold`

Args:

```json
[
  { "year": 1998, "month": 4, "day": 21 },
  { "year": 2026, "month": 6, "day": 30 },
  18
]
```

Returns:

```json
{
  "eligible": true,
  "years": 28,
  "minimum_age": 18,
  "reason": "AgeAtOrAboveThreshold"
}
```

Errors:

- same date validation errors as `CalculateAge`
- `minimum_age` greater than 150

### `AgeLens/CheckAgeDaysThreshold`

Args:

```json
[
  { "year": 2026, "month": 6, "day": 1 },
  { "year": 2026, "month": 6, "day": 30 },
  7
]
```

Returns:

```json
{
  "eligible": true,
  "days_alive": 29,
  "minimum_days": 7,
  "reason": "AgeAtOrAboveThreshold"
}
```

Errors:

- same date validation errors as `CalculateAge`

## First Named Consumer

- Handle/program id: `score-system` / `0x92bcefc26ea7437fa0f4141a7b796774f85e0773063cf592ac12f174a3e62284`
- Workflow: score-system records readiness and trust snapshots for Vara Agent Network actors.
- Method it calls on AgeLens: `AgeLens/CheckAgeDaysThreshold`.
- Args it passes: an app's registration or launch date, the snapshot date, and a maturity threshold such as `7` or `30` days.
- Return value it depends on: `eligible`, `days_alive`, `minimum_days`, and `reason`.
- Action terminated by the result: include `maturity_days` and `maturity_threshold_met` in a trust snapshot, or mark the subject as too new for a stronger readiness score.
- Why AgeLens helps: score-system can consume one documented calendar utility instead of reimplementing date arithmetic, leap-day handling, and day-threshold semantics.

## Integration Artifact

Use `idl/age_lens.idl` as the stable committed interface for client generation and partner review. Release builds regenerate matching artifacts under `target/wasm32-gear/release/`.

## Consumer Pitch

AgeLens is useful for agents that need a derived age fact but do not want to own calendar edge cases or persist personal birth-date state. The first concrete consumer is `score-system`, which can use `CheckAgeDaysThreshold` for readiness and trust snapshots; secondary consumers are onboarding, contest, and social-context agents that need `CheckAgeThreshold` or `CalculateAge`.
