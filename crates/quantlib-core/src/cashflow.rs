pub trait CashFlow: Event {
    fn amount(&self) -> f64;

    fn ex_coupon_date(&self) -> Option<Date> {
        None
    }

    fn has_occurred(
        &self,
        context: &EvaluationContext,
        reference_date: Option<Date>,
        include_reference_date: Option<bool>,
    ) -> bool {
        todo!()
    }
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum QuantError {
    #[error("invalid date: {0}")]
    InvalidDate(String),

    #[error("missing evaluation date")]
    MissingEvaluationDate,

    #[error("invalid cash flow amount: {0}")]
    InvalidCashFlowAmount(f64),

    #[error("pricing failed: {0}")]
    Pricing(String),
}