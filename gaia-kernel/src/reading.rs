//! A local sensor reading. A missing source is not stored.

#[derive(Debug, Clone, PartialEq)]
pub struct Reading {
    pub lat: f64,
    pub lon: f64,
    pub celsius: f64,
    pub source: String,
    pub live: bool,
}

pub fn ingest(lat: f64, lon: f64, celsius: f64, source: &str) -> Result<Reading, &'static str> {
    if source.trim().is_empty() {
        return Err("a reading needs a source");
    }
    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return Err("coordinates are out of range");
    }
    Ok(Reading { lat, lon, celsius, source: source.to_string(), live: false })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_a_local_reading_and_refuses_a_missing_source() {
        let reading = ingest(29.42, -98.49, 31.0, "home-sensor").unwrap();
        assert!(!reading.live);
        assert_eq!(ingest(29.42, -98.49, 31.0, "").unwrap_err(), "a reading needs a source");
    }
}
