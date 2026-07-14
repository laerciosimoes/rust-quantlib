use crate::{
    date::Date,
    event::Event,
    settings::Settings,
};

pub trait Event {
    fn date(&self) -> Date;

    fn has_occurred(
        &self,
        reference_date: Option<Date>,
        include_reference_date: Option<bool>,
    ) -> bool {
        let reference_date = reference_date
            .or_else(Settings::evaluation_date)
            .expect("evaluation date not set");

        let event_date = self.date();

        if event_date < reference_date {
            true
        } else if event_date > reference_date {
            false
        } else {
            !include_reference_date.unwrap_or(false)
        }
    }
}

/// A payment occurring on a given date.
///
/// Equivalent to QuantLib's abstract CashFlow class.
pub trait CashFlow: Event {
    /// Monetary amount of the cash flow.
    fn amount(&self) -> f64;

    /// Optional ex-coupon date.
    fn ex_coupon_date(&self) -> Option<Date> {
        None
    }

    /// Returns whether the cash flow has already occurred.
    ///
    /// This follows the same logic as QuantLib.
    fn has_occurred(
        &self,
        reference_date: Option<Date>,
        mut include_reference_date: Option<bool>,
    ) -> bool {
        // Fast path
        if let Some(reference_date) = reference_date {
            let cashflow_date = self.date();

            if reference_date < cashflow_date {
                return false;
            }

            if cashflow_date < reference_date {
                return true;
            }
        }

        let evaluation_date = Settings::evaluation_date();

        if reference_date.is_none() || reference_date == evaluation_date {
            if let Some(include_today) =
                Settings::include_todays_cash_flows()
            {
                include_reference_date = Some(include_today);
            }
        }

        Event::has_occurred(
            self,
            reference_date,
            include_reference_date,
        )
    }

    /// Returns true if the cash flow is already trading ex-coupon.
    fn trading_ex_coupon(
        &self,
        reference_date: Option<Date>,
    ) -> bool {
        let Some(ex_coupon_date) = self.ex_coupon_date() else {
            return false;
        };

        let reference_date = reference_date
            .or_else(Settings::evaluation_date)
            .expect("evaluation date not set");

        ex_coupon_date <= reference_date
    }
}

pub struct SimpleCashFlow {
    amount: f64,
    payment_date: Date,
}

impl Event for SimpleCashFlow {
    fn date(&self) -> Date {
        self.payment_date
    }
}

impl CashFlow for SimpleCashFlow {
    fn amount(&self) -> f64 {
        self.amount
    }
}