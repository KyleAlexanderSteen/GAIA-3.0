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

pub fn copy_over_localhost(note: &str) -> Result<String, String> {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:0").map_err(|err| err.to_string())?;
    let addr = listener.local_addr().map_err(|err| err.to_string())?;
    let mut incoming = listener.incoming();
    let mut client = std::net::TcpStream::connect(addr).map_err(|err| err.to_string())?;
    client.write_all(note.as_bytes()).map_err(|err| err.to_string())?;
    client.shutdown(std::net::Shutdown::Write).map_err(|err| err.to_string())?;
    let mut server = incoming.next().ok_or("no connection")?.map_err(|err| err.to_string())?;
    let mut received = String::new();
    server.read_to_string(&mut received).map_err(|err| err.to_string())?;
    Ok(received)
}

#[cfg(test)]
mod localhost_tests {
    use super::*;

    #[test]
    fn a_note_crosses_localhost() {
        assert_eq!(copy_over_localhost("watered").unwrap(), "watered");
    }
}
