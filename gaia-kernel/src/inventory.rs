//! Required components. Missing is not claimed.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub stack: &'static str,
    pub name: &'static str,
    pub present: bool,
}

pub fn required() -> Vec<Part> {
    vec![
        Part { stack: "os", name: "firmware", present: false },
        Part { stack: "os", name: "bootloader", present: false },
        Part { stack: "os", name: "kernel-process", present: true },
        Part { stack: "os", name: "scheduler", present: false },
        Part { stack: "os", name: "virtual-memory", present: false },
        Part { stack: "os", name: "driver", present: false },
        Part { stack: "os", name: "filesystem", present: false },
        Part { stack: "os", name: "permissions", present: false },
        Part { stack: "ai", name: "data", present: true },
        Part { stack: "ai", name: "model", present: false },
        Part { stack: "ai", name: "evaluation", present: false },
        Part { stack: "ai", name: "guard", present: true },
        Part { stack: "ai", name: "human-halt", present: true },
        Part { stack: "ai", name: "baseline", present: false },
    ]
}

pub fn grants_superintelligence() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firmware_and_the_baseline_are_missing() {
        let parts = required();
        assert!(parts.iter().any(|part| part.name == "firmware" && !part.present));
        assert!(parts.iter().any(|part| part.name == "baseline" && !part.present));
        assert!(!grants_superintelligence());
    }
}
