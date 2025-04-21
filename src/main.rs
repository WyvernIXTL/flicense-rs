//               Copyright Adam McKellar 2025
// Distributed under the Boost Software License, Version 1.0.
//         (See accompanying file LICENSE or copy at
//          https://www.boost.org/LICENSE_1_0.txt)

use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{read_dir, read_to_string};
use std::io::prelude::*;
use std::io::{stdout, BufWriter};
use std::path::{absolute, PathBuf};
use std::process::exit;
use std::string::ToString;

use clap::Parser;
use color_eyre::eyre::Result;
use colored::Colorize;
use serde::Deserialize;
use serde_json::to_string_pretty;

use license_fetcher::build_script::generate_package_list_with_licenses_without_env_calls;
use license_fetcher::get_package_list_macro;
use license_fetcher::PackageList;

#[cfg(all(feature = "mimalloc", feature = "tikv-jemallocator"))]
compile_error!("Features `mimalloc` and `tikv-jemallocator` are mutually exclusive and cannot be enabled at the same time.");

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(feature = "tikv-jemallocator", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn err<T>(msg: T)
where
    T: ToString,
{
    eprintln!("{}", msg.to_string().red());
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

#[derive(Deserialize)]
struct CargoToml {
    package: CargoPackage,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
}

/// CLI for printing license information of rust cargo projects to the terminal.
///
/// Cargo needs to be installed and be in the PATH.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional path to manifest dir (where Cargo.toml and Cargo.lock are). Defaults to current dir.
    manifest_dir_path: Option<PathBuf>,

    /// Output as yaml.
    #[arg(short, long)]
    yaml: bool,

    /// Output as json.
    #[arg(short, long)]
    json: bool,

    /// Outputs only a short overview.
    #[arg(short, long)]
    short: bool,

    /// Omits outputting license text.
    #[arg(short, long)]
    omit_license_text: bool,

    /// Outputs license information regarding this software and it's dependencies.
    #[arg(short, long)]
    license: bool,
}

fn print_short_license_info(package_list: PackageList) -> Result<()> {
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
    let mut stdout_buffered = BufWriter::new(stdout());
    for (license, packages) in license_map {
        write!(stdout_buffered, "{}: ", license.green())?;
        for pck in packages.iter().take(packages.len() - 1) {
            write!(stdout_buffered, "{}, ", pck)?;
        }
        write!(stdout_buffered, "{}\n", packages.last().unwrap())?;
    }
    stdout_buffered.flush()?;

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if cli.license {
        let packages = get_package_list_macro!()?;
        println!("{}", packages);
        return Ok(());
    }

    let manifest_dir = match cli.manifest_dir_path {
        Some(path) => {
            if !path.try_exists()? {
                err!("Error: Path does not exist! Path: {:#?}", path);
            }
            let absolute_path = absolute(path)?;
            if !absolute_path.is_dir() {
                absolute_path
                    .parent()
                    .unwrap_or_else(|| {
                        err!("Error: Cannot find parent of path.");
                    })
                    .to_owned()
            } else {
                absolute_path
            }
        }
        None => current_dir()?,
    };

    assert!(manifest_dir.is_dir());

    let cargo_toml_path = read_dir(manifest_dir.clone())?
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter(|entry| entry.file_name().to_string_lossy() == "Cargo.toml")
        .next()
        .unwrap_or_else(|| err!("Error: Failed finding Cargo.toml file in dir."))
        .path();

    let cargo_toml: CargoToml = toml::from_str(&read_to_string(cargo_toml_path)?)?;
    let name = cargo_toml.package.name;

    let mut package_list = generate_package_list_with_licenses_without_env_calls(
        None,
        manifest_dir.as_os_str().to_owned(),
        name,
    );

    if cli.omit_license_text {
        for pkg in package_list.iter_mut() {
            pkg.license_text = None;
        }
    }

    if cli.yaml {
        println!("{}", serde_yml::to_string(&package_list)?)
    } else if cli.json {
        println!("{}", to_string_pretty(&package_list)?)
    } else {
        if cli.short {
            print_short_license_info(package_list)?;
        } else {
            let stdout = std::io::stdout();
            let lock = stdout.lock();
            let mut stdout_buffered = BufWriter::new(lock);
            write!(stdout_buffered, "{}", package_list)?;
            stdout_buffered.flush()?;
        }
    }

    Ok(())
}
