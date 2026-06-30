use ::age_lens_client::{
    AgeLensClient as _, AgeLensClientCtors as _,
    age_lens::{
        AgeBand, AgeLens as _, AgeReport, CalculationReceipt, CalculationRequest,
        CalculationResult, Date, DaysThresholdInput, DaysThresholdReport, EligibilityReason,
        ThresholdReport, events::AgeLensEvents,
    },
};
use sails_rs::CodeId;
use sails_rs::futures::StreamExt;
use sails_rs::{client::*, gtest::*, prelude::*};

const CALLER_ID: u64 = 42;
const TEST_ACCOUNT_BALANCE: u128 = 100_000_000_000_000;

#[tokio::test]
async fn calculates_age_and_threshold_in_gtest() {
    let (env, code_id) = create_env();
    let program = deploy_age_lens_program(&env, code_id, "agelens-gtest").await;
    let service = program.age_lens();

    let age: sails_rs::Result<AgeReport, sails_rs::String> = service
        .calculate_age(date(1998, 4, 21), date(2026, 6, 30))
        .await
        .unwrap();
    assert_eq!(age.unwrap().years, 28);

    let threshold: sails_rs::Result<ThresholdReport, sails_rs::String> = service
        .check_age_threshold(date(2010, 6, 30), date(2026, 6, 30), 16)
        .await
        .unwrap();
    let threshold = threshold.unwrap();
    assert!(threshold.eligible);
    assert_eq!(threshold.reason, EligibilityReason::AgeAtOrAboveThreshold);

    let maturity: sails_rs::Result<DaysThresholdReport, sails_rs::String> = service
        .check_age_days_threshold(date(2026, 6, 1), date(2026, 6, 30), 7)
        .await
        .unwrap();
    let maturity = maturity.unwrap();
    assert!(maturity.eligible);
    assert_eq!(maturity.days_alive, 29);
    assert_eq!(maturity.reason, EligibilityReason::AgeAtOrAboveThreshold);
}

#[tokio::test]
async fn handles_leap_day_observation_in_gtest() {
    let (env, code_id) = create_env();
    let program = deploy_age_lens_program(&env, code_id, "agelens-leap").await;
    let service = program.age_lens();

    let age: sails_rs::Result<AgeReport, sails_rs::String> = service
        .calculate_age(date(2000, 2, 29), date(2026, 3, 1))
        .await
        .unwrap();
    let age = age.unwrap();
    assert_eq!(age.years, 26);
    assert_eq!(age.age_band, AgeBand::Adult);
    assert!(age.is_birthday_today);
}

#[tokio::test]
async fn records_and_verifies_calculation_receipt_in_gtest() {
    let (env, code_id) = create_env();
    let program = deploy_age_lens_program(&env, code_id, "agelens-receipt").await;
    let mut service = program.age_lens();
    let mut events = service.listen().await.unwrap();
    let request = maturity_request();

    let receipt: sails_rs::Result<CalculationReceipt, sails_rs::String> =
        service.record_calculation(request.clone()).await.unwrap();
    let receipt = receipt.unwrap();
    let expected = CalculationResult::DaysThreshold(DaysThresholdReport {
        eligible: true,
        days_alive: 29,
        minimum_days: 7,
        reason: EligibilityReason::AgeAtOrAboveThreshold,
    });

    assert_eq!(receipt.calculation_id, 1);
    assert_eq!(receipt.caller, actor_id(CALLER_ID));
    assert_eq!(receipt.request, request);
    assert_eq!(receipt.result, expected);

    assert_eq!(
        events.next().await.unwrap(),
        (
            program.id(),
            AgeLensEvents::CalculationRecorded(receipt.clone())
        )
    );

    let count: u64 = service.calculation_count().await.unwrap();
    assert_eq!(count, 1);

    let stored: Option<CalculationReceipt> = service.get_calculation(1).await.unwrap();
    assert_eq!(stored, Some(receipt));

    let verified: bool = service
        .verify_calculation(1, request.clone(), expected.clone())
        .await
        .unwrap();
    assert!(verified);

    let wrong_expected = CalculationResult::DaysThreshold(DaysThresholdReport {
        eligible: false,
        days_alive: 29,
        minimum_days: 7,
        reason: EligibilityReason::AgeBelowThreshold,
    });
    let rejected: bool = service
        .verify_calculation(1, request, wrong_expected)
        .await
        .unwrap();
    assert!(!rejected);
}

async fn deploy_age_lens_program(
    env: &GtestEnv,
    code_id: CodeId,
    salt: &str,
) -> sails_rs::client::Actor<::age_lens_client::AgeLensClientProgram, GtestEnv> {
    env.deploy::<::age_lens_client::AgeLensClientProgram>(code_id, salt.as_bytes().to_vec())
        .new()
        .await
        .unwrap()
}

fn create_env() -> (GtestEnv, CodeId) {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(CALLER_ID, TEST_ACCOUNT_BALANCE);

    let code_id = system.submit_code(::age_lens::WASM_BINARY);
    let env = GtestEnv::new(system, CALLER_ID.into());
    (env, code_id)
}

fn date(year: u16, month: u8, day: u8) -> Date {
    Date { year, month, day }
}

fn maturity_request() -> CalculationRequest {
    CalculationRequest::CheckAgeDaysThreshold(DaysThresholdInput {
        birth_date: date(2026, 6, 1),
        as_of_date: date(2026, 6, 30),
        minimum_days: 7,
    })
}

fn actor_id(id: u64) -> ActorId {
    ActorId::from(id)
}
