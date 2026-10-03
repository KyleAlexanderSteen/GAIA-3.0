//! A local person record. Not a resolved DID and not a wearable.

use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub id: String,
    pub language: String,
    pub note: String,
}

pub fn local_id(name: &str) -> Result<String, &'static str> {
    if name.trim().is_empty() {
        return Err("a name is required");
    }
    Ok(format!("local:{}", name.trim()))
}

pub fn language(tag: &str) -> Result<&'static str, &'static str> {
    match tag {
        "en" => Ok("en"),
        "es" => Ok("es"),
        "fr" => Ok("fr"),
        "de" => Ok("de"),
        "pt" => Ok("pt"),
        "und" => Ok("und"),
        _ => Err("language is not in the local set"),
    }
}

pub fn health(source: &str, beats: Option<u32>) -> Result<u32, &'static str> {
    if source.trim().is_empty() {
        return Err("a health note needs a source");
    }
    beats.ok_or("no wearable reading")
}

pub fn earth_link(live: bool) -> Result<&'static str, &'static str> {
    if live {
        return Err("no live planetary feed");
    }
    Ok("local reading only")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_local_id_is_not_a_did_and_a_wearable_is_missing() {
        assert_eq!(local_id("ada").unwrap(), "local:ada");
        assert!(language("zh").is_err());
        assert_eq!(health("watch", None), Err("no wearable reading"));
        assert_eq!(earth_link(true), Err("no live planetary feed"));
        let path = std::env::temp_dir().join(format!("gaia-vault-{}", std::process::id()));
        vault_write(&path, "only-local").unwrap();
        assert_eq!(vault_read(&path).unwrap(), "only-local");
        assert!(health_note("watch", 72).unwrap().contains("wearable=false"));
        let _ = fs::remove_file(path);
    }
}

pub fn vault_write(path: &Path, note: &str) -> Result<(), String> {
    if note.trim().is_empty() {
        return Err("empty vault note".into());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    fs::write(path, note.trim()).map_err(|err| err.to_string())
}

pub fn vault_read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|_| "no vault".to_string())
}

pub fn health_note(source: &str, beats: u32) -> Result<String, &'static str> {
    if source.trim().is_empty() {
        return Err("a health note needs a source");
    }
    if beats == 0 {
        return Err("no wearable reading");
    }
    Ok(format!("source={source} beats={beats} wearable=false"))
}