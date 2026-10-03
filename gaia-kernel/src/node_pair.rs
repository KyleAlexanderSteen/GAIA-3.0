//! Two local nodes. A split keeps the last copy.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: &'static str,
    pub records: BTreeMap<String, String>,
    pub online: bool,
}

pub fn pair() -> (Node, Node) {
    (
        Node { id: "a", records: BTreeMap::new(), online: true },
        Node { id: "b", records: BTreeMap::new(), online: true },
    )
}

pub fn write(node: &mut Node, key: &str, value: &str) {
    node.records.insert(key.to_string(), value.to_string());
}

pub fn replicate(from: &Node, to: &mut Node) -> Result<usize, &'static str> {
    if !from.online || !to.online {
        return Err("partition");
    }
    for (key, value) in &from.records {
        to.records.insert(key.clone(), value.clone());
    }
    Ok(to.records.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_note_copies_and_a_split_keeps_the_last_copy() {
        let (mut a, mut b) = pair();
        write(&mut a, "plant", "watered");
        assert_eq!(replicate(&a, &mut b).unwrap(), 1);
        assert_eq!(b.records.get("plant").map(String::as_str), Some("watered"));
        b.online = false;
        write(&mut a, "plant", "later");
        assert_eq!(replicate(&a, &mut b), Err("partition"));
        assert_eq!(b.records.get("plant").map(String::as_str), Some("watered"));
    }
}
