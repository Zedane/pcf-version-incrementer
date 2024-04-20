use std::env;
use std::path;

use clap::{Args, Parser, Subcommand};

use pcf::package::PackageFile;
use pcf::solution::SolutionFile;
use pcf::manifest::ManifestFile;
use pcf::{Version, FileHandler};

pub mod pcf;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// The pcf project directory
    #[clap(short, long, global = true)]
    path: Option<path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows the current version
    Status(StatusArgs),

    /// Increments minor version by one
    Increment(IncrementArgs),

    /// Set a specific verison
    Set(SetArgs),
}

#[derive(Args, Debug)]
struct StatusArgs {
    /// Show all file version
    #[clap(short, long, default_value_t = false)]
    verbose: bool
}

#[derive(Args, Debug)]
struct IncrementArgs {
    /// Increment major version instead
    #[clap(long, default_value_t = false)]
    major: bool,
    /// Increment minor version instead
    #[clap(long, default_value_t = false)]
    patch: bool,
}

#[derive(Args, Debug)]
struct SetArgs {
    /// Major version
    #[clap(long)]
    major: Option<u16>,
    /// Minor version
    #[clap(long)]
    minor: Option<u16>,
    /// Patch version
    #[clap(long)]
    patch: Option<u16>
}

fn main() {
    let cli = Cli::parse();

    let working_dir: path::PathBuf;

    if let Some(path) = cli.path {
        working_dir = path;
    } else {
        if let Ok(path) = env::current_dir() {
            working_dir = path;
        } else {
            eprintln!("Could not load directory");
            return;
        }
    }

    match &cli.command {
        Commands::Status(args) => {
            show_status(working_dir, args.verbose);
        },
        Commands::Increment(args) => {
            increment(working_dir, args.major, args.patch);
        },
        Commands::Set(args) => {
            set_version(&working_dir, args.major, args.minor, args.patch);
        }
    }
}

fn show_status(path: path::PathBuf, verbose: bool) {
    if let Some(manifest) = ManifestFile::get(path.clone()) {
        println!("Current Version: {}", &manifest.version.to_string());
        if verbose {
            manifest.show_status();
        }
    } else {
        eprintln!("Could not read Manifest file");
        return;
    }
    
    if verbose {
        if let Some(package) = PackageFile::get(path.clone()) {
            package.show_status();
        }
        
        if let Some(solution) = SolutionFile::get(path.clone()) {
            solution.show_status();
        }
    }
}

fn increment(path: path::PathBuf, major: bool, patch: bool) {
    if let Some(manifest) = ManifestFile::get(path.clone()) {

        let mut version = manifest.version.clone();

        if major {
            version.set_major(manifest.version.major + 1);
        } else if patch {
            version.set_patch(manifest.version.patch + 1);
        } else {
            version.set_minor(manifest.version.minor + 1);
        }

        println!("Increment: {} -> {}", manifest.version.to_string(), version.to_string());

        internal_update(&path, manifest, &mut version);
        
    } else {
        eprintln!("Could not load manifest file");
        return;
    }
}

fn set_version(path: &path::PathBuf, major: Option<u16>, minor: Option<u16>, patch: Option<u16>) {
    if let Some(manifest) = ManifestFile::get(path.clone()) {
        let mut version = manifest.version.clone();

        if let Some(major) = major {
            version.major = major;
        }

        if let Some(minor) = minor {
            version.minor = minor;
        }

        if let Some(patch) = patch {
            version.patch = patch;
        }

        println!("Set: {} -> {}", manifest.version.to_string(), version.to_string());
        
        internal_update(&path, manifest, &mut version);
    } else {
        eprintln!("Could not load manifest file");
        return;
    }
}

fn internal_update(path: &path::PathBuf, manifest: ManifestFile, version:&mut Version) {
    if let Ok(_) = manifest.update_version(&version) {
        println!("Manifest updated");
    } else {
        eprintln!("Could not update manifest file");
        return;
    }

    if let Some(package) = PackageFile::get(path.clone()) {
        match package.update_version(&version) {
            Err(_) => {
                eprintln!("Error updating package file");
            },
            Ok(_) => {
                println!("Package updated");
            }
        };
    }

    if let Some(solution) = SolutionFile::get(path.clone()) {
        // version.set_minor(version.patch);
        match solution.update_version(&version) {
            Err(_) => {
                eprintln!("Error updating solution file");
            },
            Ok(_) => {
                println!("Solution updated");
            }
        };
    }
}