use chrono::{Datelike, Days, NaiveDate};
use std::ops::RangeInclusive;

use payroll_domain::PaymentSchedule;

#[derive(Debug, Clone)]
pub struct MonthlySchedule;
impl MonthlySchedule {
    pub fn is_last_day_of_month(&self, date: NaiveDate) -> bool {
        date.month() != date.checked_add_days(Days::new(1)).unwrap().month()
    }
}
impl PaymentSchedule for MonthlySchedule {
    fn is_pay_date(&self, date: NaiveDate) -> bool {
        self.is_last_day_of_month(date)
    }
    fn get_pay_period(&self, pay_date: NaiveDate) -> RangeInclusive<NaiveDate> {
        pay_date.with_day(1).unwrap()..=pay_date
    }
}
