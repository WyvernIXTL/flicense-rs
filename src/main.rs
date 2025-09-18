//               Copyright Adam McKellar 2025
// Distributed under the Boost Software License, Version 1.0.
//         (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use colored::ColoredString;
use colored::Colorize;
use license_fetcher::error::UnpackError;
use serde_json::to_string_pretty;

use license_fetcher::build::{
    config::ConfigBuilder, metadata::package_list, package_list_with_licenses,
};
use license_fetcher::read_package_list_from_out_dir;
use license_fetcher::PackageList;

fn err<T>(msg: T)
where
    T: AsRef<str>,
{
    eprintln!("{}", msg.as_ref().red());
    exit(1);
}

macro_rules! err {
    ($($arg:tt)*) => {
        {
            err(format!($($arg)*));
            unreachable!();
        }
    };
}

fn print_short_license_info(package_list: PackageList) {
    let mut license_map: HashMap<String, Vec<String>> = HashMap::new();
    for pck in package_list.iter() {
        if let Some(license) = pck.license_identifier.clone() {
            if !license_map.contains_key(&license) {
                license_map.insert(license, vec![pck.name.clone()]);
            } else {
                license_map
                    .get_mut(&license)
                    .unwrap()
                    .push(pck.name.clone());
            }
        }
    }
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut stdout_buffered = BufWriter::new(lock);
    for (license, packages) in license_map {
        write!(stdout_buffered, "{}: ", license.green()).unwrap();
        for pck in packages.iter().take(packages.len() - 1) {
            write!(stdout_buffered, "{}, ", pck).unwrap();
        }
        write!(stdout_buffered, "{}\n", packages.last().unwrap()).unwrap();
    }
    stdout_buffered.flush().unwrap();
}

fn check(val: bool) -> ColoredString {
    if val {
        "✓".green()
    } else {
        "✗".red()
    }
}

fn print_license_stats(package_list: PackageList) {
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut stdout_buffered = BufWriter::new(lock);
    writeln!(stdout_buffered, "{:<30}{:<30}", "name", "license found").unwrap();
    let total_count = package_list.len();
    let mut count = 0;
    for p in package_list.0.into_iter() {
        writeln!(
            stdout_buffered,
            "{:<30}{:<30}",
            p.name.blue(),
            check(p.license_text.is_some())
        )
        .unwrap();
        if p.license_text.is_some() {
            count += 1;
        }
    }
    let percentage = (count as f64 / total_count as f64) * 100.0;
    let percentage_str = if percentage > 98.0 {
        format!("{:.0}%", percentage).green()
    } else if percentage < 90.0 {
        format!("{:.0}%", percentage).red()
    } else {
        format!("{:.0}%", percentage).yellow()
    };
    writeln!(stdout_buffered, "\nlicense found: {}", percentage_str).unwrap();
    stdout_buffered.flush().unwrap();
}

/// CLI for printing license information of rust cargo projects to the terminal.
///
/// Cargo needs to be installed and be in the PATH.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional path to manifest dir (where Cargo.toml and Cargo.lock are). Defaults to current dir.
    #[arg(conflicts_with = "other")]
    manifest_dir_path: Option<PathBuf>,

    /// Output as yaml.
    #[arg(short, long, group = "mode")]
    yaml: bool,

    /// Output as json.
    #[arg(short, long, group = "mode")]
    json: bool,

    /// Outputs only a short overview.
    #[arg(short, long, group = "mode", group = "no-license-text")]
    short: bool,

    /// Outputs stats regarding how many licenses have been found and for what crates.
    #[arg(long, group = "mode", group = "no-license-text")]
    stats: bool,

    /// Omits outputting license text.
    #[arg(short, long, conflicts_with = "no-license-text")]
    omit_license_text: bool,

    /// Outputs license information regarding this software and it's dependencies.
    #[arg(short, long, group = "mode", group = "other")]
    license: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.license {
        let packages = match read_package_list_from_out_dir!() {
            Ok(e) => e,
            Err(e) => match e {
                UnpackError::Empty => err!("Failed to embed licenses. No licenses found."),
                _ => {
                    eprintln!("{:?}", e);
                    exit(1);
                }
            },
        };
        println!("{}", packages);
        return;
    }

    let config = ConfigBuilder::from_path(cli.manifest_dir_path.unwrap_or(".".into()))
        .cache(false)
        .build()
        .unwrap();

    let package_list = if !cli.omit_license_text {
        package_list_with_licenses(config).unwrap()
    } else {
        package_list(&config.metadata_config).unwrap()
    };

    if cli.yaml {
        println!("{}", serde_yml::to_string(&package_list).unwrap())
    } else if cli.json {
        println!("{}", to_string_pretty(&package_list).unwrap())
    } else {
        if cli.short {
            print_short_license_info(package_list);
        } else if cli.stats {
            print_license_stats(package_list);
        } else {
            let stdout = std::io::stdout();
            let lock = stdout.lock();
            let mut stdout_buffered = BufWriter::new(lock);
            write!(stdout_buffered, "{}", package_list).unwrap();
            stdout_buffered.flush().unwrap();
        }
    }
}
