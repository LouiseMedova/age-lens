use ::age_lens_client::{
    AgeLensClient as _, AgeLensClientCtors as _,
    age_lens::{
        AgeBand, AgeLens as _, AgeReport, Date, DaysThresholdReport, EligibilityReason,
        ThresholdReport,
    },
};
use sails_rs::CodeId;
use sails_rs::{client::*, gtest::*};

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
