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

use clap::{Args, Parser, Subcommand};
use tempfile::TempDir;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[clap(flatten)]
    krate: Crate,
}

#[derive(Subcommand, Debug, Clone)]
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    update_sparse_index(&cli.krate.name)?;

    let index = crates_index::SparseIndex::new_cargo_default()?;
    let krate = index.crate_from_cache(&cli.krate.name)?;
    let version = match cli.krate.ver {
        Some(v) => krate
            .versions()
            .iter()
            .find(|kv| kv.version() == v)
            .expect("Could not find version"),
        None => krate.highest_normal_version().unwrap(),
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
            for version in krate.versions().iter().map(|v| v.version()) {
                println!("{version}");
            }
        }
    }

    Ok(())
}
