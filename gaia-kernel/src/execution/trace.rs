//! Local W3C Trace Context. Not OpenTelemetry. #734.
//! `traceparent` is formatted here. OTLP export is refused.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalTrace {
    pub trace_id: [u8; 16],
    pub span_id: [u8; 8],
    pub status_ok: bool,
}

impl LocalTrace {
    pub fn new(status_ok: bool) -> Self {
        Self {
            trace_id: fresh_id(),
            span_id: fresh_span(),
            status_ok,
        }
    }

    pub fn child(&self, status_ok: bool) -> Self {
        Self {
            trace_id: self.trace_id,
            span_id: fresh_span(),
            status_ok,
        }
    }

    /// W3C Trace Context Level 1. Flags stay `00`: not sampled, not exported.
    pub fn traceparent(&self) -> String {
        format!(
            "00-{}-{}-00",
            hex16(&self.trace_id),
            hex8(&self.span_id)
        )
    }

    pub fn export_otlp(&self) -> Result<(), &'static str> {
        let _ = self;
        Err("OTLP export is not implemented")
    }
}

pub fn status_ok(outcomes_ok: &[bool]) -> bool {
    !outcomes_ok.is_empty() && outcomes_ok.iter().all(|ok| *ok)
}

fn fresh_id() -> [u8; 16] {
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(1);
    let mut bytes = [0u8; 16];
    let raw = n.to_be_bytes();
    bytes[..8].copy_from_slice(&raw[..8]);
    bytes[8..].copy_from_slice(&raw[8..]);
    bytes[0] |= 0x01;
    if bytes == [0u8; 16] {
        bytes[15] = 1;
    }
    bytes
}

fn fresh_span() -> [u8; 8] {
    let n = fresh_id();
    let mut span = [0u8; 8];
    span.copy_from_slice(&n[8..]);
    if span == [0u8; 8] {
        span[7] = 1;
    }
    span
}

fn hex16(bytes: &[u8; 16]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hex8(bytes: &[u8; 8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traceparent_is_w3c_and_not_sampled() {
        let trace = LocalTrace {
            trace_id: [1u8; 16],
            span_id: [2u8; 8],
            status_ok: false,
        };
        let header = trace.traceparent();
        assert_eq!(header.len(), 55);
        assert!(header.starts_with("00-"));
        assert!(header.ends_with("-00"));
        assert!(header.contains("-0202020202020202-"));
    }

    #[test]
    fn failure_is_not_success_and_export_is_refused() {
        assert!(!status_ok(&[]));
        assert!(!status_ok(&[true, false]));
        assert!(status_ok(&[true]));
        let trace = LocalTrace::new(false);
        assert_eq!(trace.export_otlp(), Err("OTLP export is not implemented"));
        assert!(!trace.status_ok);
    }

    #[test]
    fn child_keeps_trace_id() {
        let parent = LocalTrace::new(true);
        let child = parent.child(false);
        assert_eq!(child.trace_id, parent.trace_id);
        assert_ne!(child.span_id, parent.span_id);
        assert!(!child.status_ok);
    }
}
