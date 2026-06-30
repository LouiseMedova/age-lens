# Architecture Note

## Summary
AgeLens is a small stateless Sails program that exposes deterministic age and threshold calculations. It is designed as a composable utility for other agents, not as an identity oracle. The first named consumer is `score-system`, which can use AgeLens for readiness and trust snapshot maturity checks.

## Program And Service Boundaries
The `Program` constructor is thin and exposes one service, `AgeLens`. All business logic lives in the service and pure helper functions.

## State Ownership
No business state is owned by the program in v1. This is intentional: birth dates are sensitive and should not be persisted by a generic utility service.

## Message Flow
The caller sends a query-style message to `AgeLens/CalculateAge`, `AgeLens/CheckAgeThreshold`, or `AgeLens/CheckAgeDaysThreshold`. The program validates inputs, computes the reply synchronously, and returns the result or a named error string.

## Routing And Public Interface
- Existing public routes that must remain stable: none, this is v1.
- New routes introduced by this release: `AgeLens/CalculateAge`, `AgeLens/CheckAgeThreshold`, `AgeLens/CheckAgeDaysThreshold`, `AgeLens/Version`.
- Any intentionally deprecated routes: none.
- Whether any method signature or reply shape changes are proposed: no released interface exists yet.

## Event Contract
- Existing events that must remain stable: none.
- Any new event surface introduced by this release: none.
- Whether any existing event payload changes are proposed: no.
- Whether event versioning is required: no.

## Generated Client Or IDL Impact
- This release requires IDL generation.
- VAN registration, callers, and readiness smoke checks consume the IDL.
- No old generated clients need to coexist for v1.

## Contract Version And Status Surface
The `AgeLens/Version` query returns the semantic version string. There is no lifecycle state such as `Active` or `ReadOnly` because the program has no write surface.

## Off-Chain Components
No frontend, indexer, or automation is required for v1. Callers can use the generated IDL directly. The `score-system` integration is a consumer workflow: score-system reads or receives a subject's registration or launch date, calls AgeLens for `days_alive` and maturity eligibility, then includes that derived fact in its trust snapshot.

## Release And Cutover Plan
Deploy the Sails program after code review, publish `SKILLS.md` and the generated IDL, then register the application in the Vara Agent Network.

## Failure And Recovery Paths
Invalid input returns a deterministic error. If the deployed IDL or docs are wrong, do not register until artifacts are fixed. If a future version adds storage or events, it should be deployed as an additive v2 with explicit privacy review.

## Open Questions
- Should v2 add an optional proof or attestation flow, or remain purely policy-neutral?
