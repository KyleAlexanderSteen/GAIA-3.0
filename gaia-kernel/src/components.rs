//! Local OS and AI components. Firmware and a baseline stay absent.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    pub virtual_addr: u32,
    pub physical_addr: u32,
}

pub fn schedule<'a>(tasks: &'a [(u8, &'a str)]) -> Vec<&'a str> {
    let mut ordered = tasks.to_vec();
    ordered.sort_by_key(|task| std::cmp::Reverse(task.0));
    ordered.into_iter().map(|task| task.1).collect()
}

pub fn map_page(virtual_addr: u32) -> Page {
    Page { virtual_addr, physical_addr: virtual_addr.wrapping_add(0x1000) }
}

pub fn probe_driver(name: &str) -> Result<&'static str, &'static str> {
    match name {
        "console" => Ok("present"),
        "disk" => Err("no host disk claimed"),
        _ => Err("unknown driver"),
    }
}

pub fn write_file(volume: &mut Vec<(String, String)>, path: &str, body: &str) {
    volume.retain(|entry| entry.0 != path);
    volume.push((path.to_string(), body.to_string()));
}

pub fn allow(user: &str, path: &str) -> bool {
    user == "owner" || path.starts_with("/public/")
}

pub fn evaluate(score: Option<f32>, baseline: Option<f32>) -> Result<f32, &'static str> {
    let score = score.ok_or("no score")?;
    let baseline = baseline.ok_or("no baseline")?;
    Ok(score - baseline)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn components_run_and_a_missing_baseline_is_not_a_win() {
        assert_eq!(schedule(&[(1, "low"), (3, "high")]), vec!["high", "low"]);
        assert_eq!(map_page(16).physical_addr, 0x1010);
        assert_eq!(probe_driver("disk"), Err("no host disk claimed"));
        let mut volume = Vec::new();
        write_file(&mut volume, "/etc/host", "local");
        assert!(!allow("guest", "/etc/host"));
        assert_eq!(evaluate(Some(1.0), None), Err("no baseline"));
    }
}
