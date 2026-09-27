//! #1064 discovery budget. Ranking cannot grant authority.

#[derive(Debug, Clone)]
pub struct Descriptor {
    pub server: &'static str,
    pub name: &'static str,
    pub schema_chars: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct DiscoveryCaps {
    pub max_tools: usize,
    pub max_chars: usize,
}

impl Default for DiscoveryCaps {
    fn default() -> Self {
        Self {
            max_tools: 8,
            max_chars: 2048,
        }
    }
}

pub fn discover_tools(all: &[Descriptor], caps: DiscoveryCaps) -> Result<Vec<Descriptor>, &'static str> {
    let mut names = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    let mut chars = 0usize;
    for d in all {
        let key = format!("{}::{}", d.server, d.name);
        if !names.insert(key) {
            return Err("collision");
        }
        if out.len() >= caps.max_tools {
            break;
        }
        if chars + d.schema_chars > caps.max_chars {
            break;
        }
        chars += d.schema_chars;
        out.push(d.clone());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caps_truncate() {
        let all = vec![
            Descriptor {
                server: "a",
                name: "one",
                schema_chars: 10,
            },
            Descriptor {
                server: "a",
                name: "two",
                schema_chars: 10,
            },
        ];
        let got = discover_tools(&all, DiscoveryCaps { max_tools: 1, max_chars: 2048 }).unwrap();
        assert_eq!(got.len(), 1);
    }

    #[test]
    fn collision_fails() {
        let all = vec![
            Descriptor {
                server: "a",
                name: "t",
                schema_chars: 1,
            },
            Descriptor {
                server: "a",
                name: "t",
                schema_chars: 1,
            },
        ];
        assert_eq!(discover_tools(&all, DiscoveryCaps::default()), Err("collision"));
    }
}
