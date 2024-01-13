/*!
 * Command line tool to query crates.io index
 *
 * Install with cargo:
 * `cargo install crates-query`
 *
 * There are several query subcommands:
 *  * List a given crates dependencies
 *  * Get the minimum rust version
 *  * Query features available
 *  * Get versions published
 */

use std::{error::Error, fs::File, io::Write, process::Command};

use clap::{crate_authors, crate_description, crate_version, Args, Parser, Subcommand};
use crates_index::{SparseIndex, Version};
use tempfile::TempDir;

const LONG_VERSION: &str = concat!(
    crate_version!(),
    " ",
    crate_authors!(),
    "\n",
    crate_description!()
);

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = LONG_VERSION)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(flatten)]
    krate: Crate,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    Dependencies,
    RustVersion,
    Features,
    Versions,
}

#[derive(Args, Debug, Clone)]
pub struct Crate {
    name: String,
    #[arg(short, long)]
    ver: Option<String>,
}

/// Force update of sparse index
fn update_sparse_index(name: &str) -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;

    let mut f = File::create(dir.path().join(".log"))?;

    let output = Command::new("cargo")
        .current_dir(&dir)
        .args(["new", "temp"])
        .output()?;
    f.write_all(&output.stderr)?;
    let output = Command::new("cargo")
        .current_dir(dir.path().join("temp"))
        .args(["add", name])
        .output()?;
    f.write_all(&output.stderr)?;

    Ok(())
}

/// Get crate name and version if exists in index
fn get_crate_and_version(
    index: &SparseIndex,
    name: String,
    ver: Option<String>,
) -> Option<(crates_index::Crate, Version)> {
    match index.crate_from_cache(&name) {
        Ok(krate) => match ver {
            Some(v) => krate
                .versions()
                .iter()
                .find(|kv| kv.version() == v)
                .map(|found_version| (krate.clone(), found_version.clone())),
            None => Some((
                krate.clone(),
                krate.highest_normal_version().unwrap().clone(),
            )),
        },
        Err(_) => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let index = crates_index::SparseIndex::new_cargo_default()?;

    let (krate, version) = {
        // If `must_update` is false, check if the index already has the info we need
        let must_update = cli.command == Commands::Versions || cli.krate.ver.is_none();
        if must_update {
            update_sparse_index(&cli.krate.name)?;
            get_crate_and_version(&index, cli.krate.name.clone(), cli.krate.ver.clone()).unwrap()
        } else {
            match get_crate_and_version(&index, cli.krate.name.clone(), cli.krate.ver.clone()) {
                Some(info) => info,
                None => {
                    update_sparse_index(&cli.krate.name)?;
                    get_crate_and_version(&index, cli.krate.name.clone(), cli.krate.ver.clone())
                        .unwrap()
                }
            }
        }
    };

    match cli.command {
        Commands::Dependencies => {
            println!("{} {} dependencies:\n", version.name(), version.version());
            for dep in version.dependencies() {
                println!("{} {}", dep.crate_name(), dep.requirement());
            }
        }
        Commands::Features => {
            println!("{} {} features:\n", version.name(), version.version());
            for key in version.features().keys() {
                println!("{}", key);
            }
        }
        Commands::RustVersion => {
            println!("{} {} rust version:\n", version.name(), version.version());
            match version.rust_version() {
                Some(v) => println!("Minimum Rust Version: {v}"),
                None => println!("None"),
            }
        }
        Commands::Versions => {
            println!("{} versions:\n", version.name());
            let mut versions: Vec<_> = krate
                .versions()
                .iter()
                .map(|v| semver::Version::parse(v.version()).unwrap())
                .collect();
            versions.sort();
            for version in versions {
                println!("{}", version);
            }
        }
    }

    Ok(())
}
