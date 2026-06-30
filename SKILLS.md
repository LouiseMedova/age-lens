# AgeLens Agent Skill

AgeLens is a Sails service for deterministic age, eligibility, and auditable calculation receipt checks.

## Use Cases

- Onboarding agents can ask whether a user meets a configured age threshold.
- Social agents can compute birthday-aware context without storing raw birth dates.
- Event or contest agents can calculate eligibility at a fixed `as_of_date`.
- Agent lifecycle tools can compute an agent's activation age from its launch date.
- Readiness and trust systems can check whether an app has existed long enough to count as mature.
- Audit-oriented agents can record a calculation receipt and later verify it by `calculation_id`.

## Privacy Model

The query-only methods do not store input dates. `RecordCalculation` intentionally stores the request and result in a public receipt and emits `CalculationRecorded`; callers should use it for activation, registration, or other non-sensitive dates unless they have a clear reason to record a personal birth date. AgeLens does not prove that the supplied date is true.

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

### `AgeLens/RecordCalculation`

Args:

```json
[
  {
    "CheckAgeDaysThreshold": {
      "birth_date": { "year": 2026, "month": 6, "day": 1 },
      "as_of_date": { "year": 2026, "month": 6, "day": 30 },
      "minimum_days": 7
    }
  }
]
```

Returns:

```json
{
  "calculation_id": 1,
  "caller": "0x...",
  "request": {
    "CheckAgeDaysThreshold": {
      "birth_date": { "year": 2026, "month": 6, "day": 1 },
      "as_of_date": { "year": 2026, "month": 6, "day": 30 },
      "minimum_days": 7
    }
  },
  "result": {
    "DaysThreshold": {
      "eligible": true,
      "days_alive": 29,
      "minimum_days": 7,
      "reason": "AgeAtOrAboveThreshold"
    }
  }
}
```

Event:

- `CalculationRecorded(CalculationReceipt)`

Errors:

- same validation errors as the selected calculation request

### `AgeLens/GetCalculation`

Args:

```json
[1]
```

Returns the stored `CalculationReceipt` or `null`.

### `AgeLens/VerifyCalculation`

Args:

```json
[
  1,
  {
    "CheckAgeDaysThreshold": {
      "birth_date": { "year": 2026, "month": 6, "day": 1 },
      "as_of_date": { "year": 2026, "month": 6, "day": 30 },
      "minimum_days": 7
    }
  },
  {
    "DaysThreshold": {
      "eligible": true,
      "days_alive": 29,
      "minimum_days": 7,
      "reason": "AgeAtOrAboveThreshold"
    }
  }
]
```

Returns `true` only when the stored receipt, supplied inputs, supplied expected output, and recomputed output all agree.

### `AgeLens/CalculationCount`

Args:

```json
[]
```

Returns the number of stored receipts.

## First Named Consumer

- Handle/program id: `score-system` / `0x92bcefc26ea7437fa0f4141a7b796774f85e0773063cf592ac12f174a3e62284`
- Workflow: score-system records readiness and trust snapshots for Vara Agent Network actors.
- Method it calls on AgeLens: `AgeLens/RecordCalculation` for auditable snapshots, or `AgeLens/CheckAgeDaysThreshold` for query-only checks.
- Args it passes: an app's registration or launch date, the snapshot date, and a maturity threshold such as `7` or `30` days.
- Return value it depends on: `calculation_id`, `eligible`, `days_alive`, `minimum_days`, and `reason`.
- Action terminated by the result: include `calculation_id`, `maturity_days`, and `maturity_threshold_met` in a trust snapshot, or mark the subject as too new for a stronger readiness score.
- Why AgeLens helps: score-system can consume one documented calendar utility and receipt surface instead of reimplementing date arithmetic, leap-day handling, threshold semantics, and receipt verification.

## Integration Artifact

Use `idl/age_lens.idl` as the stable committed interface for client generation and partner review. Release builds regenerate matching artifacts under `target/wasm32-gear/release/`.

## Consumer Pitch

AgeLens is useful for agents that need a derived age fact but do not want to own calendar edge cases. The first concrete consumer is `score-system`, which can use `RecordCalculation` for readiness and trust snapshots when provenance matters; secondary consumers are onboarding, contest, and social-context agents that need `CheckAgeThreshold`, `CheckAgeDaysThreshold`, or `CalculateAge`.
