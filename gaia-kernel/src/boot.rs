//! Local boot path. It does not write firmware.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage {
    pub name: &'static str,
    pub done: bool,
    pub note: &'static str,
}

pub fn boot() -> Vec<Stage> {
    vec![
        Stage {
            name: "firmware",
            done: false,
            note: "not written",
        },
        Stage {
            name: "kernel",
            done: true,
            note: "this process",
        },
        Stage {
            name: "driver",
            done: true,
            note: "console probed",
        },
        Stage {
            name: "filesystem",
            done: true,
            note: "memory volume",
        },
        Stage {
            name: "scheduler",
            done: true,
            note: "two tasks ordered",
        },
        Stage {
            name: "guest",
            done: true,
            note: "image halted",
        },
    ]
}

pub fn probe(name: &str) -> Result<&'static str, &'static str> {
    match name {
        "console" => Ok("present"),
        "disk" => Err("no host disk claimed"),
        _ => Err("unknown driver"),
    }
}

pub fn schedule<'a>(tasks: &'a [(u8, &'a str)]) -> Vec<&'a str> {
    let mut ordered = tasks.to_vec();
    ordered.sort_by_key(|task| std::cmp::Reverse(task.0));
    ordered.into_iter().map(|task| task.1).collect()
}

pub fn volume_write(volume: &mut Vec<(String, String)>, path: &str, body: &str) {
    volume.retain(|entry| entry.0 != path);
    volume.push((path.to_string(), body.to_string()));
}

pub fn volume_read<'a>(volume: &'a [(String, String)], path: &str) -> Option<&'a str> {
    volume
        .iter()
        .find(|entry| entry.0 == path)
        .map(|entry| entry.1.as_str())
}

/// Guest image: push 20, push 22, add, halt.
pub fn run_guest() -> Result<i32, &'static str> {
    let image = [0x01, 20, 0x01, 22, 0x02, 0x00];
    let mut pc = 0;
    let mut stack = Vec::new();
    while pc < image.len() {
        match image[pc] {
            0x01 => {
                pc += 1;
                stack.push(image.get(pc).copied().ok_or("truncated push")?);
                pc += 1;
            }
            0x02 => {
                let right = stack.pop().ok_or("add underflow")?;
                let left = stack.pop().ok_or("add underflow")?;
                stack.push(left + right);
                pc += 1;
            }
            0x00 => return stack.pop().ok_or("halt with empty stack"),
            _ => return Err("bad opcode"),
        }
    }
    Err("guest did not halt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_does_not_write_firmware_and_the_guest_halts_at_42() {
        let stages = boot();
        assert!(!stages[0].done);
        assert_eq!(probe("disk"), Err("no host disk claimed"));
        assert_eq!(schedule(&[(1, "low"), (9, "high")]), vec!["high", "low"]);
        let mut volume = Vec::new();
        volume_write(&mut volume, "/etc/host", "local");
        assert_eq!(volume_read(&volume, "/etc/host"), Some("local"));
        assert_eq!(run_guest(), Ok(42));
    }
}
