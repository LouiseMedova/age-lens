#![no_std]

#[cfg(test)]
extern crate std;

use sails_rs::{cell::RefCell, collections::BTreeMap, gstd::msg, prelude::*};

pub const VERSION: &str = "0.2.0";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub enum AgeBand {
    Child,
    Teen,
    Adult,
    Senior,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub enum EligibilityReason {
    AgeAtOrAboveThreshold,
    AgeBelowThreshold,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub struct AgeReport {
    pub years: u16,
    pub months_since_birthday: u8,
    pub days_alive: u32,
    pub days_until_next_birthday: u16,
    pub age_band: AgeBand,
    pub is_birthday_today: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub struct ThresholdReport {
    pub eligible: bool,
    pub years: u16,
    pub minimum_age: u16,
    pub reason: EligibilityReason,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub struct DaysThresholdReport {
    pub eligible: bool,
    pub days_alive: u32,
    pub minimum_days: u32,
    pub reason: EligibilityReason,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub struct CalculateAgeInput {
    pub birth_date: Date,
    pub as_of_date: Date,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub struct AgeThresholdInput {
    pub birth_date: Date,
    pub as_of_date: Date,
    pub minimum_age: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub struct DaysThresholdInput {
    pub birth_date: Date,
    pub as_of_date: Date,
    pub minimum_days: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub enum CalculationRequest {
    CalculateAge(CalculateAgeInput),
    CheckAgeThreshold(AgeThresholdInput),
    CheckAgeDaysThreshold(DaysThresholdInput),
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub enum CalculationResult {
    Age(AgeReport),
    Threshold(ThresholdReport),
    DaysThreshold(DaysThresholdReport),
}

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub struct CalculationReceipt {
    pub calculation_id: u64,
    pub caller: ActorId,
    pub request: CalculationRequest,
    pub result: CalculationResult,
}

#[sails_rs::event]
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, TypeInfo, ReflectHash)]
#[codec(crate = sails_rs::scale_codec)]
#[type_info(crate = sails_rs::type_info)]
#[reflect_hash(crate = sails_rs)]
pub enum AgeLensEvent {
    CalculationRecorded(CalculationReceipt),
}

struct AgeLensState {
    next_calculation_id: u64,
    calculations: BTreeMap<u64, CalculationReceipt>,
}

impl Default for AgeLensState {
    fn default() -> Self {
        Self {
            next_calculation_id: 1,
            calculations: BTreeMap::new(),
        }
    }
}

pub struct Program {
    state: RefCell<AgeLensState>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[program]
impl Program {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(AgeLensState::default()),
        }
    }

    pub fn age_lens(&self) -> AgeLensService<'_> {
        AgeLensService::new(&self.state)
    }
}

pub struct AgeLensService<'a> {
    state: &'a RefCell<AgeLensState>,
}

impl<'a> AgeLensService<'a> {
    fn new(state: &'a RefCell<AgeLensState>) -> Self {
        Self { state }
    }
}

#[service(events = AgeLensEvent)]
impl AgeLensService<'_> {
    #[export(unwrap_result)]
    pub fn calculate_age(&self, birth_date: Date, as_of_date: Date) -> Result<AgeReport, String> {
        calculate_age_report(birth_date, as_of_date)
    }

    #[export(unwrap_result)]
    pub fn check_age_threshold(
        &self,
        birth_date: Date,
        as_of_date: Date,
        minimum_age: u16,
    ) -> Result<ThresholdReport, String> {
        check_age_threshold_report(birth_date, as_of_date, minimum_age)
    }

    #[export(unwrap_result)]
    pub fn check_age_days_threshold(
        &self,
        birth_date: Date,
        as_of_date: Date,
        minimum_days: u32,
    ) -> Result<DaysThresholdReport, String> {
        check_age_days_threshold_report(birth_date, as_of_date, minimum_days)
    }

    #[export]
    pub fn version(&self) -> String {
        VERSION.into()
    }

    #[export(unwrap_result)]
    pub fn record_calculation(
        &mut self,
        request: CalculationRequest,
    ) -> Result<CalculationReceipt, String> {
        let result = execute_calculation_request(request.clone())?;
        let receipt = {
            let mut state = self.state.borrow_mut();
            let calculation_id = state.next_calculation_id;
            state.next_calculation_id = state
                .next_calculation_id
                .checked_add(1)
                .ok_or_else(|| String::from("calculation_id overflow"))?;

            let receipt = CalculationReceipt {
                calculation_id,
                caller: msg::source(),
                request,
                result,
            };
            state.calculations.insert(calculation_id, receipt.clone());
            receipt
        };

        self.emit_event(AgeLensEvent::CalculationRecorded(receipt.clone()))
            .expect("failed to emit CalculationRecorded");

        Ok(receipt)
    }

    #[export]
    pub fn get_calculation(&self, calculation_id: u64) -> Option<CalculationReceipt> {
        self.state
            .borrow()
            .calculations
            .get(&calculation_id)
            .cloned()
    }

    #[export]
    pub fn verify_calculation(
        &self,
        calculation_id: u64,
        inputs: CalculationRequest,
        expected: CalculationResult,
    ) -> bool {
        let Ok(recomputed) = execute_calculation_request(inputs.clone()) else {
            return false;
        };

        if recomputed != expected {
            return false;
        }

        match self.state.borrow().calculations.get(&calculation_id) {
            Some(receipt) => receipt.request == inputs && receipt.result == expected,
            None => false,
        }
    }

    #[export]
    pub fn calculation_count(&self) -> u64 {
        self.state.borrow().calculations.len() as u64
    }
}

fn execute_calculation_request(request: CalculationRequest) -> Result<CalculationResult, String> {
    match request {
        CalculationRequest::CalculateAge(input) => {
            calculate_age_report(input.birth_date, input.as_of_date).map(CalculationResult::Age)
        }
        CalculationRequest::CheckAgeThreshold(input) => {
            check_age_threshold_report(input.birth_date, input.as_of_date, input.minimum_age)
                .map(CalculationResult::Threshold)
        }
        CalculationRequest::CheckAgeDaysThreshold(input) => {
            check_age_days_threshold_report(input.birth_date, input.as_of_date, input.minimum_days)
                .map(CalculationResult::DaysThreshold)
        }
    }
}

fn calculate_age_report(birth_date: Date, as_of_date: Date) -> Result<AgeReport, String> {
    validate_date(birth_date)?;
    validate_date(as_of_date)?;

    if date_lt(as_of_date, birth_date) {
        return Err("as_of_date cannot be before birth_date".into());
    }

    let observed_this_year = observed_birthday_in_year(birth_date, as_of_date.year);
    let has_had_birthday = !date_lt(as_of_date, observed_this_year);
    let mut years = as_of_date.year - birth_date.year;

    if !has_had_birthday {
        years -= 1;
    }

    let last_birthday = if has_had_birthday {
        observed_this_year
    } else {
        observed_birthday_in_year(birth_date, as_of_date.year - 1)
    };

    let next_birthday = if date_eq(as_of_date, observed_this_year) {
        as_of_date
    } else if has_had_birthday {
        observed_birthday_in_year(birth_date, as_of_date.year + 1)
    } else {
        observed_this_year
    };

    Ok(AgeReport {
        years,
        months_since_birthday: full_months_between(last_birthday, as_of_date),
        days_alive: days_alive(birth_date, as_of_date),
        days_until_next_birthday: (day_number(next_birthday) - day_number(as_of_date)) as u16,
        age_band: age_band(years),
        is_birthday_today: date_eq(as_of_date, observed_this_year),
    })
}

fn check_age_threshold_report(
    birth_date: Date,
    as_of_date: Date,
    minimum_age: u16,
) -> Result<ThresholdReport, String> {
    if minimum_age > 150 {
        return Err("minimum_age must be 150 or lower".into());
    }

    let report = calculate_age_report(birth_date, as_of_date)?;
    let eligible = report.years >= minimum_age;

    Ok(ThresholdReport {
        eligible,
        years: report.years,
        minimum_age,
        reason: if eligible {
            EligibilityReason::AgeAtOrAboveThreshold
        } else {
            EligibilityReason::AgeBelowThreshold
        },
    })
}

fn check_age_days_threshold_report(
    birth_date: Date,
    as_of_date: Date,
    minimum_days: u32,
) -> Result<DaysThresholdReport, String> {
    let report = calculate_age_report(birth_date, as_of_date)?;
    let eligible = report.days_alive >= minimum_days;

    Ok(DaysThresholdReport {
        eligible,
        days_alive: report.days_alive,
        minimum_days,
        reason: if eligible {
            EligibilityReason::AgeAtOrAboveThreshold
        } else {
            EligibilityReason::AgeBelowThreshold
        },
    })
}

fn validate_date(date: Date) -> Result<(), String> {
    if date.year == 0 {
        return Err("year must be greater than zero".into());
    }

    if !(1..=12).contains(&date.month) {
        return Err("month must be between 1 and 12".into());
    }

    let max_day = days_in_month(date.year, date.month);
    if date.day == 0 || date.day > max_day {
        return Err("day is invalid for the given month and year".into());
    }

    Ok(())
}

fn age_band(years: u16) -> AgeBand {
    match years {
        0..=12 => AgeBand::Child,
        13..=17 => AgeBand::Teen,
        18..=64 => AgeBand::Adult,
        _ => AgeBand::Senior,
    }
}

fn observed_birthday_in_year(birth_date: Date, year: u16) -> Date {
    if birth_date.month == 2 && birth_date.day == 29 && !is_leap_year(year) {
        Date {
            year,
            month: 3,
            day: 1,
        }
    } else {
        Date {
            year,
            month: birth_date.month,
            day: birth_date.day,
        }
    }
}

fn full_months_between(start: Date, end: Date) -> u8 {
    let mut months =
        (end.year as i32 - start.year as i32) * 12 + end.month as i32 - start.month as i32;

    if end.day < start.day {
        months -= 1;
    }

    months as u8
}

fn date_lt(left: Date, right: Date) -> bool {
    (left.year, left.month, left.day) < (right.year, right.month, right.day)
}

fn date_eq(left: Date, right: Date) -> bool {
    (left.year, left.month, left.day) == (right.year, right.month, right.day)
}

fn day_number(date: Date) -> i32 {
    days_before_year(date.year) + day_of_year(date) as i32 - 1
}

fn days_alive(birth_date: Date, as_of_date: Date) -> u32 {
    (day_number(as_of_date) - day_number(birth_date)) as u32
}

fn days_before_year(year: u16) -> i32 {
    let y = year as i32 - 1;
    365 * y + y / 4 - y / 100 + y / 400
}

fn day_of_year(date: Date) -> u16 {
    const DAYS_BEFORE_MONTH: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let leap_offset = if date.month > 2 && is_leap_year(date.year) {
        1
    } else {
        0
    };

    DAYS_BEFORE_MONTH[(date.month - 1) as usize] + date.day as u16 + leap_offset
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && !year.is_multiple_of(100) || year.is_multiple_of(400)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(year: u16, month: u8, day: u8) -> Date {
        Date { year, month, day }
    }

    fn maturity_request() -> CalculationRequest {
        CalculationRequest::CheckAgeDaysThreshold(DaysThresholdInput {
            birth_date: d(2026, 6, 1),
            as_of_date: d(2026, 6, 30),
            minimum_days: 7,
        })
    }

    #[test]
    fn calculates_full_age_after_birthday() {
        let report = calculate_age_report(d(1998, 4, 21), d(2026, 6, 30)).unwrap();

        assert_eq!(report.years, 28);
        assert_eq!(report.months_since_birthday, 2);
        assert!(report.days_alive > 10_000);
        assert_eq!(report.age_band, AgeBand::Adult);
        assert!(!report.is_birthday_today);
    }

    #[test]
    fn subtracts_year_before_birthday() {
        let report = calculate_age_report(d(1998, 12, 20), d(2026, 6, 30)).unwrap();

        assert_eq!(report.years, 27);
        assert_eq!(report.months_since_birthday, 6);
    }

    #[test]
    fn reports_threshold_eligibility() {
        let report = check_age_threshold_report(d(2010, 6, 30), d(2026, 6, 30), 16).unwrap();

        assert!(report.eligible);
        assert_eq!(report.years, 16);
        assert_eq!(report.reason, EligibilityReason::AgeAtOrAboveThreshold);
    }

    #[test]
    fn reports_day_threshold_for_agent_maturity() {
        let report = check_age_days_threshold_report(d(2026, 6, 1), d(2026, 6, 30), 7).unwrap();

        assert!(report.eligible);
        assert_eq!(report.days_alive, 29);
        assert_eq!(report.minimum_days, 7);
        assert_eq!(report.reason, EligibilityReason::AgeAtOrAboveThreshold);
    }

    #[test]
    fn rejects_invalid_dates() {
        let error = calculate_age_report(d(2026, 2, 29), d(2026, 6, 30)).unwrap_err();

        assert_eq!(error, "day is invalid for the given month and year");
    }

    #[test]
    fn observes_leap_day_birthdays_on_march_first() {
        let report = calculate_age_report(d(2000, 2, 29), d(2026, 3, 1)).unwrap();

        assert_eq!(report.years, 26);
        assert!(report.is_birthday_today);
    }

    #[test]
    fn executes_calculation_request_for_receipt_storage() {
        let result = execute_calculation_request(maturity_request()).unwrap();

        assert_eq!(
            result,
            CalculationResult::DaysThreshold(DaysThresholdReport {
                eligible: true,
                days_alive: 29,
                minimum_days: 7,
                reason: EligibilityReason::AgeAtOrAboveThreshold,
            })
        );
    }

    #[test]
    fn rejects_invalid_calculation_request_before_receipt_storage() {
        let error =
            execute_calculation_request(CalculationRequest::CalculateAge(CalculateAgeInput {
                birth_date: d(2026, 2, 29),
                as_of_date: d(2026, 6, 30),
            }))
            .unwrap_err();

        assert_eq!(error, "day is invalid for the given month and year");
    }
}
