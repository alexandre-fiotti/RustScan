use super::ScanConfig;
use crate::input::{Opts, PortRange};
use clap::Parser;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BuildOptsFromScanConfigError {
    NoTargets,
    ClapParse(String),
}

impl Display for BuildOptsFromScanConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTargets => write!(f, "No targets provided"),
            Self::ClapParse(s) => write!(f, "Failed to parse options: {}", s),
        }
    }
}

impl Error for BuildOptsFromScanConfigError {}

/// Build Opts by delegating parsing to clap (Opts::parse_from), to ensure identical semantics
pub fn build_opts_from_scan_config(cfg: &ScanConfig) -> Result<Opts, BuildOptsFromScanConfigError> {
    // Prefer current input buffer; fallback to confirmed
    let addresses_text = if !cfg.targets_input.is_empty() {
        cfg.targets_input.text().to_string()
    } else if !cfg.targets.is_empty() {
        cfg.targets.join(",")
    } else {
        String::new()
    };

    if addresses_text.trim().is_empty() {
        return Err(BuildOptsFromScanConfigError::NoTargets);
    }

    let mut argv: Vec<String> = vec!["rustscan".to_string(), "--addresses".to_string()];
    argv.push(addresses_text);

    // Ports: prefer range if input looks like a single hyphenated range without commas
    // Prefer current ports input; fallback to confirmed ports
    let ports_candidate = if !cfg.ports_input.is_empty() {
        Some(cfg.ports_input.text().to_string())
    } else {
        cfg.ports.clone()
    };

    if let Some(ports_str) = ports_candidate {
        let trimmed = ports_str.trim();
        if !trimmed.is_empty() {
            if !trimmed.contains(',') && trimmed.contains('-') {
                argv.push("--range".to_string());
                argv.push(trimmed.to_string());
            } else {
                argv.push("--ports".to_string());
                argv.push(trimmed.to_string());
            }
        }
    }

    // Timeout and batch size come from cfg defaults for now
    argv.push("--timeout".to_string());
    argv.push(cfg.timeout.to_string());
    argv.push("--batch-size".to_string());
    argv.push(cfg.batch_size.to_string());

    // Parse via clap
    let mut opts = match Opts::try_parse_from(argv.clone()) {
        Ok(o) => o,
        Err(e) => return Err(BuildOptsFromScanConfigError::ClapParse(format!("{}", e))),
    };

    // Mirror CLI default range behavior
    if opts.ports.is_none() && opts.range.is_none() {
        opts.range = Some(PortRange {
            start: 1,
            end: 65535,
        });
    }

    Ok(opts)
}
