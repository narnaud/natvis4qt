use clap::Parser;
use console::style;
use std::path::PathBuf;

mod core;
use core::*;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "Command line utility to install Qt natvis files for Qt and MSVC"
)]
struct Args {
    #[clap(subcommand)]
    command: Command,
    /// Install or update without copying the files
    #[clap(long = "dry-run")]
    dry_run: bool,
}

#[derive(Parser)]
enum Command {
    /// Install the natvis files
    Install {},
    /// Update the natvis files
    Update {
        /// If the autoupdate preference is set to false, do nothing
        #[clap(long)]
        auto: bool,
    },
    /// Adjust natvis4qt's settings
    Set {
        /// Set the Qt installation root directory
        #[clap(long = "qt-root")]
        qt_root: Option<String>,
        /// Set autoupdate strategy
        #[clap(long)]
        autoupdate: Option<bool>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Install {} => install_natvis_files(args.dry_run),
        Command::Update { auto } => {
            if !auto || get_autoupdate() {
                update_natvis_files(args.dry_run)
            }
        }
        Command::Set {
            qt_root,
            autoupdate,
        } => set_preferences(qt_root, autoupdate),
    }
}

/// Set the preferences.
pub fn set_preferences(qt_root: Option<String>, autoupdate: Option<bool>) {
    if qt_root.is_none() && autoupdate.is_none() {
        println!("qt-root         {}", get_qt_root());
        println!("autoupdate      {}", get_autoupdate());
        return;
    }
    if let Some(qt_root) = qt_root {
        set_qt_root(qt_root.as_str());
    }
    if let Some(autoupdate) = autoupdate {
        set_autoupdate(autoupdate);
    }
}

/// Update the natvis files.
fn update_natvis_files(dry_run: bool) {
    let qt_root = get_default_qt_root();
    let keys = get_install_keys();
    if qt_root.is_none() || keys.is_none() {
        println!("Nothing to do!");
        return;
    }
    ui_finalize(keys.unwrap(), get_natvis_info(qt_root.unwrap()), dry_run);
}

/// Install the natvis files.
fn install_natvis_files(dry_run: bool) {
    cliclack::intro(style("Natvis installation for Qt").on_green().black()).unwrap();

    cliclack::log::info(format!(
        "This tool will install natvis files for Qt and MSVC\nExisting natvis files will be {}!",
        style("overwritten").yellow().bold()
    ))
    .unwrap();

    let qt_root = ui_get_qt_root();
    if qt_root.is_err() {
        ui_outro_error();
        return;
    }

    let natvis_info = get_natvis_info(qt_root.unwrap());
    let keys = match ui_get_install_keys(&natvis_info) {
        Ok(keys) => keys,
        Err(_) => {
            ui_outro_error();
            return;
        }
    };
    set_install_keys(&keys);

    ui_finalize(keys, natvis_info, dry_run);
}

/// UI: finalize the installation.
fn ui_finalize(keys: Vec<String>, natvis_info: Vec<NatvisInfo>, dry_run: bool) {
    let natvis_installs = natvis_info
        .iter()
        .filter(|info| keys.contains(&info.key))
        .collect::<Vec<_>>();

    let success = ui_install_natvis_files(natvis_installs, dry_run);
    match success {
        Ok(_) => cliclack::outro(style("Success!").green().bold()).unwrap(),
        Err(_) => ui_outro_error(),
    }
}

/// UI: ask the user for the Qt installation root.
fn ui_get_qt_root() -> Result<PathBuf, std::io::Error> {
    let default_root = get_default_qt_root();

    match default_root {
        Some(root) => {
            cliclack::log::info(format!(
                "Default Qt installation root found in {}",
                style(root.to_str().unwrap()).cyan()
            ))?;
            Ok(root)
        }
        None => {
            let qt_root: String = cliclack::input("Qt installation root?")
                .placeholder("C:\\Qt")
                .validate(|input: &String| {
                    if PathBuf::from(input).exists() {
                        Ok(())
                    } else {
                        Err("Please enter a valid path")
                    }
                })
                .interact()?;
            set_qt_root(qt_root.as_str());
            Ok(PathBuf::from(qt_root.as_str()))
        }
    }
}

/// UI: ask the user to select the directories to install the natvis files.
/// Returns the selected directories keys.
fn ui_get_install_keys(dirs: &[NatvisInfo]) -> Result<Vec<String>, std::io::Error> {
    let keys = match get_install_keys() {
        Some(keys) => keys,
        None => dirs.iter().map(|info| info.key.clone()).collect::<Vec<_>>(),
    };

    let dirs_for_multiselect = dirs
        .iter()
        .map(|info| {
            (
                info.key.clone(),
                info.name.as_str(),
                info.path.to_str().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    cliclack::multiselect("Select directories to install natvis files")
        .initial_values(keys)
        .items(&dirs_for_multiselect)
        .interact()
}

/// UI: show error outro message.
fn ui_outro_error() {
    cliclack::outro(format!(
        "Problems? {}\n",
        style("https://github.com/narnaud/natvis4qt/issues")
            .cyan()
            .underlined()
    ))
    .unwrap();
    std::process::exit(0)
}

/// UI: install the natvis files.
fn ui_install_natvis_files(
    natvis_installs: Vec<&NatvisInfo>,
    dry_run: bool,
) -> Result<(), std::io::Error> {
    let overall_progress = cliclack::multi_progress("Copying natvis files...");

    let mut errors = Vec::new();
    for info in &natvis_installs {
        let spinner = overall_progress.add(cliclack::spinner());
        let file_string = if info.version.len() > 1 {
            format!(
                "{} Natvis files",
                info.version
                    .iter()
                    .map(|v| format!("Qt{}", v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            format!("Qt{} Natvis file", info.version.first().unwrap())
        };
        spinner.start(format!(
            "  Copying {} for {}",
            file_string,
            style(info.name.as_str()).cyan()
        ));
        if !dry_run && let Err(e) = copy_natvis_file(info) {
            errors.push(e);
        }
        spinner.stop(format!(
            "{} {} copied for {}",
            style("✓").green().bold(),
            file_string,
            style(info.name.as_str()).cyan()
        ));
    }
    overall_progress.stop();

    if !errors.is_empty() {
        for e in errors {
            cliclack::log::error(format!("{}", e)).unwrap();
        }
        return Err(std::io::Error::other(
            "Some errors occurred during the installation",
        ));
    }

    Ok(())
}
