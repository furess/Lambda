use std::collections::HashSet;
use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

const ART: &[&str] = &[
    r"       /\       ",
    r"      /  \      ",
    r"     / /\ \     ",
    r"    / /  \ \    ",
    r"   /_/    \_\   ",
];

pub fn print_banner() {
    let mut info = vec![format!("lambda {}", env!("CARGO_PKG_VERSION"))];

    if let Some(cpu) = cpu_model() {
        info.push(format!("CPU   {cpu}"));
    }

    let cores = cpu_cores();
    if cores > 0 {
        info.push(format!("Cores {cores}"));
    }

    let usage = cpu_usage()
        .map(|percent| format!("{percent:.1}%"))
        .unwrap_or_else(|| "n/a".to_string());
    info.push(format!("Usage {usage} (200 ms sample)"));

    if let Some((used, total, percent)) = memory() {
        info.push(format!("RAM   {used:.1} / {total:.1} GiB ({percent:.0}%)"));
    }

    let gpus = gpu_names();
    if gpus.is_empty() {
        info.push("GPU   not detected".to_string());
    } else {
        for gpu in gpus {
            info.push(format!("GPU   {gpu}"));
        }
    }

    let width = ART
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let rows = ART.len().max(info.len());

    for index in 0..rows {
        let art = ART.get(index).copied().unwrap_or("");
        let text = info.get(index).map(String::as_str).unwrap_or("");

        if index < ART.len() {
            println!("{art:<width$}  {text}", width = width);
        } else {
            println!("{:<width$}  {text}", "", width = width);
        }
    }
}

fn cpu_model() -> Option<String> {
    let text = fs::read_to_string("/proc/cpuinfo").ok()?;
    text.lines()
        .find_map(|line| line.strip_prefix("model name"))
        .and_then(|rest| rest.split_once(':'))
        .map(|(_, value)| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn cpu_cores() -> usize {
    let Ok(text) = fs::read_to_string("/proc/cpuinfo") else {
        return 0;
    };

    text.lines()
        .filter(|line| line.starts_with("processor"))
        .count()
}

fn cpu_times() -> Option<(u64, u64)> {
    let text = fs::read_to_string("/proc/stat").ok()?;
    let line = text.lines().next()?;
    let fields: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|value| value.parse().ok())
        .collect();

    if fields.len() < 4 {
        return None;
    }

    let idle = fields[3] + fields.get(4).copied().unwrap_or(0);
    let total: u64 = fields.iter().sum();
    Some((total, idle))
}

fn cpu_usage() -> Option<f64> {
    let (total_before, idle_before) = cpu_times()?;
    thread::sleep(Duration::from_millis(200));
    let (total_after, idle_after) = cpu_times()?;

    let total_delta = total_after.saturating_sub(total_before) as f64;
    let idle_delta = idle_after.saturating_sub(idle_before) as f64;

    if total_delta <= 0.0 {
        return None;
    }

    Some((1.0 - idle_delta / total_delta) * 100.0)
}

fn memory() -> Option<(f64, f64, f64)> {
    let text = fs::read_to_string("/proc/meminfo").ok()?;
    let mut total_kib = None;
    let mut available_kib = None;

    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let Some(key) = parts.next() else {
            continue;
        };
        let value = parts.next().and_then(|value| value.parse::<u64>().ok());

        match key {
            "MemTotal:" => total_kib = value,
            "MemAvailable:" => available_kib = value,
            _ => {}
        }
    }

    let total_kib = total_kib?;
    let available_kib = available_kib?;
    let used_kib = total_kib.saturating_sub(available_kib);
    let total = total_kib as f64 / 1024.0 / 1024.0;
    let used = used_kib as f64 / 1024.0 / 1024.0;
    let percent = if total > 0.0 {
        used / total * 100.0
    } else {
        0.0
    };

    Some((used, total, percent))
}

fn gpu_names() -> Vec<String> {
    let mut gpus = lspci_gpus();
    if gpus.is_empty() {
        gpus = sysfs_gpus();
    }
    gpus
}

fn lspci_gpus() -> Vec<String> {
    let Ok(output) = Command::new("lspci").output() else {
        return Vec::new();
    };

    if !output.status.success() {
        return Vec::new();
    }

    let text = String::from_utf8_lossy(&output.stdout);
    text.lines()
        .filter(|line| {
            let lower = line.to_ascii_lowercase();
            lower.contains("vga compatible controller")
                || lower.contains("3d controller")
                || lower.contains("display controller")
        })
        .filter_map(|line| line.splitn(3, ':').nth(2))
        .map(clean_gpu_name)
        .filter(|name| !name.is_empty())
        .collect()
}

fn clean_gpu_name(raw: &str) -> String {
    let raw = raw.trim();
    match raw.rsplit_once(" (rev ") {
        Some((name, _)) => name.trim().to_string(),
        None => raw.to_string(),
    }
}

fn sysfs_gpus() -> Vec<String> {
    let Ok(entries) = fs::read_dir("/sys/class/drm") else {
        return Vec::new();
    };

    let mut seen = HashSet::new();
    let mut gpus = Vec::new();

    for entry in entries.flatten() {
        let uevent = entry.path().join("device").join("uevent");
        let Ok(text) = fs::read_to_string(uevent) else {
            continue;
        };

        let mut driver = None;
        let mut pci_id = None;
        for line in text.lines() {
            if let Some(value) = line.strip_prefix("DRIVER=") {
                driver = Some(value.trim().to_string());
            }
            if let Some(value) = line.strip_prefix("PCI_ID=") {
                pci_id = Some(value.trim().to_string());
            }
        }

        let Some(pci_id) = pci_id else {
            continue;
        };
        if !seen.insert(pci_id.clone()) {
            continue;
        }

        match driver {
            Some(driver) => gpus.push(format!("{driver} [{pci_id}]")),
            None => gpus.push(format!("PCI {pci_id}")),
        }
    }

    gpus
}
