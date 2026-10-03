//! Rising variance on a series. A warning is not published without a confirmation.

pub fn variance(values: &[f32]) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let mean = values.iter().sum::<f32>() / values.len() as f32;
    values.iter().map(|value| (value - mean).powi(2)).sum::<f32>() / values.len() as f32
}

pub fn rising(values: &[f32]) -> Result<bool, &'static str> {
    if values.len() < 4 {
        return Err("series is too short");
    }
    let mid = values.len() / 2;
    Ok(variance(&values[mid..]) > variance(&values[..mid]))
}

pub fn publish(rising: bool, confirmed: Option<bool>) -> Result<&'static str, &'static str> {
    if !rising {
        return Ok("no warning");
    }
    match confirmed {
        Some(true) => Ok("confirmed"),
        Some(false) => Ok("refuted"),
        None => Err("a warning is not published without a confirmation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rising_variance_is_not_published_without_confirmation() {
        let series = [1.0, 1.1, 0.9, 1.0, 0.0, 3.0, 0.2, 2.8];
        assert!(rising(&series).unwrap());
        assert_eq!(publish(true, None), Err("a warning is not published without a confirmation"));
        assert_eq!(publish(true, Some(false)), Ok("refuted"));
    }
}
