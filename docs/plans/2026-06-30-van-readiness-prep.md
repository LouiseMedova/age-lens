# VAN Readiness Preparation

Prepared before `Registry/RegisterApplication`, while waiting for the
`Review/ApproveApplicationPermit(Register)` permit from `cerberus`.

## Prepared Files

- Identity card args: `/private/tmp/van-agelens-card.json`
- Board announcement args: `/private/tmp/van-agelens-board-post.json`
- Readiness manifest draft: `/private/tmp/van-agelens-readiness.json`

## After Application Permit

1. Build `/private/tmp/van-agelens-register-approved.json` from the permit id
   and `/private/tmp/van-agelens-register-app.json`.
2. Estimate and call `Registry/RegisterApplication`.
3. Estimate and call `Board/SetIdentityCard` with
   `/private/tmp/van-agelens-card.json`.
4. Wait at least 60 seconds for the board rate limit.
5. Estimate and call `Board/PostAnnouncement` with
   `/private/tmp/van-agelens-board-post.json`.
6. Update the readiness manifest `build_proof.local_smoke` after a real
   local-node smoke or accepted equivalent smoke proof.
7. Run readiness check:

```bash
APP_HEX=0xe42153aedda060f7a5d536f81c85103172b3630155e18f8b034486ff0e79b1e9 \
PID=0xa9c8c5a6ef989e39ea52491c9390e8df3e300e88e80348883f98fd08b0293663 \
INDEXER_GRAPHQL_URL=https://agents-explorer.vara.network/graphql \
VARA_NETWORK=mainnet \
node /Users/luisa/.agents/skills/vara-agent-network-skills/scripts/readiness-check.mjs \
  --manifest /private/tmp/van-agelens-readiness.json \
  --out /private/tmp/van-agelens-readiness-output.json
```

8. If `overall` is `PASS`, call `Registry/SubmitApplication`.

## Draft Readiness Check

Ran against `/private/tmp/van-agelens-readiness.json` before Application
registration.

Result: `overall: FAIL`, with expected pre-registration blockers only.

Passing checks:

- `github_ok`: PASS
- `skills_ok`: PASS
- `idl_ok`: PASS
- `documented_errors`: PASS
- `documented_method`: PASS
- `smoke_ok`: PASS

Expected blockers:

- `identity_card_ok`: FAIL, because the Application is not registered and the
  identity card cannot be set yet.
- `build_proof`: FAIL, because `local_smoke.ok` is deliberately left `false`
  until a real local-node smoke or accepted equivalent smoke proof is recorded.

The safe smoke call executed successfully:

```bash
vara-wallet --network mainnet --json call \
  0xe42153aedda060f7a5d536f81c85103172b3630155e18f8b034486ff0e79b1e9 \
  AgeLens/CalculateAge \
  --args '[{"year":2000,"month":1,"day":1},{"year":2026,"month":6,"day":30}]' \
  --idl idl/age_lens.idl
```

## Card Summary

The prepared identity card presents `agelens` as a Services-track Sails utility
for deterministic age, eligibility, days-alive, and receipt verification facts.
It points callers to query-only methods for privacy-sensitive checks and to
`RecordCalculation` plus `VerifyCalculation` when provenance is needed.

## Announcement Summary

The prepared announcement satisfies the completion-quality requirement by
naming:

- Method: `AgeLens/CalculateAge(birth_date, as_of_date)`
- Args shape: two `Date` objects
- Return shape: `AgeReport`
- Error behavior: invalid date or `as_of_date` before `birth_date` throws a
  String validation error
- Target callers: `score-system`, onboarding agents, contest agents, and other
  VAN Services apps that need shared date-derived facts

It also names the receipt flow:
`RecordCalculation(request) -> CalculationReceipt` and
`VerifyCalculation(calculation_id, inputs, expected) -> bool`.

## Final Readiness Check

After Application registration, identity card, and the manual `Invitation`
announcement were live, `/private/tmp/van-agelens-readiness.json` was updated
with accepted post-deploy smoke evidence:

- mainnet discover succeeded
- `Version` returned `0.2.0`
- `CalculationCount` returned `0`
- `AgeLens/CalculateAge(2000-01-01, 2026-06-30)` returned
  `years=26`, `days_alive=9677`, and `age_band=Adult`

`preflight-register.mjs` passed for the GitHub URL, SKILLS URL/hash, and IDL
URL/hash.

`readiness-check.mjs` result: `overall: PASS`.

Passing checks:

- `github_ok`
- `skills_ok`
- `idl_ok`
- `identity_card_ok`
- `documented_errors`
- `build_proof`
- `documented_method`
- `smoke_ok`

`Registry/SubmitApplication` then moved AgeLens to `Submitted` revision `1`,
and `@cerberus` was pinged for Stage 2b / `PublishApplication` review in chat
message `105`.

Cerberus then published AgeLens:

- Application status: `Live`
- Review verdict: `ApprovedForListing`
- Publish tx:
  `0xb5f0a54eb1e13a16c7bedd93892ed2ab4243a8a750057d6ce1aebaf7be7a75b0`
- Publish block: `34260979`
- Cerberus chat message: `106`
