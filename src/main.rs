use console::style;
use std::path::PathBuf;
use std::{env, fs};

/// Struct to store natvis information for installation.
struct NatvisInfo {
    /// Key to identify the directory
    key: String,
    /// Name to display to the user
    name: String,
    /// Path to the directory
    path: PathBuf,
    /// Version(s) to install
    version: Vec<u8>,
}

fn main() {
    cliclack::intro(style("Natvis installation for Qt").on_green().black()).unwrap();

    cliclack::log::info(format!("This tool will install natvis files for Qt and MSVC\nExisting natvis files will be {}!", style("overwritten").yellow().bold())).unwrap();

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

/// Returns all the MS Visual Studio user directories.
fn get_msvc_dirs() -> Vec<NatvisInfo> {
    let mut dirs = Vec::new();

    let document_dir = dirs::document_dir().expect("Could not find document directory");

    let vs_versions = [
        ("2019", "Visual Studio 2019"),
        ("vs2022", "Visual Studio 2022"),
    ];
    for (key, name) in vs_versions.iter() {
        let vs_dir = document_dir.join(name);
        if vs_dir.exists() {
            dirs.push(NatvisInfo {
                key: key.to_string(),
                name: name.to_string(),
                path: vs_dir.join("Visualizers"),
                version: vec![5, 6],
            });
        }
    }

    dirs
}

/// Returns all the Qt installation.
fn get_qt_dirs(qt_root: PathBuf) -> Vec<NatvisInfo> {
    if !qt_root.exists() {
        return Vec::new();
    }

    let mut dirs = Vec::new();

    // Get all children of C:\Qt, and check the one looking like *.*.* (version)
    let qt_versions = qt_root
        .read_dir()
        .expect("Could not read Qt directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|entry| entry.contains('.'))
        .map(|entry| entry.to_string());

    for version in qt_versions {
        // If it has a child directory starting with msvc, add it to the list
        let qt_version_dir = qt_root.join(&version);

        let qt_version_major = version
            .split('.')
            .next()
            .expect("Could not extract Qt major version")
            .parse::<u8>()
            .expect("Could not parse Qt major version");

        let qt_msvc_dirs = qt_version_dir
            .read_dir()
            .expect("Could not read Qt version directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|entry| entry.starts_with("msvc"))
            .map(|entry| entry.to_string())
            .map(|entry| NatvisInfo {
                key: version.clone(),
                name: {
                    let mut qt_version = "Qt ".to_string();
                    qt_version.push_str(&version);
                    qt_version
                },
                path: qt_version_dir.join(entry).join("natvis"),
                version: vec![qt_version_major],
            });

        dirs.extend(qt_msvc_dirs);
    }

    dirs
}

/// Returns all the possible natvis info for installation.
fn get_natvis_installs(qt_root: PathBuf) -> Vec<NatvisInfo> {
    let mut dirs = Vec::new();

    dirs.extend(get_msvc_dirs());
    dirs.extend(get_qt_dirs(qt_root));

    dirs
}

/// UI: ask the user for the Qt installation root.
fn ui_get_qt_root() -> Result<PathBuf, std::io::Error> {
    let default_root = PathBuf::from("C:\\Qt");
    if default_root.exists() {
        cliclack::log::info(format!(
            "Default Qt installation root found in {}",
            style(default_root.to_str().unwrap()).cyan()
        ))?;
        return Ok(default_root);
    }

    cliclack::input("Qt installation root?")
        .placeholder("C:\\Qt")
        .validate(|input: &String| {
            if PathBuf::from(input).exists() {
                Ok(())
            } else {
                Err("Please enter a valid path")
            }
        })
        .interact()
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
    let spinner = cliclack::spinner();
    spinner.start("Copying natvis files...");

    let mut errors = Vec::new();
    for info in &natvis_installs {
        spinner.set_message(format!("Copying natvis files for {}", info.name).as_str());
        cliclack::log::info(format!("Copying natvis files for {}", info.name))?;
        for version in info.version.iter() {
            if let Err(e) = copy_natvis_file(*version, info.path.clone()) {
                errors.push(e);
            }
        }
    }

    match errors.is_empty() {
        true => spinner.stop(format!("Copied {} natvis files", natvis_installs.len())),
        _ => {
            spinner.clear();
            for e in errors {
                cliclack::log::error(format!("{}", e)).unwrap();
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Some errors occurred during the installation",
            ));
        }
    }

    Ok(())
}

/// Copy natvis file to the given path.
fn copy_natvis_file(version: u8, dst: PathBuf) -> Result<(), std::io::Error> {
    let natvis_file_name = format!("qt{}.natvis", version);

    // Create directory unconditionally
    if fs::create_dir_all(&dst).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Could not create {}", dst.display()),
        ));
    }

    // Get executable directory
    let src = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("natvis")
        .join(&natvis_file_name);

    if fs::copy(src, dst.join(&natvis_file_name)).is_err() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Could not copy {} to {}", natvis_file_name, dst.display()),
        ));
    }
    Ok(())
}
