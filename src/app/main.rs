use console::style;
use std::path::PathBuf;

mod core;
use core::*;

fn main() {
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

    let natvis_installs = get_natvis_installs(qt_root.unwrap());

    let install_keys = ui_get_install_keys(&natvis_installs);
    if install_keys.is_err() {
        ui_outro_error();
        return;
    }

    let natvis_installs = natvis_installs
        .iter()
        .filter(|info| install_keys.as_ref().unwrap().contains(&info.key.as_str()))
        .collect::<Vec<_>>();

    let success = ui_install_natvis_files(natvis_installs);

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
fn ui_get_install_keys(dirs: &[NatvisInfo]) -> Result<Vec<&str>, std::io::Error> {
    let keys = dirs
        .iter()
        .map(|info| info.key.as_str())
        .collect::<Vec<_>>();
    let dirs_for_multiselect = dirs
        .iter()
        .map(|info| {
            (
                info.key.as_str(),
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
fn ui_install_natvis_files(natvis_installs: Vec<&NatvisInfo>) -> Result<(), std::io::Error> {
    let overall_progress = cliclack::multi_progress("Copying natvis files...");

    let mut errors = Vec::new();
    for info in &natvis_installs {
        let spinner = overall_progress.add(cliclack::spinner());
        let file_string : String = if info.version.len() > 1 {
            format!(
                "{} Natvis files",
                info.version.iter().map(|v| format!("Qt{}", v)).collect::<Vec<_>>().join(", ")
            )
        } else {
            format!("Qt{} Natvis file", info.version.first().unwrap())
        };
        spinner.start(format!("  Copying {} for {}", file_string, style(info.name.as_str()).cyan()).as_str());
        if let Err(e) = copy_natvis_file(info) {
            errors.push(e);
        }
        spinner.stop(format!("{} {} copied for {}", style("✓").green().bold(), file_string, style(info.name.as_str()).cyan()).as_str());
    }
    overall_progress.stop();

    if !errors.is_empty() {
        overall_progress.stop();
        for e in errors {
            cliclack::log::error(format!("{}", e)).unwrap();
        }
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Some errors occurred during the installation",
        ));
    }

    Ok(())
}
