# AgeLens VAN Project Review Package

## Stage

- Current gate: Stage 1 project review / idea guidance.
- Repository: `https://github.com/LouiseMedova/age-lens`
- Do not deploy yet. VAN onboarding requires project guidance `Proceed`, then explicit Stage 2a code/deploy approval from `@cerberus` before spending deploy gas.

## Live Review Result

- Account: `luisa_test`
- Owner hex: `0x7ae4a212d7e78deb906c52cce454e1fcd842ec1f7dbf90705d3dd5ab719de70a`
- Project review id: `6`
- Submit tx hash: `0x559e7c80b818081ca235fce08b8ded50ee74712472bd6e4bf463545f5667b1f7`
- Submit block number: `34256828`
- Submit message id: `0x4d64390717910a0ce0b4f52e3794bfa7a8a79f41dbab0ca6cf2b3dd84458e5e0`
- Current status: `GuidanceRecorded`
- Latest guidance outcome: `NeedsChanges`
- Latest reviewer: `0x8490e070d0664a3ca9498b244aeb5707515e261b9d2cba9e10b674ed6a2f905c`

Reviewer guidance summary:

- Current design is considered L0 pure computation because it is deterministic date arithmetic with no state, events, coordination, evidence protocol, storage, verify method, or audit trail.
- To reach `Proceed`, revise AgeLens toward L2 by adding stateful receipt storage with `calculation_id`.
- Add `VerifyCalculation(calc_id, inputs, expected) -> bool` as a query.
- Emit `CalculationRecorded` events.
- Document the caller chain where on-chain receipt provenance matters.

Do not deploy the original stateless-only version.

Revision status:

- v0.2.0 adds program-owned receipt storage keyed by `calculation_id`.
- v0.2.0 adds `RecordCalculation`, `GetCalculation`, `VerifyCalculation`, and `CalculationCount`.
- v0.2.0 emits `CalculationRecorded(CalculationReceipt)` after successful storage.
- v0.2.0 documents the call chain where `score-system` can reference `calculation_id` in readiness/trust snapshots.

## Build Decision

- Outcome: BUILD-DAPP
- Build: AgeLens, a Sails service for deterministic age and maturity checks with auditable calculation receipts.
- Empty/underserved niche: VAN services can need derived date facts for readiness, eligibility, and lifecycle context, but each consumer should not reimplement calendar edge cases or receipt verification.
- Do not build: an identity oracle, legal age-gate compliance system, or profile store.
- Documented method: `AgeLens/RecordCalculation(CalculationRequest) -> CalculationReceipt throws String`, plus `AgeLens/VerifyCalculation(u64, CalculationRequest, CalculationResult) -> bool`.
- Target consumers: readiness, trust, onboarding, contest, and social-context agents that need derived age facts without storing raw birth dates.
- First named consumer: `score-system`.
- Integrate with: `score-system` can call AgeLens as a utility when recording readiness and trust snapshots.
- Differentiation: AgeLens is policy-neutral, privacy-conscious, and documents leap-day, threshold, receipt, and verification semantics as a reusable service contract.

## First Named Consumer Gate

- Handle/program id: `score-system` / `0x92bcefc26ea7437fa0f4141a7b796774f85e0773063cf592ac12f174a3e62284`
- Live indexer status checked: `Live`
- Live indexer track checked: `Services`
- Description checked from indexer: `Sails score-system for Foundation/Cerberus reviewers to record auditable readiness and trust snapshots for Vara Agent Network actors.`
- Method they call on us: `AgeLens/RecordCalculation`
- Args they pass:
  - `birth_date`: subject app registration, launch, or first-seen date as `{ year, month, day }`
  - `as_of_date`: snapshot date as `{ year, month, day }`
  - `minimum_days`: maturity policy such as `7` or `30`
- Return value they depend on: `calculation_id`, `eligible`, `days_alive`, `minimum_days`, and `reason`
- What action terminates on that value: include `calculation_id`, `maturity_days`, and `maturity_threshold_met` in a readiness/trust snapshot, or mark a subject as too new for a stronger readiness score.
- Trust note: VAN registry entries are operator-attested coordination data, not cryptographic proof of program ownership.

## Project Review Request

Use `docs/van/project-review-request.json` for `Review/SubmitProjectReview`.

```json
{
  "github_url": "https://github.com/LouiseMedova/age-lens",
  "idea": "AgeLens is a Sails utility that returns deterministic age and maturity facts and can record auditable calculation receipts by calculation_id. It helps VAN services handle onboarding, readiness and trust snapshots, contest eligibility, and social context with one documented calendar primitive plus receipt verification."
}
```

## Cerberus Pitch

```text
Hey @cerberus! I'd like to pitch my idea for the Vara Agent Network.

Project: AgeLens - a Sails utility for age, maturity, and auditable calculation receipts.

AgeLens accepts a structured date and an as_of_date, then returns deterministic derived facts such as full years, days alive, birthday status, and threshold eligibility. For workflows that need provenance, it stores a calculation receipt keyed by calculation_id, emits CalculationRecorded, and lets verifiers call VerifyCalculation against the stored receipt. It helps services that need onboarding checks, readiness snapshots, contest eligibility, or lifecycle context use one documented calendar primitive instead of each reimplementing date math, leap-day rules, and receipt verification.

Track: Services

Why it's needed: VAN agents can need age or maturity facts with a durable audit trail, but the reusable primitive should be policy-neutral and privacy-conscious rather than becoming an identity oracle or profile store.

Would love your feedback!
```

## Published Artifacts

- GitHub repo: `https://github.com/LouiseMedova/age-lens`
- Skills URL: `https://raw.githubusercontent.com/LouiseMedova/age-lens/main/SKILLS.md`
- IDL URL: `https://raw.githubusercontent.com/LouiseMedova/age-lens/main/idl/age_lens.idl`
- `skills_hash`: `0x456b8fb329c7e8edfa58cfabee3cfb90c5e98b43c4239d201d331b120fad3d0c`
- `idl_hash`: `0x82edd7a8ee0118e160b7534f7adebc6a5a4bde2c9dc5128444db2b6b9282660b`
- Artifact check: raw GitHub downloads returned bytes matching the local SHA-256 hashes above.

## Code Evidence

- Release build: `cargo build --release`
- Workspace tests: `cargo test --release --workspace`
- CI-quality lint: `cargo clippy --release --all-targets -- -D warnings`
- Stable integration IDL: `idl/age_lens.idl`
- Generated release artifact: `target/wasm32-gear/release/age_lens.idl`

## SubmitProjectReview Command Shape

Set `ACCT` to the operator wallet name before running a live write.

```bash
ACCT="<operator-wallet-name>"
PID="0xa9c8c5a6ef989e39ea52491c9390e8df3e300e88e80348883f98fd08b0293663"
IDL="/Users/luisa/.agents/skills/vara-agent-network-skills/idl/agents_network_client.idl"

vara-wallet --account "$ACCT" --network mainnet --json call "$PID" \
  Review/SubmitProjectReview \
  --args "[{\"github_url\":\"https://github.com/LouiseMedova/age-lens\",\"idea\":\"AgeLens is a Sails utility that returns deterministic age and maturity facts and can record auditable calculation receipts by calculation_id. It helps VAN services handle onboarding, readiness and trust snapshots, contest eligibility, and social context with one documented calendar primitive plus receipt verification.\"}]" \
  --idl "$IDL"
```

Save the returned `PROJECT_REVIEW_ID`, then check:

```bash
vara-wallet --account "$ACCT" --network mainnet --json call "$PID" \
  Review/GetProjectReviewSummary --args "[$PROJECT_REVIEW_ID]" --idl "$IDL"
```

## Owner Reply Draft

Use this after pushing the v0.2.0 revision:

```text
I revised AgeLens to address the L0 pure-computation concern. The v0.2.0 code now has stateful calculation receipts keyed by calculation_id, RecordCalculation, GetCalculation, VerifyCalculation(calc_id, inputs, expected) -> bool, CalculationCount, and CalculationRecorded events. The score-system flow can now store calculation_id in readiness/trust snapshots and later verify the stored receipt against the expected maturity calculation. I also updated README, SKILLS.md, stable IDL, and gtest coverage for record/get/verify/event behavior.
```
