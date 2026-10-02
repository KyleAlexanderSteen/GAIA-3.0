//! Forecast ledger for #1280. A score without a horizon and a form is NeedVerify.
//! No trainer. No 2027 forecast.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreForm {
    Binary,
    MultiCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LedgerRow {
    pub question: String,
    pub horizon_open: String,
    pub horizon_resolve: String,
    pub probability: f64,
    pub form: Option<ScoreForm>,
    pub resolution_source: String,
    pub outcome: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LedgerError {
    Missing(&'static str),
    Probability,
    Unresolved,
}

fn empty(value: &str) -> bool {
    value.trim().is_empty()
}

pub fn admit(row: &LedgerRow) -> Result<(), LedgerError> {
    if empty(&row.question) {
        return Err(LedgerError::Missing("question"));
    }
    if empty(&row.horizon_open) || empty(&row.horizon_resolve) {
        return Err(LedgerError::Missing("horizon"));
    }
    if row.form.is_none() {
        return Err(LedgerError::Missing("form"));
    }
    if empty(&row.resolution_source) {
        return Err(LedgerError::Missing("resolution_source"));
    }
    if !(0.0..=1.0).contains(&row.probability) {
        return Err(LedgerError::Probability);
    }
    Ok(())
}

/// Binary Brier term `(p - o)^2`. Multi-category is refused here.
pub fn binary_brier(row: &LedgerRow) -> Result<f64, LedgerError> {
    admit(row)?;
    if row.form != Some(ScoreForm::Binary) {
        return Err(LedgerError::Missing("binary form"));
    }
    let Some(outcome) = row.outcome else {
        return Err(LedgerError::Unresolved);
    };
    if outcome > 1 {
        return Err(LedgerError::Unresolved);
    }
    let gap = row.probability - f64::from(outcome);
    Ok(gap * gap)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row() -> LedgerRow {
        LedgerRow {
            question: "rain".into(),
            horizon_open: "2026-10-01".into(),
            horizon_resolve: "2026-10-02".into(),
            probability: 0.7,
            form: Some(ScoreForm::Binary),
            resolution_source: "fixture".into(),
            outcome: Some(1),
        }
    }

    #[test]
    fn missing_horizon_is_rejected() {
        let mut row = row();
        row.horizon_resolve.clear();
        assert_eq!(admit(&row), Err(LedgerError::Missing("horizon")));
    }

    #[test]
    fn missing_form_is_rejected() {
        let mut row = row();
        row.form = None;
        assert_eq!(admit(&row), Err(LedgerError::Missing("form")));
    }

    #[test]
    fn unresolved_binary_is_not_a_score() {
        let mut row = row();
        row.outcome = None;
        assert_eq!(binary_brier(&row), Err(LedgerError::Unresolved));
    }

    #[test]
    fn binary_score_is_squared_error() {
        let score = binary_brier(&row()).unwrap();
        assert!((score - 0.09).abs() < 1e-9);
    }
}
